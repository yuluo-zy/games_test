// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/tiny-glade/src/screenshot.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 36534a826f8528192dd7e41265bf71b11c5c88b1b42326b405ffaf7f151ef036
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: screenshot.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 31, 66, 104, 109, 130, 182, 253
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use anyhow; // namespace candidate
//   package identities: [{"name":"anyhow","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.86"}]
// use bevy_app; // namespace candidate
//   package identities: [{"name":"bevy_app","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use image; // namespace candidate
//   package identities: [{"name":"image","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.24.9"},{"name":"image","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.25.1"}]
// use lock_api; // namespace candidate
//   package identities: [{"name":"lock_api","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.9"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.27"}]
// use once_cell; // namespace candidate
//   package identities: [{"name":"once_cell","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.21.3"}]
// use parking_lot; // namespace candidate
//   package identities: [{"name":"parking_lot","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.11.2"},{"name":"parking_lot","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.1"}]
// use parking_lot_core; // namespace candidate
//   package identities: [{"name":"parking_lot_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.8.6"},{"name":"parking_lot_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.9.3"}]
// use paths; // namespace candidate
//   package identities: [{"name":"paths","source":"local_or_path","version":"0.1.0"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.1"}]
// use rhapsody; // namespace candidate
//   package identities: [{"name":"rhapsody","source":"local_or_path","version":"0.1.0"}]
// use std; // namespace candidate
//   package identities: []
// use steamworks; // namespace candidate
//   package identities: [{"name":"steamworks","source":"git+https://github.com/PounceLight/steamworks-rs.git?rev=d05eaa60db328156aeee172b0b157f3f69c70b30#d05eaa60db328156aeee172b0b157f3f69c70b30","version":"0.11.0"}]
// use tempfile; // namespace candidate
//   package identities: [{"name":"tempfile","source":"registry+https://github.com/rust-lang/crates.io-index","version":"3.20.0"}]
// use tiny_glade; // namespace candidate
//   package identities: [{"name":"tiny-glade","source":"local_or_path","version":"0.2.0"}]
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: tiny_glade::screenshot::impl$0::build
// 记录形式: compiled_procedure; instances=1
// 源码行: 50
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":130,"record_offset":5408,"rva":3256016,"va":5371965136}
// 编译实例 RVAs: 0x31aed0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1402ce490
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-313954","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371965136,"raw_hex":"e89e35fbff","size":5,"soft_fail":false,"target_va":5371651216,"va":5371965165,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403276f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t51187","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371965136,"raw_hex":"e8f3c70000","size":5,"soft_fail":false,"target_va":5372016368,"va":5371965176,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] alias_set_at_0x140326e40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t48946","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5371965136,"raw_hex":"e932bf0000","size":5,"soft_fail":false,"target_va":5372014144,"va":5371965193,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_app::app::App::add_systems
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_app","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_app-0.16.0/src/app.rs
//     evidence={"module":130,"record_offset":5532,"parent_record_offset":5408,"ancestor_record_offset":5408,"inlinee":9225,"source_line":301,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::screenshot::impl$6::eq
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 59
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3102
// 首条位置: {"module":130,"inlinee":12546}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::screenshot::impl$10::clone
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 73
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x310E
// 首条位置: {"module":130,"inlinee":12558}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":130,"record_offset":42824,"parent_record_offset":42760,"ancestor_record_offset":42760,"inlinee":6804,"source_line":2163,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$30::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":130,"record_offset":42908,"parent_record_offset":42760,"ancestor_record_offset":42760,"inlinee":8124,"source_line":1738,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":130,"record_offset":43492,"parent_record_offset":43468,"ancestor_record_offset":42760,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":130,"record_offset":43516,"parent_record_offset":43492,"ancestor_record_offset":42760,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":130,"record_offset":43468,"parent_record_offset":43292,"ancestor_record_offset":42760,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":130,"record_offset":43232,"parent_record_offset":43204,"ancestor_record_offset":42760,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":130,"record_offset":43292,"parent_record_offset":43260,"ancestor_record_offset":42760,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":130,"record_offset":43260,"parent_record_offset":43232,"ancestor_record_offset":42760,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":130,"record_offset":43328,"parent_record_offset":43292,"ancestor_record_offset":42760,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":130,"record_offset":43168,"parent_record_offset":43136,"ancestor_record_offset":42760,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":130,"record_offset":43136,"parent_record_offset":43068,"ancestor_record_offset":42760,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":130,"record_offset":43108,"parent_record_offset":43068,"ancestor_record_offset":42760,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":130,"record_offset":43204,"parent_record_offset":43168,"ancestor_record_offset":42760,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$11::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":130,"record_offset":43036,"parent_record_offset":43004,"ancestor_record_offset":42760,"inlinee":4623,"source_line":3286,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":130,"record_offset":43068,"parent_record_offset":43036,"ancestor_record_offset":42760,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":130,"record_offset":43352,"parent_record_offset":43328,"ancestor_record_offset":42760,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":130,"record_offset":43376,"parent_record_offset":43352,"ancestor_record_offset":42760,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":130,"record_offset":43592,"parent_record_offset":43568,"ancestor_record_offset":42760,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":130,"record_offset":43424,"parent_record_offset":43400,"ancestor_record_offset":42760,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":130,"record_offset":43400,"parent_record_offset":43376,"ancestor_record_offset":42760,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":130,"record_offset":43568,"parent_record_offset":43168,"ancestor_record_offset":42760,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":130,"record_offset":42848,"parent_record_offset":42824,"ancestor_record_offset":42760,"inlinee":6812,"source_line":2956,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":130,"record_offset":42872,"parent_record_offset":42848,"ancestor_record_offset":42760,"inlinee":4128,"source_line":3760,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$10::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":130,"record_offset":42940,"parent_record_offset":42908,"ancestor_record_offset":42760,"inlinee":8126,"source_line":642,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::impl$10::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":130,"record_offset":42972,"parent_record_offset":42940,"ancestor_record_offset":42760,"inlinee":8128,"source_line":13,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] std::sys_common::wtf8::impl$35::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys_common\wtf8.rs
//     evidence={"module":130,"record_offset":43004,"parent_record_offset":42972,"ancestor_record_offset":42760,"inlinee":8130,"source_line":132,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::screenshot::impl$1::drop
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 94
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3F04
// 首条位置: {"module":253,"inlinee":16132}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":253,"record_offset":1052452,"parent_record_offset":1052240,"ancestor_record_offset":1052240,"inlinee":5501,"source_line":600,"archive":"_pdb/B/modules/00253.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":253,"record_offset":1052272,"parent_record_offset":1052240,"ancestor_record_offset":1052240,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/00253.jsonl.gz"}
//   [pdb_inline_direct] std::thread::panicking
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\mod.rs
//     evidence={"module":253,"record_offset":1052300,"parent_record_offset":1052240,"ancestor_record_offset":1052240,"inlinee":5903,"source_line":814,"archive":"_pdb/B/modules/00253.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":253,"record_offset":1052384,"parent_record_offset":1052356,"ancestor_record_offset":1052240,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00253.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":253,"record_offset":1052408,"parent_record_offset":1052384,"ancestor_record_offset":1052240,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00253.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\panicking.rs
//     evidence={"module":253,"record_offset":1052356,"parent_record_offset":1052328,"ancestor_record_offset":1052240,"inlinee":5907,"source_line":469,"archive":"_pdb/B/modules/00253.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\panicking.rs
//     evidence={"module":253,"record_offset":1052328,"parent_record_offset":1052300,"ancestor_record_offset":1052240,"inlinee":5905,"source_line":621,"archive":"_pdb/B/modules/00253.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::screenshot::ScreenshotCapturedCallback::new
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 102
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2F03
// 首条位置: {"module":130,"inlinee":12035}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":130,"record_offset":12532,"parent_record_offset":12496,"ancestor_record_offset":12496,"inlinee":4280,"source_line":273,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":130,"record_offset":12624,"parent_record_offset":12600,"ancestor_record_offset":12496,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":130,"record_offset":12648,"parent_record_offset":12624,"ancestor_record_offset":12496,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":130,"record_offset":12568,"parent_record_offset":12532,"ancestor_record_offset":12496,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":130,"record_offset":12600,"parent_record_offset":12568,"ancestor_record_offset":12496,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":130,"record_offset":12676,"parent_record_offset":12648,"ancestor_record_offset":12496,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::screenshot::ScreenshotCapturedCallback::call
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 110
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x310B
// 首条位置: {"module":130,"inlinee":12555}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::boxed::impl$28::call_once
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":130,"record_offset":38576,"parent_record_offset":38412,"ancestor_record_offset":38412,"inlinee":7605,"source_line":1975,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::take
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":130,"record_offset":38456,"parent_record_offset":38412,"ancestor_record_offset":38412,"inlinee":4908,"source_line":1726,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":130,"record_offset":38540,"parent_record_offset":38412,"ancestor_record_offset":38412,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":130,"record_offset":38712,"parent_record_offset":38688,"ancestor_record_offset":38412,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":130,"record_offset":38688,"parent_record_offset":38604,"ancestor_record_offset":38412,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":130,"record_offset":38604,"parent_record_offset":38576,"ancestor_record_offset":38412,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":130,"record_offset":38632,"parent_record_offset":38604,"ancestor_record_offset":38412,"inlinee":5033,"source_line":219,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":130,"record_offset":38480,"parent_record_offset":38456,"ancestor_record_offset":38412,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":130,"record_offset":38656,"parent_record_offset":38632,"ancestor_record_offset":38412,"inlinee":5034,"source_line":393,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":130,"record_offset":38504,"parent_record_offset":38480,"ancestor_record_offset":38412,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::screenshot::gather_screenshot_requests
// 记录形式: compiled_procedure; instances=1
// 源码行: 166
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":130,"record_offset":5592,"rva":3256080,"va":5371965200}
// 编译实例 RVAs: 0x31af10
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14003bb80
//     owner=unresolved_namespace; count=1
//     target_aliases=532 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3014743","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371965200,"raw_hex":"e8a9ffd1ff","size":5,"soft_fail":false,"target_va":5368953728,"va":5371968466,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400412d0
//     owner=unresolved_namespace; count=3
//     target_aliases=39 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2995397","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371965200,"raw_hex":"e83b4bd2ff","size":5,"soft_fail":false,"target_va":5368976080,"va":5371971472,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140161c30
//     owner=unresolved_namespace; count=1
//     target_aliases=47 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1814280","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371965200,"raw_hex":"e8f850e4ff","size":5,"soft_fail":false,"target_va":5370158128,"va":5371972403,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402b54f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-417087","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371965200,"raw_hex":"e8c1a2f9ff","size":5,"soft_fail":false,"target_va":5371548912,"va":5371965994,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402ce390
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-320684","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371965200,"raw_hex":"e8541bfbff","size":5,"soft_fail":false,"target_va":5371650960,"va":5371971639,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402e5d80
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-223073","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371965200,"raw_hex":"e89f98fcff","size":5,"soft_fail":false,"target_va":5371747712,"va":5371970780,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048db70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1513706","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371965200,"raw_hex":"e8ea181700","size":5,"soft_fail":false,"target_va":5373483888,"va":5371970177,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405d0030
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2831538","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371965200,"raw_hex":"e8b2342b00","size":5,"soft_fail":false,"target_va":5374804016,"va":5371972473,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e0800
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2899540","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371965200,"raw_hex":"e8543e2c00","size":5,"soft_fail":false,"target_va":5374871552,"va":5371972007,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405f78e0
//     owner=unresolved_namespace; count=8
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2994164","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371965200,"raw_hex":"e8f4af2d00","size":5,"soft_fail":false,"target_va":5374965984,"va":5371971815,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405fbf20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3010052","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371965200,"raw_hex":"e804ee2d00","size":5,"soft_fail":false,"target_va":5374983968,"va":5371973911,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140606660
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3053758","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371965200,"raw_hex":"e8be982e00","size":5,"soft_fail":false,"target_va":5375026784,"va":5371973021,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=7
//     target_aliases=1 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3114825","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371965200,"raw_hex":"e849872f00","size":5,"soft_fail":false,"target_va":5375081488,"va":5371966658,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=10
//     target_aliases=1 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3115029","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371965200,"raw_hex":"e815882f00","size":5,"soft_fail":false,"target_va":5375081504,"va":5371966470,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14093fad0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6438465","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371965200,"raw_hex":"e8413e6200","size":5,"soft_fail":false,"target_va":5378407120,"va":5371968650,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b5c0a0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8654607","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371965200,"raw_hex":"e80f0f8400","size":5,"soft_fail":false,"target_va":5380620448,"va":5371965836,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b5c8c0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8654068","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371965200,"raw_hex":"e8f40c8400","size":5,"soft_fail":false,"target_va":5380622528,"va":5371968455,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b5c990
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8654167","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371965200,"raw_hex":"e8570d8400","size":5,"soft_fail":false,"target_va":5380622736,"va":5371968564,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140be24b0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9201872","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371965200,"raw_hex":"e8d0688c00","size":5,"soft_fail":false,"target_va":5381170352,"va":5371968475,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d61b00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10773813","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371965200,"raw_hex":"e83565a400","size":5,"soft_fail":false,"target_va":5382740736,"va":5371966918,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d76ed0
//     owner=unresolved_namespace; count=3
//     target_aliases=5 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10861391","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371965200,"raw_hex":"e84fbba500","size":5,"soft_fail":false,"target_va":5382827728,"va":5371966332,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t39880046","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371965200,"raw_hex":"e86e856002","size":5,"soft_fail":false,"target_va":5411847168,"va":5371967117,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t39876567","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371965200,"raw_hex":"e8d7776002","size":5,"soft_fail":false,"target_va":5411847184,"va":5371970612,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t39569914","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371965200,"raw_hex":"e8fac95b02","size":5,"soft_fail":false,"target_va":5411540992,"va":5371971073,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t10873757","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371965200,"raw_hex":"e89deba500","size":5,"soft_fail":false,"target_va":5382840208,"va":5371966446,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=10
//     evidence={"assembly":"\tcallq\t2388808","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371965200,"raw_hex":"e848732400","size":5,"soft_fail":false,"target_va":5374355568,"va":5371966755,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t39347386","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371965200,"raw_hex":"e8ba645802","size":5,"soft_fail":false,"target_va":5411318240,"va":5371970849,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t39569829","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371965200,"raw_hex":"e8a5c95b02","size":5,"soft_fail":false,"target_va":5411540963,"va":5371971129,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::input::action_set::DigitalActionState::just_pressed
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t6441264","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371965200,"raw_hex":"e830496200","size":5,"soft_fail":false,"target_va":5378407056,"va":5371965787,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::meta_commander::MetaCommander::consume
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t6303185","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371965200,"raw_hex":"e8d12d6000","size":5,"soft_fail":false,"target_va":5378273536,"va":5371970346,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::meta_commander::MetaCommander::peek
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t6303272","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371965200,"raw_hex":"e8282e6000","size":5,"soft_fail":false,"target_va":5378273472,"va":5371970195,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::gui::EscMenuState::is_closed
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t6439704","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371965200,"raw_hex":"e818436200","size":5,"soft_fail":false,"target_va":5378407392,"va":5371967683,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t39521553","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371965200,"raw_hex":"e8110d5b02","size":5,"soft_fail":false,"target_va":5411492368,"va":5371970810,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t39523913","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371965200,"raw_hex":"e849165b02","size":5,"soft_fail":false,"target_va":5411494800,"va":5371970882,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::lock_exclusive_slow
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t39514258","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371965200,"raw_hex":"e892f05a02","size":5,"soft_fail":false,"target_va":5411485168,"va":5371970905,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::unlock_exclusive_slow
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t39516619","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371965200,"raw_hex":"e8cbf95a02","size":5,"soft_fail":false,"target_va":5411487584,"va":5371970960,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::acquire_resource
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t9858217","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371965200,"raw_hex":"e8a96c9600","size":5,"soft_fail":false,"target_va":5381823600,"va":5371965378,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=9
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t9858544","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371965200,"raw_hex":"e8f06d9600","size":5,"soft_fail":false,"target_va":5381824016,"va":5371965467,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] paths::config
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"paths","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t8923102","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371965200,"raw_hex":"e8de278800","size":5,"soft_fail":false,"target_va":5380889392,"va":5371966285,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::_join
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t10777752","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371965200,"raw_hex":"e89874a400","size":5,"soft_fail":false,"target_va":5382744080,"va":5371966323,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::file::date_time_string_for_filename
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t9941757","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371965200,"raw_hex":"e8fdb29700","size":5,"soft_fail":false,"target_va":5381908112,"va":5371966350,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":130,"record_offset":9352,"parent_record_offset":5592,"ancestor_record_offset":5592,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":130,"record_offset":13912,"parent_record_offset":5592,"ancestor_record_offset":5592,"inlinee":4783,"source_line":386,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":130,"record_offset":10988,"parent_record_offset":5592,"ancestor_record_offset":5592,"inlinee":6804,"source_line":2163,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":130,"record_offset":10468,"parent_record_offset":5592,"ancestor_record_offset":5592,"inlinee":5345,"source_line":2205,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::impl$56::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":130,"record_offset":10712,"parent_record_offset":5592,"ancestor_record_offset":5592,"inlinee":7957,"source_line":3466,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":130,"record_offset":5824,"parent_record_offset":5592,"ancestor_record_offset":5592,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":130,"record_offset":11076,"parent_record_offset":5592,"ancestor_record_offset":5592,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":130,"record_offset":18752,"parent_record_offset":5592,"ancestor_record_offset":5592,"inlinee":4883,"source_line":3450,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$19::extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":130,"record_offset":18808,"parent_record_offset":5592,"ancestor_record_offset":5592,"inlinee":4917,"source_line":3492,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":130,"record_offset":8944,"parent_record_offset":5592,"ancestor_record_offset":5592,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::iterators::impl$0::last
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":130,"record_offset":13724,"parent_record_offset":5592,"ancestor_record_offset":5592,"inlinee":10715,"source_line":26,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::reader::EventReader::read
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/reader.rs
//     evidence={"module":130,"record_offset":7112,"parent_record_offset":5592,"ancestor_record_offset":5592,"inlinee":8034,"source_line":27,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::Ord::max
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":130,"record_offset":14732,"parent_record_offset":5592,"ancestor_record_offset":5592,"inlinee":6234,"source_line":998,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":130,"record_offset":12216,"parent_record_offset":5592,"ancestor_record_offset":5592,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":130,"record_offset":18204,"parent_record_offset":5592,"ancestor_record_offset":5592,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":130,"record_offset":7872,"parent_record_offset":5592,"ancestor_record_offset":5592,"inlinee":12536,"source_line":2758,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":130,"record_offset":9436,"parent_record_offset":5592,"ancestor_record_offset":5592,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":130,"record_offset":18596,"parent_record_offset":5592,"ancestor_record_offset":5592,"inlinee":10002,"source_line":213,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_direct] lock_api::rwlock::RwLock::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":130,"record_offset":10496,"parent_record_offset":5592,"ancestor_record_offset":5592,"inlinee":5840,"source_line":479,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":130,"record_offset":9184,"parent_record_offset":5592,"ancestor_record_offset":5592,"inlinee":5199,"source_line":2688,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$30::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":130,"record_offset":11444,"parent_record_offset":5592,"ancestor_record_offset":5592,"inlinee":8124,"source_line":1738,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$45::deref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":130,"record_offset":9212,"parent_record_offset":5592,"ancestor_record_offset":5592,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::screenshot::ScreenshotCapturedCallback::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/screenshot.rs
//     evidence={"module":130,"record_offset":12496,"parent_record_offset":5592,"ancestor_record_offset":5592,"inlinee":12035,"source_line":102,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::screenshot::add_captured_screenshots_to_steam_library
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/screenshot.rs
//     evidence={"module":130,"record_offset":8340,"parent_record_offset":5592,"ancestor_record_offset":5592,"inlinee":12542,"source_line":466,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::screenshot::consume_steam_screenshot_request
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/screenshot.rs
//     evidence={"module":130,"record_offset":5852,"parent_record_offset":5592,"ancestor_record_offset":5592,"inlinee":12532,"source_line":447,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":130,"record_offset":10856,"parent_record_offset":10832,"ancestor_record_offset":5592,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":130,"record_offset":10880,"parent_record_offset":10856,"ancestor_record_offset":5592,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":130,"record_offset":9612,"parent_record_offset":9588,"ancestor_record_offset":5592,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":130,"record_offset":12568,"parent_record_offset":12532,"ancestor_record_offset":5592,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":130,"record_offset":10832,"parent_record_offset":10808,"ancestor_record_offset":5592,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":130,"record_offset":9588,"parent_record_offset":9532,"ancestor_record_offset":5592,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":130,"record_offset":12532,"parent_record_offset":12496,"ancestor_record_offset":5592,"inlinee":4280,"source_line":273,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":130,"record_offset":10748,"parent_record_offset":10712,"ancestor_record_offset":5592,"inlinee":7958,"source_line":293,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$1::new_uninit_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":130,"record_offset":10776,"parent_record_offset":10748,"ancestor_record_offset":5592,"inlinee":6163,"source_line":480,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$1::try_new_uninit_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":130,"record_offset":10808,"parent_record_offset":10776,"ancestor_record_offset":5592,"inlinee":6164,"source_line":513,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$4::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":130,"record_offset":10928,"parent_record_offset":10712,"ancestor_record_offset":5592,"inlinee":7960,"source_line":972,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":130,"record_offset":9400,"parent_record_offset":9376,"ancestor_record_offset":5592,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":130,"record_offset":11144,"parent_record_offset":11120,"ancestor_record_offset":5592,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":130,"record_offset":18984,"parent_record_offset":18956,"ancestor_record_offset":5592,"inlinee":4962,"source_line":326,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":130,"record_offset":11792,"parent_record_offset":11764,"ancestor_record_offset":5592,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":130,"record_offset":19068,"parent_record_offset":19044,"ancestor_record_offset":5592,"inlinee":4737,"source_line":512,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":130,"record_offset":9560,"parent_record_offset":9532,"ancestor_record_offset":5592,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":130,"record_offset":9532,"parent_record_offset":9508,"ancestor_record_offset":5592,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":130,"record_offset":19044,"parent_record_offset":19012,"ancestor_record_offset":5592,"inlinee":4964,"source_line":622,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":130,"record_offset":11192,"parent_record_offset":11168,"ancestor_record_offset":5592,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":130,"record_offset":11168,"parent_record_offset":11144,"ancestor_record_offset":5592,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":130,"record_offset":19012,"parent_record_offset":18984,"ancestor_record_offset":5592,"inlinee":4963,"source_line":541,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":130,"record_offset":11852,"parent_record_offset":11820,"ancestor_record_offset":5592,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":130,"record_offset":11820,"parent_record_offset":11792,"ancestor_record_offset":5592,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":130,"record_offset":9508,"parent_record_offset":9484,"ancestor_record_offset":5592,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":130,"record_offset":11888,"parent_record_offset":11852,"ancestor_record_offset":5592,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":130,"record_offset":11724,"parent_record_offset":11688,"ancestor_record_offset":5592,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":130,"record_offset":11688,"parent_record_offset":11616,"ancestor_record_offset":5592,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":130,"record_offset":15140,"parent_record_offset":15104,"ancestor_record_offset":5592,"inlinee":4776,"source_line":1888,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":130,"record_offset":5924,"parent_record_offset":5852,"ancestor_record_offset":5592,"inlinee":5345,"source_line":2205,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$37::drop
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":130,"record_offset":6728,"parent_record_offset":6704,"ancestor_record_offset":5592,"inlinee":4774,"source_line":2550,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":130,"record_offset":11120,"parent_record_offset":11076,"ancestor_record_offset":5592,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":130,"record_offset":7500,"parent_record_offset":7476,"ancestor_record_offset":5592,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":130,"record_offset":18908,"parent_record_offset":18872,"ancestor_record_offset":5592,"inlinee":4678,"source_line":3552,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":130,"record_offset":18956,"parent_record_offset":18908,"ancestor_record_offset":5592,"inlinee":4961,"source_line":1271,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":130,"record_offset":6648,"parent_record_offset":6560,"ancestor_record_offset":5592,"inlinee":5297,"source_line":1895,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::swap_remove
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":130,"record_offset":6560,"parent_record_offset":5852,"ancestor_record_offset":5592,"inlinee":5532,"source_line":1927,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":130,"record_offset":11764,"parent_record_offset":11724,"ancestor_record_offset":5592,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$11::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":130,"record_offset":11580,"parent_record_offset":11548,"ancestor_record_offset":5592,"inlinee":4623,"source_line":3286,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":130,"record_offset":7476,"parent_record_offset":7216,"ancestor_record_offset":5592,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":130,"record_offset":19288,"parent_record_offset":18908,"ancestor_record_offset":5592,"inlinee":6531,"source_line":13,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":130,"record_offset":18872,"parent_record_offset":18808,"ancestor_record_offset":5592,"inlinee":4677,"source_line":28,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":130,"record_offset":8972,"parent_record_offset":8944,"ancestor_record_offset":5592,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::event_cursor::EventCursor::read
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/event_cursor.rs
//     evidence={"module":130,"record_offset":7152,"parent_record_offset":7112,"ancestor_record_offset":5592,"inlinee":8036,"source_line":78,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::event_cursor::EventCursor::read_with_id
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/event_cursor.rs
//     evidence={"module":130,"record_offset":7184,"parent_record_offset":7152,"ancestor_record_offset":5592,"inlinee":8037,"source_line":88,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::iterators::EventIteratorWithId::new
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":130,"record_offset":7216,"parent_record_offset":7184,"ancestor_record_offset":5592,"inlinee":8039,"source_line":54,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::iterators::impl$0::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":130,"record_offset":7928,"parent_record_offset":7896,"ancestor_record_offset":5592,"inlinee":12537,"source_line":14,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::iterators::impl$3::last
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":130,"record_offset":13756,"parent_record_offset":13724,"ancestor_record_offset":5592,"inlinee":10716,"source_line":112,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::iterators::impl$3::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":130,"record_offset":7956,"parent_record_offset":7928,"ancestor_record_offset":5592,"inlinee":12538,"source_line":86,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$19::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/system_param.rs
//     evidence={"module":130,"record_offset":16816,"parent_record_offset":16780,"ancestor_record_offset":5592,"inlinee":10713,"source_line":1234,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":130,"record_offset":11912,"parent_record_offset":11888,"ancestor_record_offset":5592,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":130,"record_offset":11936,"parent_record_offset":11912,"ancestor_record_offset":5592,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":130,"record_offset":8660,"parent_record_offset":8636,"ancestor_record_offset":5592,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":130,"record_offset":6616,"parent_record_offset":6560,"ancestor_record_offset":5592,"inlinee":5303,"source_line":4520,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":130,"record_offset":12152,"parent_record_offset":12128,"ancestor_record_offset":5592,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":130,"record_offset":11984,"parent_record_offset":11960,"ancestor_record_offset":5592,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::and_then_or_clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":130,"record_offset":8012,"parent_record_offset":7988,"ancestor_record_offset":5592,"inlinee":5541,"source_line":332,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::last
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":130,"record_offset":13788,"parent_record_offset":13756,"ancestor_record_offset":5592,"inlinee":10717,"source_line":176,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":130,"record_offset":7988,"parent_record_offset":7956,"ancestor_record_offset":5592,"inlinee":4570,"source_line":83,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":130,"record_offset":8092,"parent_record_offset":8068,"ancestor_record_offset":5592,"inlinee":4573,"source_line":84,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\collect.rs
//     evidence={"module":130,"record_offset":18844,"parent_record_offset":18808,"ancestor_record_offset":5592,"inlinee":4756,"source_line":322,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::any::check::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":130,"record_offset":8276,"parent_record_offset":7896,"ancestor_record_offset":5592,"inlinee":12540,"source_line":2765,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":130,"record_offset":19232,"parent_record_offset":18908,"ancestor_record_offset":5592,"inlinee":4679,"source_line":790,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":130,"record_offset":19256,"parent_record_offset":19232,"ancestor_record_offset":5592,"inlinee":6052,"source_line":796,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":130,"record_offset":7896,"parent_record_offset":7872,"ancestor_record_offset":5592,"inlinee":6092,"source_line":2362,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\maybe_uninit.rs
//     evidence={"module":130,"record_offset":10952,"parent_record_offset":10928,"ancestor_record_offset":5592,"inlinee":7961,"source_line":510,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":130,"record_offset":11960,"parent_record_offset":11936,"ancestor_record_offset":5592,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":130,"record_offset":7428,"parent_record_offset":7396,"ancestor_record_offset":5592,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":130,"record_offset":7276,"parent_record_offset":7216,"ancestor_record_offset":5592,"inlinee":8040,"source_line":1859,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":130,"record_offset":13816,"parent_record_offset":13788,"ancestor_record_offset":5592,"inlinee":4518,"source_line":1447,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":130,"record_offset":8036,"parent_record_offset":8012,"ancestor_record_offset":5592,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":130,"record_offset":9376,"parent_record_offset":9352,"ancestor_record_offset":5592,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":130,"record_offset":17736,"parent_record_offset":17648,"ancestor_record_offset":5592,"inlinee":10718,"source_line":1524,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":130,"record_offset":8068,"parent_record_offset":7988,"ancestor_record_offset":5592,"inlinee":4571,"source_line":1546,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$39::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":130,"record_offset":13876,"parent_record_offset":13756,"ancestor_record_offset":5592,"inlinee":4927,"source_line":2486,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":130,"record_offset":12128,"parent_record_offset":11724,"ancestor_record_offset":5592,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=54
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":130,"record_offset":6312,"parent_record_offset":5852,"ancestor_record_offset":5592,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":130,"record_offset":7764,"parent_record_offset":7740,"ancestor_record_offset":5592,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":130,"record_offset":8216,"parent_record_offset":8192,"ancestor_record_offset":5592,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":130,"record_offset":15172,"parent_record_offset":15140,"ancestor_record_offset":5592,"inlinee":4777,"source_line":425,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":130,"record_offset":6244,"parent_record_offset":6204,"ancestor_record_offset":5592,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":130,"record_offset":6588,"parent_record_offset":6560,"ancestor_record_offset":5592,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":130,"record_offset":12676,"parent_record_offset":12648,"ancestor_record_offset":5592,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":130,"record_offset":11232,"parent_record_offset":11076,"ancestor_record_offset":5592,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":130,"record_offset":7332,"parent_record_offset":7216,"ancestor_record_offset":5592,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":130,"record_offset":7716,"parent_record_offset":7216,"ancestor_record_offset":5592,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":130,"record_offset":7396,"parent_record_offset":7364,"ancestor_record_offset":5592,"inlinee":8042,"source_line":364,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::get
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":130,"record_offset":7364,"parent_record_offset":7332,"ancestor_record_offset":5592,"inlinee":8041,"source_line":537,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":130,"record_offset":7740,"parent_record_offset":7716,"ancestor_record_offset":5592,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":130,"record_offset":8192,"parent_record_offset":8168,"ancestor_record_offset":5592,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":130,"record_offset":8144,"parent_record_offset":8092,"ancestor_record_offset":5592,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":130,"record_offset":8168,"parent_record_offset":8144,"ancestor_record_offset":5592,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$190::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":130,"record_offset":6204,"parent_record_offset":6168,"ancestor_record_offset":5592,"inlinee":5335,"source_line":156,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$190::position
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":130,"record_offset":6168,"parent_record_offset":5852,"ancestor_record_offset":5592,"inlinee":12533,"source_line":331,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":130,"record_offset":20980,"parent_record_offset":20916,"ancestor_record_offset":5592,"inlinee":4945,"source_line":2871,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":130,"record_offset":18660,"parent_record_offset":18624,"ancestor_record_offset":5592,"inlinee":10004,"source_line":2920,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":130,"record_offset":6492,"parent_record_offset":6368,"ancestor_record_offset":5592,"inlinee":5818,"source_line":2871,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange_weak
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":130,"record_offset":6016,"parent_record_offset":5980,"ancestor_record_offset":5592,"inlinee":5824,"source_line":2920,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":130,"record_offset":11016,"parent_record_offset":10988,"ancestor_record_offset":5592,"inlinee":6812,"source_line":2956,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":130,"record_offset":6764,"parent_record_offset":6728,"ancestor_record_offset":5592,"inlinee":5814,"source_line":2986,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":130,"record_offset":11040,"parent_record_offset":11016,"ancestor_record_offset":5592,"inlinee":4128,"source_line":3760,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":130,"record_offset":6516,"parent_record_offset":6492,"ancestor_record_offset":5592,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":130,"record_offset":6040,"parent_record_offset":6016,"ancestor_record_offset":5592,"inlinee":5825,"source_line":3828,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":130,"record_offset":6788,"parent_record_offset":6764,"ancestor_record_offset":5592,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":130,"record_offset":6820,"parent_record_offset":6728,"ancestor_record_offset":5592,"inlinee":4779,"source_line":4075,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":130,"record_offset":20888,"parent_record_offset":20860,"ancestor_record_offset":5592,"inlinee":9996,"source_line":653,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::RwLock::write
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":130,"record_offset":5952,"parent_record_offset":5852,"ancestor_record_offset":5592,"inlinee":5840,"source_line":479,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::impl$25::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":130,"record_offset":8636,"parent_record_offset":8340,"ancestor_record_offset":5592,"inlinee":12544,"source_line":1660,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::impl$26::drop
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":130,"record_offset":6340,"parent_record_offset":6312,"ancestor_record_offset":5592,"inlinee":11657,"source_line":1667,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":130,"record_offset":18624,"parent_record_offset":18596,"ancestor_record_offset":5592,"inlinee":10003,"source_line":66,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":130,"record_offset":20916,"parent_record_offset":20888,"ancestor_record_offset":5592,"inlinee":4941,"source_line":100,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::deadlock_acquire
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":130,"record_offset":6072,"parent_record_offset":5980,"ancestor_record_offset":5592,"inlinee":5826,"source_line":1139,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::deadlock_release
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":130,"record_offset":6404,"parent_record_offset":6368,"ancestor_record_offset":5592,"inlinee":5812,"source_line":1145,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::lock_exclusive
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":130,"record_offset":5980,"parent_record_offset":5952,"ancestor_record_offset":5592,"inlinee":5841,"source_line":67,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::unlock_exclusive
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":130,"record_offset":6368,"parent_record_offset":6340,"ancestor_record_offset":5592,"inlinee":5817,"source_line":94,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":130,"record_offset":6100,"parent_record_offset":6072,"ancestor_record_offset":5592,"inlinee":5827,"source_line":1104,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":130,"record_offset":6432,"parent_record_offset":6404,"ancestor_record_offset":5592,"inlinee":4943,"source_line":1120,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::OsStr::to_os_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":130,"record_offset":12264,"parent_record_offset":12240,"ancestor_record_offset":5592,"inlinee":5171,"source_line":926,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$10::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":130,"record_offset":11480,"parent_record_offset":11444,"ancestor_record_offset":5592,"inlinee":8126,"source_line":642,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$59::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":130,"record_offset":9260,"parent_record_offset":9236,"ancestor_record_offset":5592,"inlinee":5134,"source_line":1639,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":130,"record_offset":9308,"parent_record_offset":9284,"ancestor_record_offset":5592,"inlinee":5138,"source_line":599,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":130,"record_offset":9284,"parent_record_offset":9260,"ancestor_record_offset":5592,"inlinee":5136,"source_line":617,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":130,"record_offset":9236,"parent_record_offset":9212,"ancestor_record_offset":5592,"inlinee":5132,"source_line":2182,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$37::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":130,"record_offset":12240,"parent_record_offset":12216,"ancestor_record_offset":5592,"inlinee":5169,"source_line":1825,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::impl$10::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":130,"record_offset":11516,"parent_record_offset":11480,"ancestor_record_offset":5592,"inlinee":8128,"source_line":13,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] std::sys_common::wtf8::impl$35::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys_common\wtf8.rs
//     evidence={"module":130,"record_offset":11548,"parent_record_offset":11516,"ancestor_record_offset":5592,"inlinee":8130,"source_line":132,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::screenshot::consume_steam_screenshot_request::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/screenshot.rs
//     evidence={"module":130,"record_offset":6280,"parent_record_offset":6168,"ancestor_record_offset":5592,"inlinee":12535,"source_line":456,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::screenshot::gather_screenshot_requests::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/screenshot.rs
//     evidence={"module":130,"record_offset":8300,"parent_record_offset":8276,"ancestor_record_offset":5592,"inlinee":12541,"source_line":197,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::screenshot::gather_screenshot_requests::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 197
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x30FD
// 首条位置: {"module":130,"inlinee":12541}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::screenshot::gather_screenshot_requests::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 262
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2EFD
// 首条位置: {"module":109,"inlinee":12029}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":109,"record_offset":84548,"parent_record_offset":84492,"ancestor_record_offset":84492,"inlinee":5345,"source_line":2205,"archive":"_pdb/B/modules/00109.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":109,"record_offset":85076,"parent_record_offset":84492,"ancestor_record_offset":84492,"inlinee":8786,"source_line":1784,"archive":"_pdb/B/modules/00109.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":109,"record_offset":85192,"parent_record_offset":84492,"ancestor_record_offset":84492,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00109.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":109,"record_offset":84732,"parent_record_offset":84492,"ancestor_record_offset":84492,"inlinee":5236,"source_line":771,"archive":"_pdb/B/modules/00109.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":109,"record_offset":84576,"parent_record_offset":84492,"ancestor_record_offset":84492,"inlinee":10002,"source_line":213,"archive":"_pdb/B/modules/00109.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":109,"record_offset":85004,"parent_record_offset":84980,"ancestor_record_offset":84492,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00109.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":109,"record_offset":84980,"parent_record_offset":84924,"ancestor_record_offset":84492,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00109.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":109,"record_offset":85584,"parent_record_offset":85556,"ancestor_record_offset":84492,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/00109.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":109,"record_offset":84952,"parent_record_offset":84924,"ancestor_record_offset":84492,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00109.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":109,"record_offset":84924,"parent_record_offset":84900,"ancestor_record_offset":84492,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00109.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":109,"record_offset":84900,"parent_record_offset":84880,"ancestor_record_offset":84492,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00109.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":109,"record_offset":86076,"parent_record_offset":86040,"ancestor_record_offset":84492,"inlinee":4776,"source_line":1888,"archive":"_pdb/B/modules/00109.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$37::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":109,"record_offset":86040,"parent_record_offset":86016,"ancestor_record_offset":84492,"inlinee":4774,"source_line":2550,"archive":"_pdb/B/modules/00109.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":109,"record_offset":85612,"parent_record_offset":85584,"ancestor_record_offset":84492,"inlinee":5033,"source_line":219,"archive":"_pdb/B/modules/00109.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":109,"record_offset":85100,"parent_record_offset":85076,"ancestor_record_offset":84492,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00109.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":109,"record_offset":85636,"parent_record_offset":85612,"ancestor_record_offset":84492,"inlinee":5034,"source_line":393,"archive":"_pdb/B/modules/00109.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=21
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":109,"record_offset":84760,"parent_record_offset":84732,"ancestor_record_offset":84492,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00109.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":109,"record_offset":86108,"parent_record_offset":86076,"ancestor_record_offset":84492,"inlinee":4777,"source_line":425,"archive":"_pdb/B/modules/00109.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":109,"record_offset":85128,"parent_record_offset":85100,"ancestor_record_offset":84492,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00109.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":109,"record_offset":85156,"parent_record_offset":85100,"ancestor_record_offset":84492,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00109.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":109,"record_offset":85948,"parent_record_offset":85884,"ancestor_record_offset":84492,"inlinee":4945,"source_line":2871,"archive":"_pdb/B/modules/00109.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":109,"record_offset":84640,"parent_record_offset":84604,"ancestor_record_offset":84492,"inlinee":10004,"source_line":2920,"archive":"_pdb/B/modules/00109.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":109,"record_offset":85972,"parent_record_offset":85948,"ancestor_record_offset":84492,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/00109.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":109,"record_offset":84664,"parent_record_offset":84640,"ancestor_record_offset":84492,"inlinee":5825,"source_line":3828,"archive":"_pdb/B/modules/00109.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":109,"record_offset":86136,"parent_record_offset":86076,"ancestor_record_offset":84492,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/00109.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":109,"record_offset":86168,"parent_record_offset":86040,"ancestor_record_offset":84492,"inlinee":4779,"source_line":4075,"archive":"_pdb/B/modules/00109.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":109,"record_offset":85856,"parent_record_offset":85828,"ancestor_record_offset":84492,"inlinee":9996,"source_line":653,"archive":"_pdb/B/modules/00109.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":109,"record_offset":84604,"parent_record_offset":84576,"ancestor_record_offset":84492,"inlinee":10003,"source_line":66,"archive":"_pdb/B/modules/00109.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":109,"record_offset":85884,"parent_record_offset":85856,"ancestor_record_offset":84492,"inlinee":4941,"source_line":100,"archive":"_pdb/B/modules/00109.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":109,"record_offset":84696,"parent_record_offset":84604,"ancestor_record_offset":84492,"inlinee":5827,"source_line":1104,"archive":"_pdb/B/modules/00109.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":109,"record_offset":85920,"parent_record_offset":85884,"ancestor_record_offset":84492,"inlinee":4943,"source_line":1120,"archive":"_pdb/B/modules/00109.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::screenshot::consume_steam_screenshot_request
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 447
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x30F4
// 首条位置: {"module":130,"inlinee":12532}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":130,"record_offset":5924,"parent_record_offset":5852,"ancestor_record_offset":5852,"inlinee":5345,"source_line":2205,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::swap_remove
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":130,"record_offset":6560,"parent_record_offset":5852,"ancestor_record_offset":5852,"inlinee":5532,"source_line":1927,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":130,"record_offset":6312,"parent_record_offset":5852,"ancestor_record_offset":5852,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$190::position
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":130,"record_offset":6168,"parent_record_offset":5852,"ancestor_record_offset":5852,"inlinee":12533,"source_line":331,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_direct] lock_api::rwlock::RwLock::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":130,"record_offset":5952,"parent_record_offset":5852,"ancestor_record_offset":5852,"inlinee":5840,"source_line":479,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$37::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":130,"record_offset":6728,"parent_record_offset":6704,"ancestor_record_offset":5852,"inlinee":4774,"source_line":2550,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":130,"record_offset":6648,"parent_record_offset":6560,"ancestor_record_offset":5852,"inlinee":5297,"source_line":1895,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":130,"record_offset":6616,"parent_record_offset":6560,"ancestor_record_offset":5852,"inlinee":5303,"source_line":4520,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":130,"record_offset":6704,"parent_record_offset":6680,"ancestor_record_offset":5852,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":130,"record_offset":6244,"parent_record_offset":6204,"ancestor_record_offset":5852,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":130,"record_offset":6588,"parent_record_offset":6560,"ancestor_record_offset":5852,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$190::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":130,"record_offset":6204,"parent_record_offset":6168,"ancestor_record_offset":5852,"inlinee":5335,"source_line":156,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":130,"record_offset":6492,"parent_record_offset":6368,"ancestor_record_offset":5852,"inlinee":5818,"source_line":2871,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":130,"record_offset":6016,"parent_record_offset":5980,"ancestor_record_offset":5852,"inlinee":5824,"source_line":2920,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":130,"record_offset":6764,"parent_record_offset":6728,"ancestor_record_offset":5852,"inlinee":5814,"source_line":2986,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":130,"record_offset":6516,"parent_record_offset":6492,"ancestor_record_offset":5852,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":130,"record_offset":6040,"parent_record_offset":6016,"ancestor_record_offset":5852,"inlinee":5825,"source_line":3828,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":130,"record_offset":6788,"parent_record_offset":6764,"ancestor_record_offset":5852,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":130,"record_offset":6820,"parent_record_offset":6728,"ancestor_record_offset":5852,"inlinee":4779,"source_line":4075,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::impl$26::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":130,"record_offset":6340,"parent_record_offset":6312,"ancestor_record_offset":5852,"inlinee":11657,"source_line":1667,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::deadlock_acquire
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":130,"record_offset":6072,"parent_record_offset":5980,"ancestor_record_offset":5852,"inlinee":5826,"source_line":1139,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::deadlock_release
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":130,"record_offset":6404,"parent_record_offset":6368,"ancestor_record_offset":5852,"inlinee":5812,"source_line":1145,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::lock_exclusive
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":130,"record_offset":5980,"parent_record_offset":5952,"ancestor_record_offset":5852,"inlinee":5841,"source_line":67,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::unlock_exclusive
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":130,"record_offset":6368,"parent_record_offset":6340,"ancestor_record_offset":5852,"inlinee":5817,"source_line":94,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":130,"record_offset":6100,"parent_record_offset":6072,"ancestor_record_offset":5852,"inlinee":5827,"source_line":1104,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":130,"record_offset":6432,"parent_record_offset":6404,"ancestor_record_offset":5852,"inlinee":4943,"source_line":1120,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::screenshot::consume_steam_screenshot_request::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/screenshot.rs
//     evidence={"module":130,"record_offset":6280,"parent_record_offset":6168,"ancestor_record_offset":5852,"inlinee":12535,"source_line":456,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::screenshot::consume_steam_screenshot_request::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 456
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x30F7
// 首条位置: {"module":130,"inlinee":12535}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::screenshot::add_captured_screenshots_to_steam_library
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 466
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x30FE
// 首条位置: {"module":130,"inlinee":12542}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":130,"record_offset":8392,"parent_record_offset":8340,"ancestor_record_offset":8340,"inlinee":5345,"source_line":2205,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":130,"record_offset":8692,"parent_record_offset":8340,"ancestor_record_offset":8340,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_direct] lock_api::rwlock::RwLock::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":130,"record_offset":8420,"parent_record_offset":8340,"ancestor_record_offset":8340,"inlinee":5840,"source_line":479,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_direct] lock_api::rwlock::impl$25::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":130,"record_offset":8636,"parent_record_offset":8340,"ancestor_record_offset":8340,"inlinee":12544,"source_line":1660,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":130,"record_offset":8660,"parent_record_offset":8636,"ancestor_record_offset":8340,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":130,"record_offset":8872,"parent_record_offset":8748,"ancestor_record_offset":8340,"inlinee":5818,"source_line":2871,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":130,"record_offset":8484,"parent_record_offset":8448,"ancestor_record_offset":8340,"inlinee":5824,"source_line":2920,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":130,"record_offset":8896,"parent_record_offset":8872,"ancestor_record_offset":8340,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":130,"record_offset":8508,"parent_record_offset":8484,"ancestor_record_offset":8340,"inlinee":5825,"source_line":3828,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::impl$26::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":130,"record_offset":8720,"parent_record_offset":8692,"ancestor_record_offset":8340,"inlinee":11657,"source_line":1667,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::deadlock_acquire
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":130,"record_offset":8540,"parent_record_offset":8448,"ancestor_record_offset":8340,"inlinee":5826,"source_line":1139,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::deadlock_release
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":130,"record_offset":8784,"parent_record_offset":8748,"ancestor_record_offset":8340,"inlinee":5812,"source_line":1145,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::lock_exclusive
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":130,"record_offset":8448,"parent_record_offset":8420,"ancestor_record_offset":8340,"inlinee":5841,"source_line":67,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::unlock_exclusive
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":130,"record_offset":8748,"parent_record_offset":8720,"ancestor_record_offset":8340,"inlinee":5817,"source_line":94,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":130,"record_offset":8568,"parent_record_offset":8540,"ancestor_record_offset":8340,"inlinee":5827,"source_line":1104,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":130,"record_offset":8812,"parent_record_offset":8784,"ancestor_record_offset":8340,"inlinee":4943,"source_line":1120,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::screenshot::add_captured_screenshots_to_steam_library::closure$0
// 记录形式: compiled_procedure; instances=1
// 源码行: 474
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":104,"record_offset":188380,"rva":2888656,"va":5371597776}
// 编译实例 RVAs: 0x2c13d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t11046326","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371597776,"raw_hex":"e8b68da800","size":5,"soft_fail":false,"target_va":5382644528,"va":5371598197,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14002e900
//     owner=unresolved_namespace; count=1
//     target_aliases=130 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2699451","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371597776,"raw_hex":"e845cfd6ff","size":5,"soft_fail":false,"target_va":5368899840,"va":5371599286,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140161c30
//     owner=unresolved_namespace; count=1
//     target_aliases=47 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1441300","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371597776,"raw_hex":"e8ec01eaff","size":5,"soft_fail":false,"target_va":5370158128,"va":5371599423,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=5
//     target_aliases=1 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3483225","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371597776,"raw_hex":"e859263500","size":5,"soft_fail":false,"target_va":5375081504,"va":5371598274,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067dd80
//     owner=unresolved_namespace; count=4
//     target_aliases=69 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3917972","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371597776,"raw_hex":"e894c83b00","size":5,"soft_fail":false,"target_va":5375516032,"va":5371598055,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d76ed0
//     owner=unresolved_namespace; count=2
//     target_aliases=5 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11229402","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371597776,"raw_hex":"e8da58ab00","size":5,"soft_fail":false,"target_va":5382827728,"va":5371598321,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1429234e0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t40246889","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371597776,"raw_hex":"e8691e6602","size":5,"soft_fail":false,"target_va":5411845344,"va":5371598450,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923560
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t40247102","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371597776,"raw_hex":"e83e1f6602","size":5,"soft_fail":false,"target_va":5411845472,"va":5371598365,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t9190553","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371597776,"raw_hex":"e8993c8c00","size":5,"soft_fail":false,"target_va":5380789072,"va":5371598514,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t39942671","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371597776,"raw_hex":"e80f7a6102","size":5,"soft_fail":false,"target_va":5411541776,"va":5371599100,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t39893294","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371597776,"raw_hex":"e82eb96002","size":5,"soft_fail":false,"target_va":5411492368,"va":5371599069,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t39895709","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371597776,"raw_hex":"e89dc26002","size":5,"soft_fail":false,"target_va":5411494800,"va":5371599086,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::acquire_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t10225752","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371597776,"raw_hex":"e858089c00","size":5,"soft_fail":false,"target_va":5381823600,"va":5371597843,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t10226106","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371597776,"raw_hex":"e8ba099c00","size":5,"soft_fail":false,"target_va":5381824016,"va":5371597905,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::is_file
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t11146994","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371597776,"raw_hex":"e8f216aa00","size":5,"soft_fail":false,"target_va":5382745328,"va":5371598329,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] steamworks::screenshots::path_to_absolute_cstring
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"steamworks","version":"0.11.0","source":"git+https://github.com/PounceLight/steamworks-rs.git?rev=d05eaa60db328156aeee172b0b157f3f69c70b30#d05eaa60db328156aeee172b0b157f3f69c70b30"}]
//     evidence={"assembly":"\tcallq\t9189794","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371597776,"raw_hex":"e8a2398c00","size":5,"soft_fail":false,"target_va":5380788208,"va":5371598409,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*%rax","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5371597776,"raw_hex":"ffd0","size":2,"soft_fail":false,"va":5371598245,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":104,"record_offset":188584,"parent_record_offset":188380,"ancestor_record_offset":188380,"inlinee":5345,"source_line":2205,"archive":"_pdb/B/modules/00104.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":104,"record_offset":189156,"parent_record_offset":188380,"ancestor_record_offset":188380,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/00104.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":104,"record_offset":189824,"parent_record_offset":188380,"ancestor_record_offset":188380,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00104.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::take
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":104,"record_offset":188768,"parent_record_offset":188380,"ancestor_record_offset":188380,"inlinee":4908,"source_line":1726,"archive":"_pdb/B/modules/00104.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":104,"record_offset":189852,"parent_record_offset":188380,"ancestor_record_offset":188380,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00104.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":104,"record_offset":188884,"parent_record_offset":188380,"ancestor_record_offset":188380,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00104.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":104,"record_offset":188612,"parent_record_offset":188380,"ancestor_record_offset":188380,"inlinee":10002,"source_line":213,"archive":"_pdb/B/modules/00104.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":104,"record_offset":189188,"parent_record_offset":188380,"ancestor_record_offset":188380,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/00104.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":104,"record_offset":189072,"parent_record_offset":188380,"ancestor_record_offset":188380,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/00104.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$45::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":104,"record_offset":189684,"parent_record_offset":188380,"ancestor_record_offset":188380,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/00104.jsonl.gz"}
//   [pdb_inline_direct] steamworks::Client::screenshots
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"steamworks","version":"0.11.0","source":"git+https://github.com/PounceLight/steamworks-rs.git?rev=d05eaa60db328156aeee172b0b157f3f69c70b30#d05eaa60db328156aeee172b0b157f3f69c70b30"}]
//     definition_source=/home/h3/.cargo/git/checkouts/steamworks-rs-f13bba748a20c02a/d05eaa6/src/lib.rs
//     evidence={"module":104,"record_offset":189888,"parent_record_offset":188380,"ancestor_record_offset":188380,"inlinee":8452,"source_line":408,"archive":"_pdb/B/modules/00104.jsonl.gz"}
//   [pdb_inline_direct] steamworks::screenshots::Screenshots::add_screenshot_to_library
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"steamworks","version":"0.11.0","source":"git+https://github.com/PounceLight/steamworks-rs.git?rev=d05eaa60db328156aeee172b0b157f3f69c70b30#d05eaa60db328156aeee172b0b157f3f69c70b30"}]
//     definition_source=/home/h3/.cargo/git/checkouts/steamworks-rs-f13bba748a20c02a/d05eaa6/src/screenshots.rs
//     evidence={"module":104,"record_offset":190184,"parent_record_offset":188380,"ancestor_record_offset":188380,"inlinee":11919,"source_line":57,"archive":"_pdb/B/modules/00104.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":104,"record_offset":189500,"parent_record_offset":189476,"ancestor_record_offset":188380,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00104.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":104,"record_offset":189476,"parent_record_offset":189392,"ancestor_record_offset":188380,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00104.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":104,"record_offset":189392,"parent_record_offset":189364,"ancestor_record_offset":188380,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/00104.jsonl.gz"}
//   [pdb_inline_transitive] alloc::ffi::c_str::impl$2::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\ffi\c_str.rs
//     evidence={"module":104,"record_offset":190276,"parent_record_offset":190252,"ancestor_record_offset":188380,"inlinee":7853,"source_line":696,"archive":"_pdb/B/modules/00104.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":104,"record_offset":191640,"parent_record_offset":191612,"ancestor_record_offset":188380,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00104.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":104,"record_offset":191612,"parent_record_offset":191588,"ancestor_record_offset":188380,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00104.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":104,"record_offset":191588,"parent_record_offset":191564,"ancestor_record_offset":188380,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00104.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":104,"record_offset":189944,"parent_record_offset":189916,"ancestor_record_offset":188380,"inlinee":4776,"source_line":1888,"archive":"_pdb/B/modules/00104.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":104,"record_offset":189916,"parent_record_offset":189888,"ancestor_record_offset":188380,"inlinee":6804,"source_line":2163,"archive":"_pdb/B/modules/00104.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$37::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":104,"record_offset":190476,"parent_record_offset":190452,"ancestor_record_offset":188380,"inlinee":4774,"source_line":2550,"archive":"_pdb/B/modules/00104.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":104,"record_offset":189420,"parent_record_offset":189392,"ancestor_record_offset":188380,"inlinee":5033,"source_line":219,"archive":"_pdb/B/modules/00104.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":104,"record_offset":188792,"parent_record_offset":188768,"ancestor_record_offset":188380,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00104.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":104,"record_offset":189444,"parent_record_offset":189420,"ancestor_record_offset":188380,"inlinee":5034,"source_line":393,"archive":"_pdb/B/modules/00104.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":104,"record_offset":190224,"parent_record_offset":190184,"ancestor_record_offset":188380,"inlinee":9461,"source_line":1252,"archive":"_pdb/B/modules/00104.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":104,"record_offset":189244,"parent_record_offset":189212,"ancestor_record_offset":188380,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/00104.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":104,"record_offset":189272,"parent_record_offset":189212,"ancestor_record_offset":188380,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/00104.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":104,"record_offset":190252,"parent_record_offset":190184,"ancestor_record_offset":188380,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00104.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":104,"record_offset":189976,"parent_record_offset":189944,"ancestor_record_offset":188380,"inlinee":4777,"source_line":425,"archive":"_pdb/B/modules/00104.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":104,"record_offset":188820,"parent_record_offset":188792,"ancestor_record_offset":188380,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00104.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":104,"record_offset":188848,"parent_record_offset":188792,"ancestor_record_offset":188380,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00104.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":104,"record_offset":189004,"parent_record_offset":188940,"ancestor_record_offset":188380,"inlinee":4945,"source_line":2871,"archive":"_pdb/B/modules/00104.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":104,"record_offset":188676,"parent_record_offset":188640,"ancestor_record_offset":188380,"inlinee":10004,"source_line":2920,"archive":"_pdb/B/modules/00104.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":104,"record_offset":189096,"parent_record_offset":189072,"ancestor_record_offset":188380,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00104.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":104,"record_offset":190004,"parent_record_offset":189944,"ancestor_record_offset":188380,"inlinee":4128,"source_line":3760,"archive":"_pdb/B/modules/00104.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":104,"record_offset":189028,"parent_record_offset":189004,"ancestor_record_offset":188380,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/00104.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":104,"record_offset":188700,"parent_record_offset":188676,"ancestor_record_offset":188380,"inlinee":5825,"source_line":3828,"archive":"_pdb/B/modules/00104.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":104,"record_offset":189120,"parent_record_offset":189096,"ancestor_record_offset":188380,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00104.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":104,"record_offset":190572,"parent_record_offset":190512,"ancestor_record_offset":188380,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/00104.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":104,"record_offset":190604,"parent_record_offset":190476,"ancestor_record_offset":188380,"inlinee":4779,"source_line":4075,"archive":"_pdb/B/modules/00104.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":104,"record_offset":188912,"parent_record_offset":188884,"ancestor_record_offset":188380,"inlinee":9996,"source_line":653,"archive":"_pdb/B/modules/00104.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":104,"record_offset":189300,"parent_record_offset":189212,"ancestor_record_offset":188380,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/00104.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":104,"record_offset":189212,"parent_record_offset":189188,"ancestor_record_offset":188380,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/00104.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":104,"record_offset":189324,"parent_record_offset":189300,"ancestor_record_offset":188380,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/00104.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":104,"record_offset":188640,"parent_record_offset":188612,"ancestor_record_offset":188380,"inlinee":10003,"source_line":66,"archive":"_pdb/B/modules/00104.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":104,"record_offset":188940,"parent_record_offset":188912,"ancestor_record_offset":188380,"inlinee":4941,"source_line":100,"archive":"_pdb/B/modules/00104.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":104,"record_offset":188732,"parent_record_offset":188640,"ancestor_record_offset":188380,"inlinee":5827,"source_line":1104,"archive":"_pdb/B/modules/00104.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":104,"record_offset":188976,"parent_record_offset":188940,"ancestor_record_offset":188380,"inlinee":4943,"source_line":1120,"archive":"_pdb/B/modules/00104.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$59::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":104,"record_offset":189732,"parent_record_offset":189708,"ancestor_record_offset":188380,"inlinee":5134,"source_line":1639,"archive":"_pdb/B/modules/00104.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":104,"record_offset":189780,"parent_record_offset":189756,"ancestor_record_offset":188380,"inlinee":5138,"source_line":599,"archive":"_pdb/B/modules/00104.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":104,"record_offset":189756,"parent_record_offset":189732,"ancestor_record_offset":188380,"inlinee":5136,"source_line":617,"archive":"_pdb/B/modules/00104.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":104,"record_offset":189708,"parent_record_offset":189684,"ancestor_record_offset":188380,"inlinee":5132,"source_line":2182,"archive":"_pdb/B/modules/00104.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::screenshot::next_requested_screenshot_stage
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 511
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2722
// 首条位置: {"module":66,"inlinee":10018}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$18::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":66,"record_offset":17644,"parent_record_offset":17432,"ancestor_record_offset":17432,"inlinee":7678,"source_line":3482,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":66,"record_offset":17992,"parent_record_offset":17432,"ancestor_record_offset":17432,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":66,"record_offset":18020,"parent_record_offset":17432,"ancestor_record_offset":17432,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$190::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":66,"record_offset":17840,"parent_record_offset":17432,"ancestor_record_offset":17432,"inlinee":5335,"source_line":156,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":66,"record_offset":17488,"parent_record_offset":17432,"ancestor_record_offset":17432,"inlinee":10002,"source_line":213,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":66,"record_offset":17740,"parent_record_offset":17716,"ancestor_record_offset":17432,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":66,"record_offset":17788,"parent_record_offset":17764,"ancestor_record_offset":17432,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":66,"record_offset":17764,"parent_record_offset":17740,"ancestor_record_offset":17432,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":66,"record_offset":17716,"parent_record_offset":17692,"ancestor_record_offset":17432,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":66,"record_offset":17692,"parent_record_offset":17668,"ancestor_record_offset":17432,"inlinee":5037,"source_line":1593,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":66,"record_offset":17668,"parent_record_offset":17644,"ancestor_record_offset":17432,"inlinee":5310,"source_line":3273,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":66,"record_offset":17952,"parent_record_offset":17928,"ancestor_record_offset":17432,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":66,"record_offset":17876,"parent_record_offset":17840,"ancestor_record_offset":17432,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":66,"record_offset":17928,"parent_record_offset":17904,"ancestor_record_offset":17432,"inlinee":5339,"source_line":99,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$194::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":66,"record_offset":17904,"parent_record_offset":17840,"ancestor_record_offset":17432,"inlinee":5337,"source_line":440,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":66,"record_offset":18124,"parent_record_offset":18068,"ancestor_record_offset":17432,"inlinee":4945,"source_line":2871,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":66,"record_offset":17552,"parent_record_offset":17516,"ancestor_record_offset":17432,"inlinee":10004,"source_line":2920,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":66,"record_offset":18148,"parent_record_offset":18124,"ancestor_record_offset":17432,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":66,"record_offset":17576,"parent_record_offset":17552,"ancestor_record_offset":17432,"inlinee":5825,"source_line":3828,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":66,"record_offset":18044,"parent_record_offset":18020,"ancestor_record_offset":17432,"inlinee":9996,"source_line":653,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":66,"record_offset":17516,"parent_record_offset":17488,"ancestor_record_offset":17432,"inlinee":10003,"source_line":66,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":66,"record_offset":18068,"parent_record_offset":18044,"ancestor_record_offset":17432,"inlinee":4941,"source_line":100,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":66,"record_offset":17608,"parent_record_offset":17516,"ancestor_record_offset":17432,"inlinee":5827,"source_line":1104,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":66,"record_offset":18096,"parent_record_offset":18068,"ancestor_record_offset":17432,"inlinee":4943,"source_line":1120,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::screenshot::handle_screenshot_opportunity
// 记录形式: compiled_procedure; instances=1
// 源码行: 527
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":130,"record_offset":22996,"rva":3264832,"va":5371973952}
// 编译实例 RVAs: 0x31d140
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14003bb80
//     owner=unresolved_namespace; count=2
//     target_aliases=532 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3023189","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371973952,"raw_hex":"e8abded1ff","size":5,"soft_fail":false,"target_va":5368953728,"va":5371976912,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140155400
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1868468","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371973952,"raw_hex":"e84c7de3ff","size":5,"soft_fail":false,"target_va":5370106880,"va":5371975343,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140155d80
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1866106","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371973952,"raw_hex":"e88686e3ff","size":5,"soft_fail":false,"target_va":5370109312,"va":5371975413,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140161c30
//     owner=unresolved_namespace; count=2
//     target_aliases=47 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1820618","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371973952,"raw_hex":"e83638e4ff","size":5,"soft_fail":false,"target_va":5370158128,"va":5371978741,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048f660
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1514909","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371973952,"raw_hex":"e89d1d1700","size":5,"soft_fail":false,"target_va":5373490784,"va":5371975870,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140533b10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2186781","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371973952,"raw_hex":"e81d5e2100","size":5,"soft_fail":false,"target_va":5374163728,"va":5371976942,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140533d90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2187351","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371973952,"raw_hex":"e857602100","size":5,"soft_fail":false,"target_va":5374164368,"va":5371977012,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140587eb0
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2533518","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371973952,"raw_hex":"e88ea82600","size":5,"soft_fail":false,"target_va":5374508720,"va":5371975197,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405ba5f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2740778","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371973952,"raw_hex":"e82ad22900","size":5,"soft_fail":false,"target_va":5374715376,"va":5371974593,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405d9d00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2865122","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371973952,"raw_hex":"e8e2b72b00","size":5,"soft_fail":false,"target_va":5374844160,"va":5371979033,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405ee5e0
//     owner=unresolved_namespace; count=2
//     target_aliases=6 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2949855","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371973952,"raw_hex":"e8df022d00","size":5,"soft_fail":false,"target_va":5374928352,"va":5371978492,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140609520
//     owner=unresolved_namespace; count=11
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3060699","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371973952,"raw_hex":"e8dbb32e00","size":5,"soft_fail":false,"target_va":5375038752,"va":5371978048,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bd4b40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9137846","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371973952,"raw_hex":"e8b66e8b00","size":5,"soft_fail":false,"target_va":5381114688,"va":5371976837,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bd4fc0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9139722","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371973952,"raw_hex":"e80a768b00","size":5,"soft_fail":false,"target_va":5381115840,"va":5371976113,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bd5120
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9139773","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371973952,"raw_hex":"e83d768b00","size":5,"soft_fail":false,"target_va":5381116192,"va":5371976414,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bd60a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9145413","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371973952,"raw_hex":"e8458c8b00","size":5,"soft_fail":false,"target_va":5381120160,"va":5371974742,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bd6380
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9143818","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371973952,"raw_hex":"e80a868b00","size":5,"soft_fail":false,"target_va":5381120896,"va":5371977073,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bd6ce0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9148242","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371973952,"raw_hex":"e852978b00","size":5,"soft_fail":false,"target_va":5381123296,"va":5371975049,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140be24b0
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9193837","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371973952,"raw_hex":"e86d498c00","size":5,"soft_fail":false,"target_va":5381170352,"va":5371976510,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140beea80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9245016","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371973952,"raw_hex":"e858118d00","size":5,"soft_fail":false,"target_va":5381220992,"va":5371975971,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428a40c0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t39346448","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371973952,"raw_hex":"e810615802","size":5,"soft_fail":false,"target_va":5411324096,"va":5371977643,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=10
//     target_aliases=1 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t39871803","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371973952,"raw_hex":"e83b656002","size":5,"soft_fail":false,"target_va":5411847168,"va":5371975360,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923f50
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t39873235","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371973952,"raw_hex":"e8d36a6002","size":5,"soft_fail":false,"target_va":5411848016,"va":5371974776,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923fa0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t39873310","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371973952,"raw_hex":"e81e6b6002","size":5,"soft_fail":false,"target_va":5411848096,"va":5371974781,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t39563946","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371973952,"raw_hex":"e8aab25b02","size":5,"soft_fail":false,"target_va":5411541776,"va":5371977825,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::compute_pass_builder::ComputePass::bind_external_storage_buffer
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t6621251","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371973952,"raw_hex":"e843086500","size":5,"soft_fail":false,"target_va":5378597536,"va":5371976280,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch_2d
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t6624585","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371973952,"raw_hex":"e849156500","size":5,"soft_fail":false,"target_va":5378600064,"va":5371975474,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t39514916","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371973952,"raw_hex":"e824f35a02","size":5,"soft_fail":false,"target_va":5411492368,"va":5371977447,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t39517326","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371973952,"raw_hex":"e88efc5a02","size":5,"soft_fail":false,"target_va":5411494800,"va":5371977469,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::acquire_resource
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t9849534","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371973952,"raw_hex":"e8be4a9600","size":5,"soft_fail":false,"target_va":5381823600,"va":5371974061,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t9846724","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371973952,"raw_hex":"e8c43f9600","size":5,"soft_fail":false,"target_va":5381824016,"va":5371977287,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::buffer::BufferDesc::new_gpu_to_cpu
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t9139669","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371973952,"raw_hex":"e8d5758b00","size":5,"soft_fail":false,"target_va":5381115616,"va":5371975942,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::extent_2d
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t9143439","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371973952,"raw_hex":"e88f848b00","size":5,"soft_fail":false,"target_va":5381117840,"va":5371974396,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::half_res
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t9142800","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371973952,"raw_hex":"e810828b00","size":5,"soft_fail":false,"target_va":5381117744,"va":5371974939,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::new_2d
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t9142060","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371973952,"raw_hex":"e82c7f8b00","size":5,"soft_fail":false,"target_va":5381116576,"va":5371974511,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::sampled
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t9142238","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371973952,"raw_hex":"e8de7f8b00","size":5,"soft_fail":false,"target_va":5381116784,"va":5371974541,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::storage
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t9142285","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371973952,"raw_hex":"e80d808b00","size":5,"soft_fail":false,"target_va":5381116816,"va":5371974526,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::graph::RenderGraph::create_raw_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t9223959","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371973952,"raw_hex":"e817bf8c00","size":5,"soft_fail":false,"target_va":5381199056,"va":5371975092,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":130,"record_offset":25640,"parent_record_offset":22996,"ancestor_record_offset":22996,"inlinee":5345,"source_line":2205,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":130,"record_offset":23376,"parent_record_offset":22996,"ancestor_record_offset":22996,"inlinee":4613,"source_line":2705,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$18::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":130,"record_offset":23404,"parent_record_offset":22996,"ancestor_record_offset":22996,"inlinee":7678,"source_line":3482,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::product
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":130,"record_offset":24760,"parent_record_offset":22996,"ancestor_record_offset":22996,"inlinee":12554,"source_line":3553,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":130,"record_offset":23848,"parent_record_offset":22996,"ancestor_record_offset":22996,"inlinee":5657,"source_line":651,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":130,"record_offset":25192,"parent_record_offset":22996,"ancestor_record_offset":22996,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":130,"record_offset":23904,"parent_record_offset":22996,"ancestor_record_offset":22996,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":130,"record_offset":26584,"parent_record_offset":22996,"ancestor_record_offset":22996,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$190::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":130,"record_offset":23664,"parent_record_offset":22996,"ancestor_record_offset":22996,"inlinee":5335,"source_line":156,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::ComputePass::write_texture
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":130,"record_offset":24644,"parent_record_offset":22996,"ancestor_record_offset":22996,"inlinee":12552,"source_line":171,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::impl$3::read_texture
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":130,"record_offset":24600,"parent_record_offset":22996,"ancestor_record_offset":22996,"inlinee":12551,"source_line":704,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::impl$3::uniform
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":130,"record_offset":24688,"parent_record_offset":22996,"ancestor_record_offset":22996,"inlinee":12553,"source_line":682,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":130,"record_offset":23220,"parent_record_offset":22996,"ancestor_record_offset":22996,"inlinee":10002,"source_line":213,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::impl$14::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":130,"record_offset":25824,"parent_record_offset":22996,"ancestor_record_offset":22996,"inlinee":10006,"source_line":646,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::pass_builder::impl$2::read
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/pass_builder.rs
//     evidence={"module":130,"record_offset":25580,"parent_record_offset":22996,"ancestor_record_offset":22996,"inlinee":9632,"source_line":213,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::recorder::RenderGraphRecorder::add_pass
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/recorder.rs
//     evidence={"module":130,"record_offset":25520,"parent_record_offset":22996,"ancestor_record_offset":22996,"inlinee":9697,"source_line":24,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::recorder::RenderGraphRecorder::export
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/recorder.rs
//     evidence={"module":130,"record_offset":25164,"parent_record_offset":22996,"ancestor_record_offset":22996,"inlinee":9695,"source_line":75,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::recorder::RenderGraphRecorder::import_unowned
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/recorder.rs
//     evidence={"module":130,"record_offset":23940,"parent_record_offset":22996,"ancestor_record_offset":22996,"inlinee":9692,"source_line":55,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::log2
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":130,"record_offset":23968,"parent_record_offset":22996,"ancestor_record_offset":22996,"inlinee":7675,"source_line":502,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::round
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":130,"record_offset":23996,"parent_record_offset":22996,"ancestor_record_offset":22996,"inlinee":6386,"source_line":98,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::screenshot::impl$6::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/screenshot.rs
//     evidence={"module":130,"record_offset":23816,"parent_record_offset":22996,"ancestor_record_offset":22996,"inlinee":12546,"source_line":59,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::screenshot::progressive_blur_downsample
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/screenshot.rs
//     evidence={"module":130,"record_offset":24024,"parent_record_offset":22996,"ancestor_record_offset":22996,"inlinee":12547,"source_line":874,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":130,"record_offset":23508,"parent_record_offset":23484,"ancestor_record_offset":22996,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":130,"record_offset":23556,"parent_record_offset":23532,"ancestor_record_offset":22996,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":130,"record_offset":23532,"parent_record_offset":23508,"ancestor_record_offset":22996,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":130,"record_offset":23484,"parent_record_offset":23460,"ancestor_record_offset":22996,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":130,"record_offset":23460,"parent_record_offset":23428,"ancestor_record_offset":22996,"inlinee":5037,"source_line":1593,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":130,"record_offset":23428,"parent_record_offset":23404,"ancestor_record_offset":22996,"inlinee":5310,"source_line":3273,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\iter.rs
//     evidence={"module":130,"record_offset":24808,"parent_record_offset":24784,"ancestor_record_offset":22996,"inlinee":9468,"source_line":258,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$2::fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\iter.rs
//     evidence={"module":130,"record_offset":24904,"parent_record_offset":24880,"ancestor_record_offset":22996,"inlinee":9474,"source_line":263,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":130,"record_offset":25848,"parent_record_offset":25824,"ancestor_record_offset":22996,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":130,"record_offset":24328,"parent_record_offset":24024,"ancestor_record_offset":22996,"inlinee":4974,"source_line":261,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":130,"record_offset":24520,"parent_record_offset":24496,"ancestor_record_offset":22996,"inlinee":4690,"source_line":1824,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":130,"record_offset":24268,"parent_record_offset":24204,"ancestor_record_offset":22996,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::by_ref_sized::impl$0::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\by_ref_sized.rs
//     evidence={"module":130,"record_offset":24832,"parent_record_offset":24808,"ancestor_record_offset":22996,"inlinee":9470,"source_line":40,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":130,"record_offset":24496,"parent_record_offset":24472,"ancestor_record_offset":22996,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":130,"record_offset":24472,"parent_record_offset":24024,"ancestor_record_offset":22996,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::accum::impl$37::product
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\accum.rs
//     evidence={"module":130,"record_offset":24784,"parent_record_offset":24760,"ancestor_record_offset":22996,"inlinee":9487,"source_line":60,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::accum::impl$37::product::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\accum.rs
//     evidence={"module":130,"record_offset":24928,"parent_record_offset":24904,"ancestor_record_offset":22996,"inlinee":9476,"source_line":64,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":130,"record_offset":24856,"parent_record_offset":24832,"ancestor_record_offset":22996,"inlinee":6092,"source_line":2362,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::try_trait::impl$0::wrap_mut_2::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\try_trait.rs
//     evidence={"module":130,"record_offset":24880,"parent_record_offset":24856,"ancestor_record_offset":22996,"inlinee":9472,"source_line":395,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":130,"record_offset":24728,"parent_record_offset":24688,"ancestor_record_offset":22996,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":130,"record_offset":24144,"parent_record_offset":24024,"ancestor_record_offset":22996,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":130,"record_offset":23872,"parent_record_offset":23848,"ancestor_record_offset":22996,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":130,"record_offset":24556,"parent_record_offset":24024,"ancestor_record_offset":22996,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":130,"record_offset":25216,"parent_record_offset":25192,"ancestor_record_offset":22996,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":130,"record_offset":23624,"parent_record_offset":23600,"ancestor_record_offset":22996,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":130,"record_offset":23776,"parent_record_offset":23752,"ancestor_record_offset":22996,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":130,"record_offset":23700,"parent_record_offset":23664,"ancestor_record_offset":22996,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":130,"record_offset":23600,"parent_record_offset":23428,"ancestor_record_offset":22996,"inlinee":8638,"source_line":221,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":130,"record_offset":23752,"parent_record_offset":23728,"ancestor_record_offset":22996,"inlinee":5339,"source_line":99,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$194::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":130,"record_offset":23728,"parent_record_offset":23664,"ancestor_record_offset":22996,"inlinee":5337,"source_line":440,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":130,"record_offset":26176,"parent_record_offset":26112,"ancestor_record_offset":22996,"inlinee":4945,"source_line":2871,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":130,"record_offset":23284,"parent_record_offset":23248,"ancestor_record_offset":22996,"inlinee":10004,"source_line":2920,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":130,"record_offset":26200,"parent_record_offset":26176,"ancestor_record_offset":22996,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":130,"record_offset":23308,"parent_record_offset":23284,"ancestor_record_offset":22996,"inlinee":5825,"source_line":3828,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::compute_pass_builder::ComputePass::write_texture
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":130,"record_offset":24428,"parent_record_offset":24024,"ancestor_record_offset":22996,"inlinee":12552,"source_line":171,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::compute_pass_builder::impl$3::read_texture
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":130,"record_offset":24384,"parent_record_offset":24024,"ancestor_record_offset":22996,"inlinee":12551,"source_line":704,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":130,"record_offset":26084,"parent_record_offset":26056,"ancestor_record_offset":22996,"inlinee":9996,"source_line":653,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":130,"record_offset":23248,"parent_record_offset":23220,"ancestor_record_offset":22996,"inlinee":10003,"source_line":66,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":130,"record_offset":26112,"parent_record_offset":26084,"ancestor_record_offset":22996,"inlinee":4941,"source_line":100,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":130,"record_offset":23340,"parent_record_offset":23248,"ancestor_record_offset":22996,"inlinee":5827,"source_line":1104,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":130,"record_offset":26148,"parent_record_offset":26112,"ancestor_record_offset":22996,"inlinee":4943,"source_line":1120,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::texture::impl$24::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/texture.rs
//     evidence={"module":130,"record_offset":24352,"parent_record_offset":24328,"ancestor_record_offset":22996,"inlinee":12549,"source_line":49,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::texture::impl$26::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/texture.rs
//     evidence={"module":130,"record_offset":24240,"parent_record_offset":24204,"ancestor_record_offset":22996,"inlinee":9664,"source_line":67,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::recorder::RenderGraphRecorder::create
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/recorder.rs
//     evidence={"module":130,"record_offset":24204,"parent_record_offset":24024,"ancestor_record_offset":22996,"inlinee":9694,"source_line":34,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::resource::Handle::desc
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource.rs
//     evidence={"module":130,"record_offset":24176,"parent_record_offset":24024,"ancestor_record_offset":22996,"inlinee":8273,"source_line":315,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::util::impl$0::same
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/util.rs
//     evidence={"module":130,"record_offset":24296,"parent_record_offset":24204,"ancestor_record_offset":22996,"inlinee":9666,"source_line":8,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::screenshot::handle_screenshot_opportunity::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 635
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2EFF
// 首条位置: {"module":109,"inlinee":12031}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::screenshot::finish_screenshot_capture
// 记录形式: compiled_procedure; instances=1
// 源码行: 657
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":130,"record_offset":27012,"rva":3271200,"va":5371980320}
// 编译实例 RVAs: 0x31ea20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400a2580
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2607324","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371980320,"raw_hex":"e82437d8ff","size":5,"soft_fail":false,"target_va":5369374080,"va":5371981399,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140161c30
//     owner=unresolved_namespace; count=1
//     target_aliases=47 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1823329","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371980320,"raw_hex":"e89f2de4ff","size":5,"soft_fail":false,"target_va":5370158128,"va":5371981452,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402b5c00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-430351","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371980320,"raw_hex":"e8f16ef9ff","size":5,"soft_fail":false,"target_va":5371550720,"va":5371981066,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428a1640
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t39332025","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371980320,"raw_hex":"e8b9285802","size":5,"soft_fail":false,"target_va":5411313216,"va":5371981186,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t39560335","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371980320,"raw_hex":"e88fa45b02","size":5,"soft_fail":false,"target_va":5411541584,"va":5371981244,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-16334","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371980320,"raw_hex":"e832c0ffff","size":5,"soft_fail":false,"target_va":5371964048,"va":5371980377,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t39562908","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371980320,"raw_hex":"e89cae5b02","size":5,"soft_fail":false,"target_va":5411543360,"va":5371980447,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t39511101","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371980320,"raw_hex":"e83de45a02","size":5,"soft_fail":false,"target_va":5411492368,"va":5371981262,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t39513514","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371980320,"raw_hex":"e8aaed5a02","size":5,"soft_fail":false,"target_va":5411494800,"va":5371981281,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::acquire_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t9842660","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371980320,"raw_hex":"e8e42f9600","size":5,"soft_fail":false,"target_va":5381823600,"va":5371980935,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t9842933","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371980320,"raw_hex":"e8f5309600","size":5,"soft_fail":false,"target_va":5381824016,"va":5371981078,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t9404646","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371980320,"raw_hex":"e8e6808f00","size":5,"soft_fail":false,"target_va":5381385536,"va":5371980885,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t9409832","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371980320,"raw_hex":"e828958f00","size":5,"soft_fail":false,"target_va":5381390640,"va":5371980803,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t9405321","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371980320,"raw_hex":"e889838f00","size":5,"soft_fail":false,"target_va":5381386480,"va":5371981154,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t9404351","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371980320,"raw_hex":"e8bf7f8f00","size":5,"soft_fail":false,"target_va":5381384704,"va":5371980348,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::device::GraphicsDevice::wait_idle
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t9280675","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371980320,"raw_hex":"e8a39c8d00","size":5,"soft_fail":false,"target_va":5381261696,"va":5371981016,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t39555867","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371980320,"raw_hex":"e81b935b02","size":5,"soft_fail":false,"target_va":5411537104,"va":5371981232,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::screenshot::read_back_screenshot
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1529","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371980320,"raw_hex":"e8f9050000","size":5,"soft_fail":false,"target_va":5371982560,"va":5371981026,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::impl$18::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":130,"record_offset":29732,"parent_record_offset":27012,"ancestor_record_offset":27012,"inlinee":7678,"source_line":3482,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":130,"record_offset":30144,"parent_record_offset":27012,"ancestor_record_offset":27012,"inlinee":5657,"source_line":651,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":130,"record_offset":30200,"parent_record_offset":27012,"ancestor_record_offset":27012,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$190::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":130,"record_offset":29992,"parent_record_offset":27012,"ancestor_record_offset":27012,"inlinee":5335,"source_line":156,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":130,"record_offset":29576,"parent_record_offset":27012,"ancestor_record_offset":27012,"inlinee":10002,"source_line":213,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":130,"record_offset":28608,"parent_record_offset":27012,"ancestor_record_offset":27012,"inlinee":4416,"source_line":589,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":130,"record_offset":27176,"parent_record_offset":27012,"ancestor_record_offset":27012,"inlinee":4368,"source_line":626,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":130,"record_offset":27632,"parent_record_offset":27012,"ancestor_record_offset":27012,"inlinee":4385,"source_line":659,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":130,"record_offset":29836,"parent_record_offset":29812,"ancestor_record_offset":27012,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":130,"record_offset":29884,"parent_record_offset":29860,"ancestor_record_offset":27012,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":130,"record_offset":29860,"parent_record_offset":29836,"ancestor_record_offset":27012,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":130,"record_offset":29812,"parent_record_offset":29788,"ancestor_record_offset":27012,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":130,"record_offset":29788,"parent_record_offset":29756,"ancestor_record_offset":27012,"inlinee":5037,"source_line":1593,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":130,"record_offset":29756,"parent_record_offset":29732,"ancestor_record_offset":27012,"inlinee":5310,"source_line":3273,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":130,"record_offset":28928,"parent_record_offset":28900,"ancestor_record_offset":27012,"inlinee":4432,"source_line":1817,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":130,"record_offset":28980,"parent_record_offset":28956,"ancestor_record_offset":27012,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":130,"record_offset":28956,"parent_record_offset":28928,"ancestor_record_offset":27012,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":130,"record_offset":28876,"parent_record_offset":28848,"ancestor_record_offset":27012,"inlinee":4429,"source_line":1089,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":130,"record_offset":28900,"parent_record_offset":28876,"ancestor_record_offset":27012,"inlinee":4430,"source_line":1121,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":130,"record_offset":29072,"parent_record_offset":28900,"ancestor_record_offset":27012,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$44::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":130,"record_offset":29184,"parent_record_offset":29160,"ancestor_record_offset":27012,"inlinee":4443,"source_line":1808,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":130,"record_offset":29004,"parent_record_offset":28980,"ancestor_record_offset":27012,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":130,"record_offset":27332,"parent_record_offset":27300,"ancestor_record_offset":27012,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":130,"record_offset":28744,"parent_record_offset":28704,"ancestor_record_offset":27012,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":130,"record_offset":30168,"parent_record_offset":30144,"ancestor_record_offset":27012,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":130,"record_offset":27580,"parent_record_offset":27552,"ancestor_record_offset":27012,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":130,"record_offset":29532,"parent_record_offset":28704,"ancestor_record_offset":27012,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":130,"record_offset":29136,"parent_record_offset":28848,"ancestor_record_offset":27012,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":130,"record_offset":29952,"parent_record_offset":29928,"ancestor_record_offset":27012,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":130,"record_offset":30104,"parent_record_offset":30080,"ancestor_record_offset":27012,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":130,"record_offset":28044,"parent_record_offset":28020,"ancestor_record_offset":27012,"inlinee":4410,"source_line":533,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":130,"record_offset":28020,"parent_record_offset":27996,"ancestor_record_offset":27012,"inlinee":4409,"source_line":686,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":130,"record_offset":28228,"parent_record_offset":28200,"ancestor_record_offset":27012,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":130,"record_offset":29028,"parent_record_offset":29004,"ancestor_record_offset":27012,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":130,"record_offset":27972,"parent_record_offset":27948,"ancestor_record_offset":27012,"inlinee":4407,"source_line":80,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":130,"record_offset":27996,"parent_record_offset":27972,"ancestor_record_offset":27012,"inlinee":4408,"source_line":118,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":130,"record_offset":29928,"parent_record_offset":29756,"ancestor_record_offset":27012,"inlinee":8638,"source_line":221,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":130,"record_offset":30080,"parent_record_offset":30056,"ancestor_record_offset":27012,"inlinee":5339,"source_line":99,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":130,"record_offset":28300,"parent_record_offset":28272,"ancestor_record_offset":27012,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$183::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":130,"record_offset":27948,"parent_record_offset":27832,"ancestor_record_offset":27012,"inlinee":4405,"source_line":392,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$194::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":130,"record_offset":30056,"parent_record_offset":29992,"ancestor_record_offset":27012,"inlinee":5337,"source_line":440,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":130,"record_offset":27300,"parent_record_offset":27272,"ancestor_record_offset":27012,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":130,"record_offset":27660,"parent_record_offset":27632,"ancestor_record_offset":27012,"inlinee":4386,"source_line":1488,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":130,"record_offset":27800,"parent_record_offset":27768,"ancestor_record_offset":27012,"inlinee":4396,"source_line":128,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":130,"record_offset":27768,"parent_record_offset":27732,"ancestor_record_offset":27012,"inlinee":4394,"source_line":218,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":130,"record_offset":27708,"parent_record_offset":27684,"ancestor_record_offset":27012,"inlinee":4390,"source_line":315,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":130,"record_offset":27732,"parent_record_offset":27708,"ancestor_record_offset":27012,"inlinee":4392,"source_line":709,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":130,"record_offset":27684,"parent_record_offset":27660,"ancestor_record_offset":27012,"inlinee":4388,"source_line":798,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":130,"record_offset":28272,"parent_record_offset":27732,"ancestor_record_offset":27012,"inlinee":4400,"source_line":659,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":130,"record_offset":27272,"parent_record_offset":27240,"ancestor_record_offset":27012,"inlinee":4373,"source_line":433,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":130,"record_offset":27240,"parent_record_offset":27212,"ancestor_record_offset":27012,"inlinee":4372,"source_line":463,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":130,"record_offset":27432,"parent_record_offset":27400,"ancestor_record_offset":27012,"inlinee":4380,"source_line":501,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":130,"record_offset":27528,"parent_record_offset":27432,"ancestor_record_offset":27012,"inlinee":4381,"source_line":521,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":130,"record_offset":27400,"parent_record_offset":27376,"ancestor_record_offset":27012,"inlinee":4379,"source_line":533,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":130,"record_offset":27212,"parent_record_offset":27176,"ancestor_record_offset":27012,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":130,"record_offset":27552,"parent_record_offset":27528,"ancestor_record_offset":27012,"inlinee":4383,"source_line":189,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":130,"record_offset":27832,"parent_record_offset":27800,"ancestor_record_offset":27012,"inlinee":4398,"source_line":79,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":130,"record_offset":28116,"parent_record_offset":27832,"ancestor_record_offset":27012,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":130,"record_offset":28088,"parent_record_offset":27832,"ancestor_record_offset":27012,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":130,"record_offset":27892,"parent_record_offset":27832,"ancestor_record_offset":27012,"inlinee":4403,"source_line":24,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":130,"record_offset":30320,"parent_record_offset":30256,"ancestor_record_offset":27012,"inlinee":4945,"source_line":2871,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":130,"record_offset":29640,"parent_record_offset":29604,"ancestor_record_offset":27012,"inlinee":10004,"source_line":2920,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":130,"record_offset":30344,"parent_record_offset":30320,"ancestor_record_offset":27012,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":130,"record_offset":29664,"parent_record_offset":29640,"ancestor_record_offset":27012,"inlinee":5825,"source_line":3828,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":130,"record_offset":30228,"parent_record_offset":30200,"ancestor_record_offset":27012,"inlinee":9996,"source_line":653,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":130,"record_offset":29604,"parent_record_offset":29576,"ancestor_record_offset":27012,"inlinee":10003,"source_line":66,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":130,"record_offset":30256,"parent_record_offset":30228,"ancestor_record_offset":27012,"inlinee":4941,"source_line":100,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":130,"record_offset":29696,"parent_record_offset":29604,"ancestor_record_offset":27012,"inlinee":5827,"source_line":1104,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":130,"record_offset":30292,"parent_record_offset":30256,"ancestor_record_offset":27012,"inlinee":4943,"source_line":1120,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":130,"record_offset":28640,"parent_record_offset":28608,"ancestor_record_offset":27012,"inlinee":4418,"source_line":425,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":130,"record_offset":30452,"parent_record_offset":30420,"ancestor_record_offset":27012,"inlinee":4966,"source_line":601,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":130,"record_offset":30952,"parent_record_offset":30692,"ancestor_record_offset":27012,"inlinee":4968,"source_line":602,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":130,"record_offset":28772,"parent_record_offset":28744,"ancestor_record_offset":27012,"inlinee":4423,"source_line":90,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":130,"record_offset":28848,"parent_record_offset":28704,"ancestor_record_offset":27012,"inlinee":4427,"source_line":429,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":130,"record_offset":29108,"parent_record_offset":28848,"ancestor_record_offset":27012,"inlinee":4441,"source_line":591,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":130,"record_offset":28800,"parent_record_offset":28772,"ancestor_record_offset":27012,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":130,"record_offset":28704,"parent_record_offset":28672,"ancestor_record_offset":27012,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":130,"record_offset":28672,"parent_record_offset":28640,"ancestor_record_offset":27012,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::screenshot::finish_screenshot_capture::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 689
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2DB3
// 首条位置: {"module":104,"inlinee":11699}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::screenshot::hurry_active_screenshots_on_shutdown
// 记录形式: compiled_procedure; instances=1
// 源码行: 692
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":130,"record_offset":31544,"rva":3272400,"va":5371981520}
// 编译实例 RVAs: 0x31eed0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140161c30
//     owner=unresolved_namespace; count=1
//     target_aliases=47 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1824413","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371981520,"raw_hex":"e86329e4ff","size":5,"soft_fail":false,"target_va":5370158128,"va":5371982536,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405fc480
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3003311","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371981520,"raw_hex":"e8afd32d00","size":5,"soft_fail":false,"target_va":5374985344,"va":5371982028,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14060a8f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3061510","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371981520,"raw_hex":"e806b72e00","size":5,"soft_fail":false,"target_va":5375043824,"va":5371982309,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3099531","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371981520,"raw_hex":"e88b4b2f00","size":5,"soft_fail":false,"target_va":5375081504,"va":5371981968,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428a45f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t39343206","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371981520,"raw_hex":"e866545802","size":5,"soft_fail":false,"target_va":5411325424,"va":5371982213,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c10
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t39865077","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371981520,"raw_hex":"e8f54a6002","size":5,"soft_fail":false,"target_va":5411847184,"va":5371982102,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t39510201","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371981520,"raw_hex":"e8b9e05a02","size":5,"soft_fail":false,"target_va":5411492368,"va":5371982162,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t39512614","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371981520,"raw_hex":"e826ea5a02","size":5,"soft_fail":false,"target_va":5411494800,"va":5371982181,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::acquire_resource
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t9841940","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371981520,"raw_hex":"e8142d9600","size":5,"soft_fail":false,"target_va":5381823600,"va":5371981655,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t9842288","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371981520,"raw_hex":"e8702e9600","size":5,"soft_fail":false,"target_va":5381824016,"va":5371981723,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::JoinHandle$LT$T$GT$::join
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-618560","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371981520,"raw_hex":"e8c08ff6ff","size":5,"soft_fail":false,"target_va":5371363360,"va":5371981915,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tjmp\t39512519","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5371981520,"raw_hex":"e9c7e95a02","size":5,"soft_fail":false,"target_va":5411494800,"va":5371982276,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*%rax","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5371981520,"raw_hex":"ffd0","size":2,"soft_fail":false,"va":5371981945,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::drain
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":130,"record_offset":33408,"parent_record_offset":31544,"ancestor_record_offset":31544,"inlinee":5295,"source_line":2606,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::drain::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\drain.rs
//     evidence={"module":130,"record_offset":33728,"parent_record_offset":31544,"ancestor_record_offset":31544,"inlinee":8249,"source_line":156,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$18::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":130,"record_offset":32624,"parent_record_offset":31544,"ancestor_record_offset":31544,"inlinee":7678,"source_line":3482,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::iterators::impl$0::last
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":130,"record_offset":32280,"parent_record_offset":31544,"ancestor_record_offset":31544,"inlinee":10715,"source_line":26,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::reader::EventReader::read
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/reader.rs
//     evidence={"module":130,"record_offset":31724,"parent_record_offset":31544,"ancestor_record_offset":31544,"inlinee":8034,"source_line":27,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\collect.rs
//     evidence={"module":130,"record_offset":33700,"parent_record_offset":31544,"ancestor_record_offset":31544,"inlinee":4756,"source_line":322,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":130,"record_offset":32788,"parent_record_offset":31544,"ancestor_record_offset":31544,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$190::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":130,"record_offset":32708,"parent_record_offset":31544,"ancestor_record_offset":31544,"inlinee":5335,"source_line":156,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":130,"record_offset":32468,"parent_record_offset":31544,"ancestor_record_offset":31544,"inlinee":10002,"source_line":213,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_direct] once_cell::sync::impl$12::deref
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/lib.rs
//     evidence={"module":130,"record_offset":32976,"parent_record_offset":31544,"ancestor_record_offset":31544,"inlinee":6040,"source_line":1375,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":130,"record_offset":34168,"parent_record_offset":34144,"ancestor_record_offset":31544,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":130,"record_offset":34144,"parent_record_offset":34060,"ancestor_record_offset":31544,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":130,"record_offset":34060,"parent_record_offset":34032,"ancestor_record_offset":31544,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":130,"record_offset":33524,"parent_record_offset":33500,"ancestor_record_offset":31544,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":130,"record_offset":33572,"parent_record_offset":33548,"ancestor_record_offset":31544,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":130,"record_offset":33548,"parent_record_offset":33524,"ancestor_record_offset":31544,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":130,"record_offset":32672,"parent_record_offset":32648,"ancestor_record_offset":31544,"inlinee":5037,"source_line":1593,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":130,"record_offset":33500,"parent_record_offset":33408,"ancestor_record_offset":31544,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":130,"record_offset":31952,"parent_record_offset":31928,"ancestor_record_offset":31544,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":130,"record_offset":33444,"parent_record_offset":33408,"ancestor_record_offset":31544,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":130,"record_offset":33472,"parent_record_offset":33408,"ancestor_record_offset":31544,"inlinee":5297,"source_line":1895,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::drain::impl$5::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\drain.rs
//     evidence={"module":130,"record_offset":33940,"parent_record_offset":33916,"ancestor_record_offset":31544,"inlinee":8251,"source_line":157,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::drain::impl$7::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\drain.rs
//     evidence={"module":130,"record_offset":34384,"parent_record_offset":34356,"ancestor_record_offset":31544,"inlinee":5300,"source_line":175,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::drain::impl$7::drop::impl$0::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\drain.rs
//     evidence={"module":130,"record_offset":34660,"parent_record_offset":34636,"ancestor_record_offset":31544,"inlinee":5302,"source_line":180,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":130,"record_offset":31928,"parent_record_offset":31828,"ancestor_record_offset":31544,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":130,"record_offset":32648,"parent_record_offset":32624,"ancestor_record_offset":31544,"inlinee":5310,"source_line":3273,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::event_cursor::EventCursor::read_with_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/event_cursor.rs
//     evidence={"module":130,"record_offset":31796,"parent_record_offset":31752,"ancestor_record_offset":31544,"inlinee":8037,"source_line":88,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::iterators::EventIteratorWithId::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":130,"record_offset":31828,"parent_record_offset":31796,"ancestor_record_offset":31544,"inlinee":8039,"source_line":54,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::iterators::impl$3::last
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":130,"record_offset":32312,"parent_record_offset":32280,"ancestor_record_offset":31544,"inlinee":10716,"source_line":112,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$19::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/system_param.rs
//     evidence={"module":130,"record_offset":31752,"parent_record_offset":31724,"ancestor_record_offset":31544,"inlinee":10713,"source_line":1234,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":130,"record_offset":34088,"parent_record_offset":34060,"ancestor_record_offset":31544,"inlinee":5033,"source_line":219,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":130,"record_offset":34988,"parent_record_offset":34772,"ancestor_record_offset":31544,"inlinee":5303,"source_line":4520,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::last
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":130,"record_offset":32344,"parent_record_offset":32312,"ancestor_record_offset":31544,"inlinee":10717,"source_line":176,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":130,"record_offset":34464,"parent_record_offset":34440,"ancestor_record_offset":31544,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":130,"record_offset":34112,"parent_record_offset":34088,"ancestor_record_offset":31544,"inlinee":5034,"source_line":393,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::take
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":130,"record_offset":34440,"parent_record_offset":34384,"ancestor_record_offset":31544,"inlinee":5313,"source_line":793,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":130,"record_offset":32056,"parent_record_offset":32032,"ancestor_record_offset":31544,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":130,"record_offset":31872,"parent_record_offset":31828,"ancestor_record_offset":31544,"inlinee":8040,"source_line":1859,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":130,"record_offset":32372,"parent_record_offset":32344,"ancestor_record_offset":31544,"inlinee":4518,"source_line":1447,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":130,"record_offset":33916,"parent_record_offset":33728,"ancestor_record_offset":31544,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$39::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":130,"record_offset":32432,"parent_record_offset":32312,"ancestor_record_offset":31544,"inlinee":4927,"source_line":2486,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":130,"record_offset":34596,"parent_record_offset":34572,"ancestor_record_offset":31544,"inlinee":4937,"source_line":774,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":130,"record_offset":34032,"parent_record_offset":34004,"ancestor_record_offset":31544,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":130,"record_offset":33660,"parent_record_offset":33636,"ancestor_record_offset":31544,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":130,"record_offset":34572,"parent_record_offset":34548,"ancestor_record_offset":31544,"inlinee":4936,"source_line":947,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":130,"record_offset":33876,"parent_record_offset":33852,"ancestor_record_offset":31544,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":130,"record_offset":34548,"parent_record_offset":34524,"ancestor_record_offset":31544,"inlinee":4935,"source_line":908,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":130,"record_offset":32752,"parent_record_offset":32708,"ancestor_record_offset":31544,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":130,"record_offset":33964,"parent_record_offset":33940,"ancestor_record_offset":31544,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":130,"record_offset":34488,"parent_record_offset":34464,"ancestor_record_offset":31544,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":130,"record_offset":31984,"parent_record_offset":31828,"ancestor_record_offset":31544,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":130,"record_offset":33612,"parent_record_offset":33408,"ancestor_record_offset":31544,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":130,"record_offset":32032,"parent_record_offset":32008,"ancestor_record_offset":31544,"inlinee":8042,"source_line":364,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":130,"record_offset":32008,"parent_record_offset":31984,"ancestor_record_offset":31544,"inlinee":8041,"source_line":537,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":130,"record_offset":33636,"parent_record_offset":33612,"ancestor_record_offset":31544,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":130,"record_offset":33852,"parent_record_offset":33828,"ancestor_record_offset":31544,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":130,"record_offset":34524,"parent_record_offset":34384,"ancestor_record_offset":31544,"inlinee":9857,"source_line":141,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":130,"record_offset":33752,"parent_record_offset":33728,"ancestor_record_offset":31544,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":130,"record_offset":33828,"parent_record_offset":33752,"ancestor_record_offset":31544,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicPtr::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":130,"record_offset":33148,"parent_record_offset":33120,"ancestor_record_offset":31544,"inlinee":6050,"source_line":1566,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":130,"record_offset":32908,"parent_record_offset":32844,"ancestor_record_offset":31544,"inlinee":4945,"source_line":2871,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":130,"record_offset":32532,"parent_record_offset":32496,"ancestor_record_offset":31544,"inlinee":10004,"source_line":2920,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":130,"record_offset":32932,"parent_record_offset":32908,"ancestor_record_offset":31544,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":130,"record_offset":32556,"parent_record_offset":32532,"ancestor_record_offset":31544,"inlinee":5825,"source_line":3828,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":130,"record_offset":33176,"parent_record_offset":33148,"ancestor_record_offset":31544,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":130,"record_offset":32816,"parent_record_offset":32788,"ancestor_record_offset":31544,"inlinee":9996,"source_line":653,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::imp::OnceCell::is_initialized
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/imp_std.rs
//     evidence={"module":130,"record_offset":33120,"parent_record_offset":33092,"ancestor_record_offset":31544,"inlinee":6048,"source_line":49,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::Lazy::force
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/lib.rs
//     evidence={"module":130,"record_offset":33004,"parent_record_offset":32976,"ancestor_record_offset":31544,"inlinee":6042,"source_line":1308,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::OnceCell::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/lib.rs
//     evidence={"module":130,"record_offset":33092,"parent_record_offset":33060,"ancestor_record_offset":31544,"inlinee":6046,"source_line":963,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::OnceCell::get_or_init
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/lib.rs
//     evidence={"module":130,"record_offset":33032,"parent_record_offset":33004,"ancestor_record_offset":31544,"inlinee":6044,"source_line":1115,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::OnceCell::get_or_try_init
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/lib.rs
//     evidence={"module":130,"record_offset":33060,"parent_record_offset":33032,"ancestor_record_offset":31544,"inlinee":6045,"source_line":1152,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":130,"record_offset":32496,"parent_record_offset":32468,"ancestor_record_offset":31544,"inlinee":10003,"source_line":66,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":130,"record_offset":32844,"parent_record_offset":32816,"ancestor_record_offset":31544,"inlinee":4941,"source_line":100,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":130,"record_offset":32588,"parent_record_offset":32496,"ancestor_record_offset":31544,"inlinee":5827,"source_line":1104,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":130,"record_offset":32880,"parent_record_offset":32844,"ancestor_record_offset":31544,"inlinee":4943,"source_line":1120,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::screenshot::read_back_screenshot
// 记录形式: compiled_procedure; instances=1
// 源码行: 719
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":130,"record_offset":35560,"rva":3273440,"va":5371982560}
// 编译实例 RVAs: 0x31f2e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t10658959","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371982560,"raw_hex":"e88fa4a200","size":5,"soft_fail":false,"target_va":5382644528,"va":5371985564,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400a2580
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2612117","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371982560,"raw_hex":"e86b24d8ff","size":5,"soft_fail":false,"target_va":5369374080,"va":5371986192,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140161c30
//     owner=unresolved_namespace; count=1
//     target_aliases=47 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1829375","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371982560,"raw_hex":"e80116e4ff","size":5,"soft_fail":false,"target_va":5370158128,"va":5371987498,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402b5ea0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-433617","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371982560,"raw_hex":"e82f62f9ff","size":5,"soft_fail":false,"target_va":5371551392,"va":5371985004,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e7a00
//     owner=unresolved_namespace; count=1
//     target_aliases=92 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2913313","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371982560,"raw_hex":"e821742c00","size":5,"soft_fail":false,"target_va":5374900736,"va":5371987418,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405f5c20
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2972114","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371982560,"raw_hex":"e8d2592d00","size":5,"soft_fail":false,"target_va":5374958624,"va":5371986505,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405fdd60
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3008288","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371982560,"raw_hex":"e820e72d00","size":5,"soft_fail":false,"target_va":5374991712,"va":5371983419,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3097095","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371982560,"raw_hex":"e807422f00","size":5,"soft_fail":false,"target_va":5375081488,"va":5371984388,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3098092","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371982560,"raw_hex":"e8ec452f00","size":5,"soft_fail":false,"target_va":5375081504,"va":5371983407,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067dd80
//     owner=unresolved_namespace; count=1
//     target_aliases=69 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3530656","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371982560,"raw_hex":"e8a0df3500","size":5,"soft_fail":false,"target_va":5375516032,"va":5371985371,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140be24b0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9184756","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371982560,"raw_hex":"e8f4258c00","size":5,"soft_fail":false,"target_va":5381170352,"va":5371985591,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428a1640
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t39329385","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371982560,"raw_hex":"e8691e5802","size":5,"soft_fail":false,"target_va":5411313216,"va":5371983826,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428a45f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t39340471","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371982560,"raw_hex":"e8b7495802","size":5,"soft_fail":false,"target_va":5411325424,"va":5371984948,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t39862711","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371982560,"raw_hex":"e8b7416002","size":5,"soft_fail":false,"target_va":5411847168,"va":5371984452,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2371846","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371982560,"raw_hex":"e806312400","size":5,"soft_fail":false,"target_va":5374356912,"va":5371985061,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t39555037","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371982560,"raw_hex":"e8dd8f5b02","size":5,"soft_fail":false,"target_va":5411540963,"va":5371985921,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t10669443","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371982560,"raw_hex":"e883cda200","size":5,"soft_fail":false,"target_va":5382655024,"va":5371985576,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t39555847","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371982560,"raw_hex":"e807935b02","size":5,"soft_fail":false,"target_va":5411541584,"va":5371985732,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t39555937","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371982560,"raw_hex":"e861935b02","size":5,"soft_fail":false,"target_va":5411541776,"va":5371985834,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<tiny_glade::screenshot::ScreenshotSink> >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-2385121","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371982560,"raw_hex":"e81f9bdbff","size":5,"soft_fail":false,"target_va":5369601856,"va":5371986972,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t39556871","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371982560,"raw_hex":"e807975b02","size":5,"soft_fail":false,"target_va":5411542880,"va":5371986004,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t-18589","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371982560,"raw_hex":"e863b7ffff","size":5,"soft_fail":false,"target_va":5371964048,"va":5371982632,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t39560653","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371982560,"raw_hex":"e8cda55b02","size":5,"soft_fail":false,"target_va":5411543360,"va":5371982702,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t39506572","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371982560,"raw_hex":"e88cd25a02","size":5,"soft_fail":false,"target_va":5411492368,"va":5371985791,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t39508978","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371982560,"raw_hex":"e8f2db5a02","size":5,"soft_fail":false,"target_va":5411494800,"va":5371985817,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::acquire_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t9838610","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371982560,"raw_hex":"e812209600","size":5,"soft_fail":false,"target_va":5381823600,"va":5371984985,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t9838887","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371982560,"raw_hex":"e827219600","size":5,"soft_fail":false,"target_va":5381824016,"va":5371985124,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t9402387","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371982560,"raw_hex":"e813788f00","size":5,"soft_fail":false,"target_va":5381385536,"va":5371983144,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t9407576","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371982560,"raw_hex":"e8588c8f00","size":5,"soft_fail":false,"target_va":5381390640,"va":5371983059,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t9401231","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371982560,"raw_hex":"e88f738f00","size":5,"soft_fail":false,"target_va":5381386480,"va":5371985244,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t9402096","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371982560,"raw_hex":"e8f0768f00","size":5,"soft_fail":false,"target_va":5381384704,"va":5371982603,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::Builder::name
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t10706317","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371982560,"raw_hex":"e88d5da300","size":5,"soft_fail":false,"target_va":5382691056,"va":5371984734,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::Builder::spawn
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-613746","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371982560,"raw_hex":"e88ea2f6ff","size":5,"soft_fail":false,"target_va":5371371136,"va":5371984877,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t39551379","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371982560,"raw_hex":"e893815b02","size":5,"soft_fail":false,"target_va":5411537104,"va":5371985720,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::screenshot::write_image_with_callback_impl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3926","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371982560,"raw_hex":"e8560f0000","size":5,"soft_fail":false,"target_va":5371987616,"va":5371983685,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*24(%r8)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5371982560,"raw_hex":"41ff5018","size":4,"soft_fail":false,"va":5371983376,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":130,"record_offset":46608,"parent_record_offset":35560,"ancestor_record_offset":35560,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::take
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":130,"record_offset":38144,"parent_record_offset":35560,"ancestor_record_offset":35560,"inlinee":4908,"source_line":1726,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":130,"record_offset":38260,"parent_record_offset":35560,"ancestor_record_offset":35560,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":130,"record_offset":46496,"parent_record_offset":35560,"ancestor_record_offset":35560,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":130,"record_offset":46640,"parent_record_offset":35560,"ancestor_record_offset":35560,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":130,"record_offset":46524,"parent_record_offset":35560,"ancestor_record_offset":35560,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":130,"record_offset":37160,"parent_record_offset":35560,"ancestor_record_offset":35560,"inlinee":4416,"source_line":589,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":130,"record_offset":35728,"parent_record_offset":35560,"ancestor_record_offset":35560,"inlinee":4368,"source_line":626,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":130,"record_offset":36184,"parent_record_offset":35560,"ancestor_record_offset":35560,"inlinee":4385,"source_line":659,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::screenshot::ScreenshotCapturedCallback::call
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/screenshot.rs
//     evidence={"module":130,"record_offset":38412,"parent_record_offset":35560,"ancestor_record_offset":35560,"inlinee":12555,"source_line":110,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::screenshot::write_image
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/screenshot.rs
//     evidence={"module":130,"record_offset":38900,"parent_record_offset":35560,"ancestor_record_offset":35560,"inlinee":12556,"source_line":750,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":130,"record_offset":43492,"parent_record_offset":43468,"ancestor_record_offset":35560,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":130,"record_offset":43516,"parent_record_offset":43492,"ancestor_record_offset":35560,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":130,"record_offset":38712,"parent_record_offset":38688,"ancestor_record_offset":35560,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":130,"record_offset":43468,"parent_record_offset":43292,"ancestor_record_offset":35560,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":130,"record_offset":38688,"parent_record_offset":38604,"ancestor_record_offset":35560,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$28::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":130,"record_offset":38576,"parent_record_offset":38412,"ancestor_record_offset":35560,"inlinee":7605,"source_line":1975,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":130,"record_offset":38604,"parent_record_offset":38576,"ancestor_record_offset":35560,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":130,"record_offset":45588,"parent_record_offset":45556,"ancestor_record_offset":35560,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":130,"record_offset":43232,"parent_record_offset":43204,"ancestor_record_offset":35560,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":130,"record_offset":46392,"parent_record_offset":46364,"ancestor_record_offset":35560,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":130,"record_offset":46152,"parent_record_offset":46128,"ancestor_record_offset":35560,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":130,"record_offset":45636,"parent_record_offset":45612,"ancestor_record_offset":35560,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":130,"record_offset":45612,"parent_record_offset":45588,"ancestor_record_offset":35560,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":130,"record_offset":43292,"parent_record_offset":43260,"ancestor_record_offset":35560,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":130,"record_offset":43260,"parent_record_offset":43232,"ancestor_record_offset":35560,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":130,"record_offset":46128,"parent_record_offset":46104,"ancestor_record_offset":35560,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":130,"record_offset":43328,"parent_record_offset":43292,"ancestor_record_offset":35560,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":130,"record_offset":43168,"parent_record_offset":43136,"ancestor_record_offset":35560,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":130,"record_offset":43136,"parent_record_offset":43068,"ancestor_record_offset":35560,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":130,"record_offset":43748,"parent_record_offset":38900,"ancestor_record_offset":35560,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":130,"record_offset":43776,"parent_record_offset":43748,"ancestor_record_offset":35560,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":130,"record_offset":44556,"parent_record_offset":44520,"ancestor_record_offset":35560,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":130,"record_offset":44520,"parent_record_offset":44484,"ancestor_record_offset":35560,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$47::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":130,"record_offset":44484,"parent_record_offset":44448,"ancestor_record_offset":35560,"inlinee":5924,"source_line":2956,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":130,"record_offset":42824,"parent_record_offset":42760,"ancestor_record_offset":35560,"inlinee":6804,"source_line":2163,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":130,"record_offset":45556,"parent_record_offset":45512,"ancestor_record_offset":35560,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":130,"record_offset":43108,"parent_record_offset":43068,"ancestor_record_offset":35560,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":130,"record_offset":45512,"parent_record_offset":38900,"ancestor_record_offset":35560,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":130,"record_offset":43204,"parent_record_offset":43168,"ancestor_record_offset":35560,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$11::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":130,"record_offset":43036,"parent_record_offset":43004,"ancestor_record_offset":35560,"inlinee":4623,"source_line":3286,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":130,"record_offset":43068,"parent_record_offset":43036,"ancestor_record_offset":35560,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":130,"record_offset":38632,"parent_record_offset":38604,"ancestor_record_offset":35560,"inlinee":5033,"source_line":219,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":130,"record_offset":43352,"parent_record_offset":43328,"ancestor_record_offset":35560,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":130,"record_offset":43376,"parent_record_offset":43352,"ancestor_record_offset":35560,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":130,"record_offset":37496,"parent_record_offset":37468,"ancestor_record_offset":35560,"inlinee":4432,"source_line":1817,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":130,"record_offset":37548,"parent_record_offset":37524,"ancestor_record_offset":35560,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":130,"record_offset":37524,"parent_record_offset":37496,"ancestor_record_offset":35560,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":130,"record_offset":37444,"parent_record_offset":37416,"ancestor_record_offset":35560,"inlinee":4429,"source_line":1089,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":130,"record_offset":37468,"parent_record_offset":37444,"ancestor_record_offset":35560,"inlinee":4430,"source_line":1121,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":130,"record_offset":37640,"parent_record_offset":37468,"ancestor_record_offset":35560,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$44::drop
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":130,"record_offset":37752,"parent_record_offset":37728,"ancestor_record_offset":35560,"inlinee":4443,"source_line":1808,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":130,"record_offset":44448,"parent_record_offset":38900,"ancestor_record_offset":35560,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":130,"record_offset":43592,"parent_record_offset":43568,"ancestor_record_offset":35560,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":130,"record_offset":43424,"parent_record_offset":43400,"ancestor_record_offset":35560,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":130,"record_offset":37572,"parent_record_offset":37548,"ancestor_record_offset":35560,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":130,"record_offset":38656,"parent_record_offset":38632,"ancestor_record_offset":35560,"inlinee":5034,"source_line":393,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":130,"record_offset":43400,"parent_record_offset":43376,"ancestor_record_offset":35560,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":130,"record_offset":35884,"parent_record_offset":35852,"ancestor_record_offset":35560,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":130,"record_offset":37312,"parent_record_offset":37268,"ancestor_record_offset":35560,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::take
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":130,"record_offset":38456,"parent_record_offset":38412,"ancestor_record_offset":35560,"inlinee":4908,"source_line":1726,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":130,"record_offset":38540,"parent_record_offset":38412,"ancestor_record_offset":35560,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":130,"record_offset":46696,"parent_record_offset":46664,"ancestor_record_offset":35560,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":130,"record_offset":46724,"parent_record_offset":46664,"ancestor_record_offset":35560,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":130,"record_offset":36132,"parent_record_offset":36104,"ancestor_record_offset":35560,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":130,"record_offset":38100,"parent_record_offset":37268,"ancestor_record_offset":35560,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":130,"record_offset":43568,"parent_record_offset":43168,"ancestor_record_offset":35560,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=39
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":130,"record_offset":37704,"parent_record_offset":37416,"ancestor_record_offset":35560,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":130,"record_offset":36596,"parent_record_offset":36572,"ancestor_record_offset":35560,"inlinee":4410,"source_line":533,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":130,"record_offset":36572,"parent_record_offset":36548,"ancestor_record_offset":35560,"inlinee":4409,"source_line":686,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":130,"record_offset":36780,"parent_record_offset":36752,"ancestor_record_offset":35560,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":130,"record_offset":38196,"parent_record_offset":38168,"ancestor_record_offset":35560,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":130,"record_offset":37596,"parent_record_offset":37572,"ancestor_record_offset":35560,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":130,"record_offset":45084,"parent_record_offset":38900,"ancestor_record_offset":35560,"inlinee":6013,"source_line":1055,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":130,"record_offset":36524,"parent_record_offset":36500,"ancestor_record_offset":35560,"inlinee":4407,"source_line":80,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":130,"record_offset":36548,"parent_record_offset":36524,"ancestor_record_offset":35560,"inlinee":4408,"source_line":118,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":130,"record_offset":36852,"parent_record_offset":36824,"ancestor_record_offset":35560,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$183::next_back
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":130,"record_offset":36500,"parent_record_offset":36384,"ancestor_record_offset":35560,"inlinee":4405,"source_line":392,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":130,"record_offset":35852,"parent_record_offset":35824,"ancestor_record_offset":35560,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":130,"record_offset":36212,"parent_record_offset":36184,"ancestor_record_offset":35560,"inlinee":4386,"source_line":1488,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":130,"record_offset":36352,"parent_record_offset":36320,"ancestor_record_offset":35560,"inlinee":4396,"source_line":128,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":130,"record_offset":36320,"parent_record_offset":36284,"ancestor_record_offset":35560,"inlinee":4394,"source_line":218,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":130,"record_offset":36260,"parent_record_offset":36236,"ancestor_record_offset":35560,"inlinee":4390,"source_line":315,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":130,"record_offset":36284,"parent_record_offset":36260,"ancestor_record_offset":35560,"inlinee":4392,"source_line":709,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":130,"record_offset":36236,"parent_record_offset":36212,"ancestor_record_offset":35560,"inlinee":4388,"source_line":798,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":130,"record_offset":36824,"parent_record_offset":36284,"ancestor_record_offset":35560,"inlinee":4400,"source_line":659,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":130,"record_offset":35824,"parent_record_offset":35792,"ancestor_record_offset":35560,"inlinee":4373,"source_line":433,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":130,"record_offset":35792,"parent_record_offset":35764,"ancestor_record_offset":35560,"inlinee":4372,"source_line":463,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":130,"record_offset":35984,"parent_record_offset":35952,"ancestor_record_offset":35560,"inlinee":4380,"source_line":501,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":130,"record_offset":36080,"parent_record_offset":35984,"ancestor_record_offset":35560,"inlinee":4381,"source_line":521,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":130,"record_offset":35952,"parent_record_offset":35928,"ancestor_record_offset":35560,"inlinee":4379,"source_line":533,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":130,"record_offset":35764,"parent_record_offset":35728,"ancestor_record_offset":35560,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":130,"record_offset":36104,"parent_record_offset":36080,"ancestor_record_offset":35560,"inlinee":4383,"source_line":189,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":130,"record_offset":36384,"parent_record_offset":36352,"ancestor_record_offset":35560,"inlinee":4398,"source_line":79,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":130,"record_offset":36668,"parent_record_offset":36384,"ancestor_record_offset":35560,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":130,"record_offset":36640,"parent_record_offset":36384,"ancestor_record_offset":35560,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":130,"record_offset":36444,"parent_record_offset":36384,"ancestor_record_offset":35560,"inlinee":4403,"source_line":24,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicPtr::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":130,"record_offset":45276,"parent_record_offset":45252,"ancestor_record_offset":35560,"inlinee":6050,"source_line":1566,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":130,"record_offset":45828,"parent_record_offset":45764,"ancestor_record_offset":35560,"inlinee":4945,"source_line":2871,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":130,"record_offset":45420,"parent_record_offset":45384,"ancestor_record_offset":35560,"inlinee":10004,"source_line":2920,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":130,"record_offset":42848,"parent_record_offset":42824,"ancestor_record_offset":35560,"inlinee":6812,"source_line":2956,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":130,"record_offset":46548,"parent_record_offset":46524,"ancestor_record_offset":35560,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":130,"record_offset":42872,"parent_record_offset":42848,"ancestor_record_offset":35560,"inlinee":4128,"source_line":3760,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":130,"record_offset":45852,"parent_record_offset":45828,"ancestor_record_offset":35560,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":130,"record_offset":45444,"parent_record_offset":45420,"ancestor_record_offset":35560,"inlinee":5825,"source_line":3828,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":130,"record_offset":45300,"parent_record_offset":45276,"ancestor_record_offset":35560,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":130,"record_offset":45356,"parent_record_offset":38900,"ancestor_record_offset":35560,"inlinee":10002,"source_line":213,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":130,"record_offset":45736,"parent_record_offset":45708,"ancestor_record_offset":35560,"inlinee":9996,"source_line":653,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":130,"record_offset":46752,"parent_record_offset":46664,"ancestor_record_offset":35560,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":130,"record_offset":46664,"parent_record_offset":46640,"ancestor_record_offset":35560,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":130,"record_offset":46776,"parent_record_offset":46752,"ancestor_record_offset":35560,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::imp::OnceCell::is_initialized
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/imp_std.rs
//     evidence={"module":130,"record_offset":45252,"parent_record_offset":45228,"ancestor_record_offset":35560,"inlinee":6048,"source_line":49,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::Lazy::force
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/lib.rs
//     evidence={"module":130,"record_offset":45152,"parent_record_offset":45128,"ancestor_record_offset":35560,"inlinee":6042,"source_line":1308,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::OnceCell::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/lib.rs
//     evidence={"module":130,"record_offset":45228,"parent_record_offset":45200,"ancestor_record_offset":35560,"inlinee":6046,"source_line":963,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::OnceCell::get_or_init
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/lib.rs
//     evidence={"module":130,"record_offset":45176,"parent_record_offset":45152,"ancestor_record_offset":35560,"inlinee":6044,"source_line":1115,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::OnceCell::get_or_try_init
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/lib.rs
//     evidence={"module":130,"record_offset":45200,"parent_record_offset":45176,"ancestor_record_offset":35560,"inlinee":6045,"source_line":1152,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::impl$12::deref
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/lib.rs
//     evidence={"module":130,"record_offset":45128,"parent_record_offset":38900,"ancestor_record_offset":35560,"inlinee":6040,"source_line":1375,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":130,"record_offset":45384,"parent_record_offset":45356,"ancestor_record_offset":35560,"inlinee":10003,"source_line":66,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":130,"record_offset":45764,"parent_record_offset":45736,"ancestor_record_offset":35560,"inlinee":4941,"source_line":100,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":130,"record_offset":45476,"parent_record_offset":45384,"ancestor_record_offset":35560,"inlinee":5827,"source_line":1104,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":130,"record_offset":45800,"parent_record_offset":45764,"ancestor_record_offset":35560,"inlinee":4943,"source_line":1120,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":130,"record_offset":41752,"parent_record_offset":38900,"ancestor_record_offset":35560,"inlinee":4416,"source_line":589,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":130,"record_offset":37196,"parent_record_offset":37160,"ancestor_record_offset":35560,"inlinee":4418,"source_line":425,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":130,"record_offset":39096,"parent_record_offset":38900,"ancestor_record_offset":35560,"inlinee":4368,"source_line":626,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":130,"record_offset":39760,"parent_record_offset":39724,"ancestor_record_offset":35560,"inlinee":4966,"source_line":601,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":130,"record_offset":40328,"parent_record_offset":40056,"ancestor_record_offset":35560,"inlinee":4968,"source_line":602,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":130,"record_offset":37340,"parent_record_offset":37312,"ancestor_record_offset":35560,"inlinee":4423,"source_line":90,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":130,"record_offset":37416,"parent_record_offset":37268,"ancestor_record_offset":35560,"inlinee":4427,"source_line":429,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":130,"record_offset":37676,"parent_record_offset":37416,"ancestor_record_offset":35560,"inlinee":4441,"source_line":591,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":130,"record_offset":40776,"parent_record_offset":38900,"ancestor_record_offset":35560,"inlinee":4385,"source_line":659,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$10::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":130,"record_offset":42940,"parent_record_offset":42908,"ancestor_record_offset":35560,"inlinee":8126,"source_line":642,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$30::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":130,"record_offset":42908,"parent_record_offset":42760,"ancestor_record_offset":35560,"inlinee":8124,"source_line":1738,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::impl$10::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":130,"record_offset":42972,"parent_record_offset":42940,"ancestor_record_offset":35560,"inlinee":8128,"source_line":13,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":130,"record_offset":37368,"parent_record_offset":37340,"ancestor_record_offset":35560,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] std::sys_common::wtf8::impl$35::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys_common\wtf8.rs
//     evidence={"module":130,"record_offset":43004,"parent_record_offset":42972,"ancestor_record_offset":35560,"inlinee":8130,"source_line":132,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::Builder::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\mod.rs
//     evidence={"module":130,"record_offset":44420,"parent_record_offset":38900,"ancestor_record_offset":35560,"inlinee":12559,"source_line":289,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":130,"record_offset":37268,"parent_record_offset":37232,"ancestor_record_offset":35560,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":130,"record_offset":37232,"parent_record_offset":37196,"ancestor_record_offset":35560,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::screenshot::impl$10::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/screenshot.rs
//     evidence={"module":130,"record_offset":42760,"parent_record_offset":38900,"ancestor_record_offset":35560,"inlinee":12558,"source_line":73,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::screenshot::write_image
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 750
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x310C
// 首条位置: {"module":130,"inlinee":12556}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":130,"record_offset":43748,"parent_record_offset":38900,"ancestor_record_offset":38900,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":130,"record_offset":45512,"parent_record_offset":38900,"ancestor_record_offset":38900,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":130,"record_offset":44448,"parent_record_offset":38900,"ancestor_record_offset":38900,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::take
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":130,"record_offset":39568,"parent_record_offset":38900,"ancestor_record_offset":38900,"inlinee":4908,"source_line":1726,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":130,"record_offset":39684,"parent_record_offset":38900,"ancestor_record_offset":38900,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":130,"record_offset":45084,"parent_record_offset":38900,"ancestor_record_offset":38900,"inlinee":6013,"source_line":1055,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":130,"record_offset":45356,"parent_record_offset":38900,"ancestor_record_offset":38900,"inlinee":10002,"source_line":213,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_direct] once_cell::sync::impl$12::deref
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/lib.rs
//     evidence={"module":130,"record_offset":45128,"parent_record_offset":38900,"ancestor_record_offset":38900,"inlinee":6040,"source_line":1375,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":130,"record_offset":41752,"parent_record_offset":38900,"ancestor_record_offset":38900,"inlinee":4416,"source_line":589,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":130,"record_offset":39096,"parent_record_offset":38900,"ancestor_record_offset":38900,"inlinee":4368,"source_line":626,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":130,"record_offset":40776,"parent_record_offset":38900,"ancestor_record_offset":38900,"inlinee":4385,"source_line":659,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_direct] std::thread::Builder::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\mod.rs
//     evidence={"module":130,"record_offset":44420,"parent_record_offset":38900,"ancestor_record_offset":38900,"inlinee":12559,"source_line":289,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::screenshot::impl$10::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/screenshot.rs
//     evidence={"module":130,"record_offset":42760,"parent_record_offset":38900,"ancestor_record_offset":38900,"inlinee":12558,"source_line":73,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":130,"record_offset":43492,"parent_record_offset":43468,"ancestor_record_offset":38900,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":130,"record_offset":43516,"parent_record_offset":43492,"ancestor_record_offset":38900,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":130,"record_offset":46204,"parent_record_offset":46180,"ancestor_record_offset":38900,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":130,"record_offset":43468,"parent_record_offset":43292,"ancestor_record_offset":38900,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":130,"record_offset":46180,"parent_record_offset":46152,"ancestor_record_offset":38900,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":130,"record_offset":45588,"parent_record_offset":45556,"ancestor_record_offset":38900,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":130,"record_offset":43232,"parent_record_offset":43204,"ancestor_record_offset":38900,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":130,"record_offset":46392,"parent_record_offset":46364,"ancestor_record_offset":38900,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":130,"record_offset":46152,"parent_record_offset":46128,"ancestor_record_offset":38900,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":130,"record_offset":45636,"parent_record_offset":45612,"ancestor_record_offset":38900,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":130,"record_offset":45612,"parent_record_offset":45588,"ancestor_record_offset":38900,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":130,"record_offset":43292,"parent_record_offset":43260,"ancestor_record_offset":38900,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":130,"record_offset":43260,"parent_record_offset":43232,"ancestor_record_offset":38900,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":130,"record_offset":46128,"parent_record_offset":46104,"ancestor_record_offset":38900,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":130,"record_offset":43328,"parent_record_offset":43292,"ancestor_record_offset":38900,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":130,"record_offset":43168,"parent_record_offset":43136,"ancestor_record_offset":38900,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":130,"record_offset":43136,"parent_record_offset":43068,"ancestor_record_offset":38900,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":130,"record_offset":44604,"parent_record_offset":44556,"ancestor_record_offset":38900,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":130,"record_offset":43776,"parent_record_offset":43748,"ancestor_record_offset":38900,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":130,"record_offset":44556,"parent_record_offset":44520,"ancestor_record_offset":38900,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":130,"record_offset":44520,"parent_record_offset":44484,"ancestor_record_offset":38900,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$47::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":130,"record_offset":44484,"parent_record_offset":44448,"ancestor_record_offset":38900,"inlinee":5924,"source_line":2956,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":130,"record_offset":42824,"parent_record_offset":42760,"ancestor_record_offset":38900,"inlinee":6804,"source_line":2163,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":130,"record_offset":45556,"parent_record_offset":45512,"ancestor_record_offset":38900,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":130,"record_offset":43108,"parent_record_offset":43068,"ancestor_record_offset":38900,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":130,"record_offset":43204,"parent_record_offset":43168,"ancestor_record_offset":38900,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$11::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":130,"record_offset":43036,"parent_record_offset":43004,"ancestor_record_offset":38900,"inlinee":4623,"source_line":3286,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":130,"record_offset":43068,"parent_record_offset":43036,"ancestor_record_offset":38900,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":130,"record_offset":43352,"parent_record_offset":43328,"ancestor_record_offset":38900,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":130,"record_offset":43376,"parent_record_offset":43352,"ancestor_record_offset":38900,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":130,"record_offset":40148,"parent_record_offset":40120,"ancestor_record_offset":38900,"inlinee":4432,"source_line":1817,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":130,"record_offset":40200,"parent_record_offset":40176,"ancestor_record_offset":38900,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":130,"record_offset":40176,"parent_record_offset":40148,"ancestor_record_offset":38900,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":130,"record_offset":40088,"parent_record_offset":40056,"ancestor_record_offset":38900,"inlinee":4429,"source_line":1089,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":130,"record_offset":40120,"parent_record_offset":40088,"ancestor_record_offset":38900,"inlinee":4430,"source_line":1121,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":130,"record_offset":40292,"parent_record_offset":40120,"ancestor_record_offset":38900,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$44::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":130,"record_offset":40404,"parent_record_offset":40380,"ancestor_record_offset":38900,"inlinee":4443,"source_line":1808,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":130,"record_offset":43592,"parent_record_offset":43568,"ancestor_record_offset":38900,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":130,"record_offset":43424,"parent_record_offset":43400,"ancestor_record_offset":38900,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":130,"record_offset":39592,"parent_record_offset":39568,"ancestor_record_offset":38900,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":130,"record_offset":43400,"parent_record_offset":43376,"ancestor_record_offset":38900,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":130,"record_offset":39260,"parent_record_offset":39228,"ancestor_record_offset":38900,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":130,"record_offset":39924,"parent_record_offset":39884,"ancestor_record_offset":38900,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":130,"record_offset":39516,"parent_record_offset":39488,"ancestor_record_offset":38900,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":130,"record_offset":40024,"parent_record_offset":39884,"ancestor_record_offset":38900,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":130,"record_offset":43568,"parent_record_offset":43168,"ancestor_record_offset":38900,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":130,"record_offset":39724,"parent_record_offset":39684,"ancestor_record_offset":38900,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":130,"record_offset":41188,"parent_record_offset":41164,"ancestor_record_offset":38900,"inlinee":4410,"source_line":533,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":130,"record_offset":41164,"parent_record_offset":41140,"ancestor_record_offset":38900,"inlinee":4409,"source_line":686,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":130,"record_offset":41372,"parent_record_offset":41344,"ancestor_record_offset":38900,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":130,"record_offset":39620,"parent_record_offset":39592,"ancestor_record_offset":38900,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":130,"record_offset":39648,"parent_record_offset":39592,"ancestor_record_offset":38900,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":130,"record_offset":41116,"parent_record_offset":41092,"ancestor_record_offset":38900,"inlinee":4407,"source_line":80,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":130,"record_offset":41140,"parent_record_offset":41116,"ancestor_record_offset":38900,"inlinee":4408,"source_line":118,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":130,"record_offset":41444,"parent_record_offset":41416,"ancestor_record_offset":38900,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$183::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":130,"record_offset":41092,"parent_record_offset":40976,"ancestor_record_offset":38900,"inlinee":4405,"source_line":392,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":130,"record_offset":39228,"parent_record_offset":39200,"ancestor_record_offset":38900,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":130,"record_offset":40804,"parent_record_offset":40776,"ancestor_record_offset":38900,"inlinee":4386,"source_line":1488,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":130,"record_offset":40944,"parent_record_offset":40912,"ancestor_record_offset":38900,"inlinee":4396,"source_line":128,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":130,"record_offset":40912,"parent_record_offset":40876,"ancestor_record_offset":38900,"inlinee":4394,"source_line":218,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":130,"record_offset":40852,"parent_record_offset":40828,"ancestor_record_offset":38900,"inlinee":4390,"source_line":315,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":130,"record_offset":40876,"parent_record_offset":40852,"ancestor_record_offset":38900,"inlinee":4392,"source_line":709,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":130,"record_offset":40828,"parent_record_offset":40804,"ancestor_record_offset":38900,"inlinee":4388,"source_line":798,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":130,"record_offset":41416,"parent_record_offset":40876,"ancestor_record_offset":38900,"inlinee":4400,"source_line":659,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":130,"record_offset":39200,"parent_record_offset":39164,"ancestor_record_offset":38900,"inlinee":4373,"source_line":433,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":130,"record_offset":39164,"parent_record_offset":39136,"ancestor_record_offset":38900,"inlinee":4372,"source_line":463,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":130,"record_offset":39368,"parent_record_offset":39332,"ancestor_record_offset":38900,"inlinee":4380,"source_line":501,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":130,"record_offset":39464,"parent_record_offset":39368,"ancestor_record_offset":38900,"inlinee":4381,"source_line":521,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":130,"record_offset":39332,"parent_record_offset":39304,"ancestor_record_offset":38900,"inlinee":4379,"source_line":533,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":130,"record_offset":39136,"parent_record_offset":39096,"ancestor_record_offset":38900,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":130,"record_offset":39488,"parent_record_offset":39464,"ancestor_record_offset":38900,"inlinee":4383,"source_line":189,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":130,"record_offset":40976,"parent_record_offset":40944,"ancestor_record_offset":38900,"inlinee":4398,"source_line":79,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":130,"record_offset":41260,"parent_record_offset":40976,"ancestor_record_offset":38900,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":130,"record_offset":41232,"parent_record_offset":40976,"ancestor_record_offset":38900,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":130,"record_offset":41036,"parent_record_offset":40976,"ancestor_record_offset":38900,"inlinee":4403,"source_line":24,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicPtr::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":130,"record_offset":45276,"parent_record_offset":45252,"ancestor_record_offset":38900,"inlinee":6050,"source_line":1566,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":130,"record_offset":45828,"parent_record_offset":45764,"ancestor_record_offset":38900,"inlinee":4945,"source_line":2871,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":130,"record_offset":45420,"parent_record_offset":45384,"ancestor_record_offset":38900,"inlinee":10004,"source_line":2920,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":130,"record_offset":42848,"parent_record_offset":42824,"ancestor_record_offset":38900,"inlinee":6812,"source_line":2956,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":130,"record_offset":42872,"parent_record_offset":42848,"ancestor_record_offset":38900,"inlinee":4128,"source_line":3760,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":130,"record_offset":45852,"parent_record_offset":45828,"ancestor_record_offset":38900,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":130,"record_offset":45444,"parent_record_offset":45420,"ancestor_record_offset":38900,"inlinee":5825,"source_line":3828,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":130,"record_offset":45300,"parent_record_offset":45276,"ancestor_record_offset":38900,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":130,"record_offset":45736,"parent_record_offset":45708,"ancestor_record_offset":38900,"inlinee":9996,"source_line":653,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::imp::OnceCell::is_initialized
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/imp_std.rs
//     evidence={"module":130,"record_offset":45252,"parent_record_offset":45228,"ancestor_record_offset":38900,"inlinee":6048,"source_line":49,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::Lazy::force
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/lib.rs
//     evidence={"module":130,"record_offset":45152,"parent_record_offset":45128,"ancestor_record_offset":38900,"inlinee":6042,"source_line":1308,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::OnceCell::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/lib.rs
//     evidence={"module":130,"record_offset":45228,"parent_record_offset":45200,"ancestor_record_offset":38900,"inlinee":6046,"source_line":963,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::OnceCell::get_or_init
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/lib.rs
//     evidence={"module":130,"record_offset":45176,"parent_record_offset":45152,"ancestor_record_offset":38900,"inlinee":6044,"source_line":1115,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::OnceCell::get_or_try_init
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/lib.rs
//     evidence={"module":130,"record_offset":45200,"parent_record_offset":45176,"ancestor_record_offset":38900,"inlinee":6045,"source_line":1152,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":130,"record_offset":45384,"parent_record_offset":45356,"ancestor_record_offset":38900,"inlinee":10003,"source_line":66,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":130,"record_offset":45764,"parent_record_offset":45736,"ancestor_record_offset":38900,"inlinee":4941,"source_line":100,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":130,"record_offset":45476,"parent_record_offset":45384,"ancestor_record_offset":38900,"inlinee":5827,"source_line":1104,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":130,"record_offset":45800,"parent_record_offset":45764,"ancestor_record_offset":38900,"inlinee":4943,"source_line":1120,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":130,"record_offset":39800,"parent_record_offset":39760,"ancestor_record_offset":38900,"inlinee":4418,"source_line":425,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":130,"record_offset":39760,"parent_record_offset":39724,"ancestor_record_offset":38900,"inlinee":4966,"source_line":601,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":130,"record_offset":40328,"parent_record_offset":40056,"ancestor_record_offset":38900,"inlinee":4968,"source_line":602,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":130,"record_offset":39952,"parent_record_offset":39924,"ancestor_record_offset":38900,"inlinee":4423,"source_line":90,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":130,"record_offset":40056,"parent_record_offset":39884,"ancestor_record_offset":38900,"inlinee":4427,"source_line":429,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":130,"record_offset":42320,"parent_record_offset":42048,"ancestor_record_offset":38900,"inlinee":4441,"source_line":591,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$10::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":130,"record_offset":42940,"parent_record_offset":42908,"ancestor_record_offset":38900,"inlinee":8126,"source_line":642,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$30::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":130,"record_offset":42908,"parent_record_offset":42760,"ancestor_record_offset":38900,"inlinee":8124,"source_line":1738,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::impl$10::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":130,"record_offset":42972,"parent_record_offset":42940,"ancestor_record_offset":38900,"inlinee":8128,"source_line":13,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":130,"record_offset":39980,"parent_record_offset":39952,"ancestor_record_offset":38900,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] std::sys_common::wtf8::impl$35::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys_common\wtf8.rs
//     evidence={"module":130,"record_offset":43004,"parent_record_offset":42972,"ancestor_record_offset":38900,"inlinee":8130,"source_line":132,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":130,"record_offset":39884,"parent_record_offset":39840,"ancestor_record_offset":38900,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":130,"record_offset":39840,"parent_record_offset":39800,"ancestor_record_offset":38900,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::screenshot::write_image::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 765
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1FAE
// 首条位置: {"module":31,"inlinee":8110}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":31,"record_offset":16608,"parent_record_offset":16476,"ancestor_record_offset":16476,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":31,"record_offset":16816,"parent_record_offset":16476,"ancestor_record_offset":16476,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":31,"record_offset":16640,"parent_record_offset":16476,"ancestor_record_offset":16476,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":31,"record_offset":16524,"parent_record_offset":16476,"ancestor_record_offset":16476,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":31,"record_offset":17312,"parent_record_offset":17284,"ancestor_record_offset":16476,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":31,"record_offset":17284,"parent_record_offset":17228,"ancestor_record_offset":16476,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":31,"record_offset":17256,"parent_record_offset":17228,"ancestor_record_offset":16476,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":31,"record_offset":17228,"parent_record_offset":17204,"ancestor_record_offset":16476,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":31,"record_offset":17204,"parent_record_offset":17180,"ancestor_record_offset":16476,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":31,"record_offset":16932,"parent_record_offset":16896,"ancestor_record_offset":16476,"inlinee":4776,"source_line":1888,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$37::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":31,"record_offset":16896,"parent_record_offset":16868,"ancestor_record_offset":16476,"inlinee":4774,"source_line":2550,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":31,"record_offset":16696,"parent_record_offset":16664,"ancestor_record_offset":16476,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":31,"record_offset":16724,"parent_record_offset":16664,"ancestor_record_offset":16476,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":31,"record_offset":16868,"parent_record_offset":16844,"ancestor_record_offset":16476,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":31,"record_offset":16964,"parent_record_offset":16932,"ancestor_record_offset":16476,"inlinee":4777,"source_line":425,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":31,"record_offset":16548,"parent_record_offset":16524,"ancestor_record_offset":16476,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":31,"record_offset":16572,"parent_record_offset":16548,"ancestor_record_offset":16476,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":31,"record_offset":16992,"parent_record_offset":16932,"ancestor_record_offset":16476,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":31,"record_offset":17024,"parent_record_offset":16896,"ancestor_record_offset":16476,"inlinee":4779,"source_line":4075,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":31,"record_offset":16752,"parent_record_offset":16664,"ancestor_record_offset":16476,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":31,"record_offset":16664,"parent_record_offset":16640,"ancestor_record_offset":16476,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":31,"record_offset":16776,"parent_record_offset":16752,"ancestor_record_offset":16476,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::screenshot::write_image::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 775
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2DB4
// 首条位置: {"module":104,"inlinee":11700}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::thread::JoinHandle::is_finished
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\mod.rs
//     evidence={"module":104,"record_offset":21640,"parent_record_offset":21620,"ancestor_record_offset":21620,"inlinee":11423,"source_line":1901,"archive":"_pdb/B/modules/00104.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":104,"record_offset":21688,"parent_record_offset":21664,"ancestor_record_offset":21620,"inlinee":4776,"source_line":1888,"archive":"_pdb/B/modules/00104.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::strong_count
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":104,"record_offset":21664,"parent_record_offset":21640,"ancestor_record_offset":21620,"inlinee":11385,"source_line":1799,"archive":"_pdb/B/modules/00104.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":104,"record_offset":21720,"parent_record_offset":21688,"ancestor_record_offset":21620,"inlinee":4777,"source_line":425,"archive":"_pdb/B/modules/00104.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":104,"record_offset":21748,"parent_record_offset":21688,"ancestor_record_offset":21620,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00104.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::screenshot::write_image_with_callback_impl
// 记录形式: compiled_procedure; instances=1
// 源码行: 789
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":130,"record_offset":48308,"rva":3278496,"va":5371987616}
// 编译实例 RVAs: 0x3206a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$image..codecs..jpeg..encoder..JpegEncoder$LT$W$GT$$u20$as$u20$image..image..ImageEncoder$GT$::write_image
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-292378","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371987616,"raw_hex":"e8e689fbff","size":5,"soft_fail":false,"target_va":5371696096,"va":5371988469,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$tempfile..file..TempPath$u20$as$u20$core..ops..drop..Drop$GT$::drop
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t8793872","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371987616,"raw_hex":"e8102f8600","size":5,"soft_fail":false,"target_va":5380783824,"va":5371989947,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400ad810
//     owner=unresolved_namespace; count=1
//     target_aliases=34 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2571632","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371987616,"raw_hex":"e890c2d8ff","size":5,"soft_fail":false,"target_va":5369419792,"va":5371991419,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400b2280
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2550884","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371987616,"raw_hex":"e89c13d9ff","size":5,"soft_fail":false,"target_va":5369438848,"va":5371989727,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400f62c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2272608","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371987616,"raw_hex":"e8a052ddff","size":5,"soft_fail":false,"target_va":5369717440,"va":5371990043,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400f6330
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2272652","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371987616,"raw_hex":"e87452ddff","size":5,"soft_fail":false,"target_va":5369717552,"va":5371990199,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14014fff0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1902952","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371987616,"raw_hex":"e898f6e2ff","size":5,"soft_fail":false,"target_va":5370085360,"va":5371988307,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140312690
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-58853","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371987616,"raw_hex":"e81b1affff","size":5,"soft_fail":false,"target_va":5371930256,"va":5371989104,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405ceca0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2809343","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371987616,"raw_hex":"e8ffdd2a00","size":5,"soft_fail":false,"target_va":5374799008,"va":5371989660,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405d3020
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2825895","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371987616,"raw_hex":"e8a71e2b00","size":5,"soft_fail":false,"target_va":5374816288,"va":5371990388,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405ecc10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2929832","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371987616,"raw_hex":"e8a8b42c00","size":5,"soft_fail":false,"target_va":5374921744,"va":5371991907,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405fdd60
//     owner=unresolved_namespace; count=6
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3003531","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371987616,"raw_hex":"e88bd42d00","size":5,"soft_fail":false,"target_va":5374991712,"va":5371988176,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3093461","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371987616,"raw_hex":"e8d5332f00","size":5,"soft_fail":false,"target_va":5375081488,"va":5371988022,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=17
//     target_aliases=1 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3093339","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371987616,"raw_hex":"e85b332f00","size":5,"soft_fail":false,"target_va":5375081504,"va":5371988160,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067dd80
//     owner=unresolved_namespace; count=1
//     target_aliases=69 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3526414","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371987616,"raw_hex":"e80ecf3500","size":5,"soft_fail":false,"target_va":5375516032,"va":5371989613,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d76ed0
//     owner=unresolved_namespace; count=4
//     target_aliases=5 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10838882","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371987616,"raw_hex":"e86263a500","size":5,"soft_fail":false,"target_va":5382827728,"va":5371988841,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14289aa40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t39297930","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371987616,"raw_hex":"e88aa35702","size":5,"soft_fail":false,"target_va":5411285568,"va":5371987633,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428a2020
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t39327980","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371987616,"raw_hex":"e8ec185802","size":5,"soft_fail":false,"target_va":5411315744,"va":5371987759,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428a3890
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t39332651","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371987616,"raw_hex":"e82b2b5802","size":5,"soft_fail":false,"target_va":5411322000,"va":5371989344,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428a3f50
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t39334504","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371987616,"raw_hex":"e868325802","size":5,"soft_fail":false,"target_va":5411323728,"va":5371989219,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428a4000
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t39334986","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371987616,"raw_hex":"e84a345802","size":5,"soft_fail":false,"target_va":5411323904,"va":5371988913,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428a4220
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t39334150","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371987616,"raw_hex":"e806315802","size":5,"soft_fail":false,"target_va":5411324448,"va":5371990293,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: screenshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t39858770","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371987616,"raw_hex":"e852326002","size":5,"soft_fail":false,"target_va":5411847168,"va":5371988393,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t39550420","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371987616,"raw_hex":"e8d47d5b02","size":5,"soft_fail":false,"target_va":5411540992,"va":5371990567,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t10852217","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371987616,"raw_hex":"e87997a500","size":5,"soft_fail":false,"target_va":5382840208,"va":5371987986,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t10663198","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371987616,"raw_hex":"e81eb5a200","size":5,"soft_fail":false,"target_va":5382655024,"va":5371991821,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t39551190","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371987616,"raw_hex":"e8d6805b02","size":5,"soft_fail":false,"target_va":5411541776,"va":5371990581,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t39551445","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371987616,"raw_hex":"e8d5815b02","size":5,"soft_fail":false,"target_va":5411542064,"va":5371990614,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_end_index_len_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t39552563","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371987616,"raw_hex":"e833865b02","size":5,"soft_fail":false,"target_va":5411543104,"va":5371990536,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] image::codecs::jpeg::encoder::JpegEncoder$LT$W$GT$::new_with_quality
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"image","version":"0.24.9","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-294085","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371987616,"raw_hex":"e83b83fbff","size":5,"soft_fail":false,"target_va":5371694288,"va":5371988368,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] image::image::ImageFormat::from_path::inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"image","version":"0.24.9","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t8927908","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371987616,"raw_hex":"e8a43a8800","size":5,"soft_fail":false,"target_va":5380917456,"va":5371989543,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t39501879","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371987616,"raw_hex":"e837c05a02","size":5,"soft_fail":false,"target_va":5411492368,"va":5371990484,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t39504289","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371987616,"raw_hex":"e8a1c95a02","size":5,"soft_fail":false,"target_va":5411494800,"va":5371990506,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::acquire_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t9835033","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371987616,"raw_hex":"e819129600","size":5,"soft_fail":false,"target_va":5381823600,"va":5371988562,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t9835379","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371987616,"raw_hex":"e873139600","size":5,"soft_fail":false,"target_va":5381824016,"va":5371988632,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::backtrace::Backtrace::capture
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t10705338","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371987616,"raw_hex":"e8ba59a300","size":5,"soft_fail":false,"target_va":5382694656,"va":5371989313,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::DirBuilder::_create
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t10720686","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371987616,"raw_hex":"e8ae95a300","size":5,"soft_fail":false,"target_va":5382709584,"va":5371988893,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::extension
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t10754502","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371987616,"raw_hex":"e8c619a400","size":5,"soft_fail":false,"target_va":5382743968,"va":5371989461,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::parent
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t10752339","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371987616,"raw_hex":"e85311a400","size":5,"soft_fail":false,"target_va":5382741200,"va":5371988856,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t*24(%rdi)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5371987616,"raw_hex":"ff5718","size":3,"soft_fail":false,"va":5371988130,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":130,"record_offset":54516,"parent_record_offset":48308,"ancestor_record_offset":48308,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":130,"record_offset":54600,"parent_record_offset":48308,"ancestor_record_offset":48308,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":130,"record_offset":56120,"parent_record_offset":48308,"ancestor_record_offset":48308,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::screenshot::ScreenshotCapturedCallback::call
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/screenshot.rs
//     evidence={"module":130,"record_offset":55128,"parent_record_offset":48308,"ancestor_record_offset":48308,"inlinee":12555,"source_line":110,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::screenshot::write_image_impl
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/screenshot.rs
//     evidence={"module":130,"record_offset":48464,"parent_record_offset":48308,"ancestor_record_offset":48308,"inlinee":12560,"source_line":815,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":130,"record_offset":54764,"parent_record_offset":54740,"ancestor_record_offset":48308,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":130,"record_offset":54788,"parent_record_offset":54764,"ancestor_record_offset":48308,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":130,"record_offset":49560,"parent_record_offset":49536,"ancestor_record_offset":48308,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":130,"record_offset":54708,"parent_record_offset":54672,"ancestor_record_offset":48308,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":130,"record_offset":54740,"parent_record_offset":54708,"ancestor_record_offset":48308,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":130,"record_offset":49536,"parent_record_offset":49480,"ancestor_record_offset":48308,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::convert::impl$19::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed\convert.rs
//     evidence={"module":130,"record_offset":54636,"parent_record_offset":54600,"ancestor_record_offset":48308,"inlinee":9745,"source_line":611,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":130,"record_offset":54672,"parent_record_offset":54636,"ancestor_record_offset":48308,"inlinee":4280,"source_line":273,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$28::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":130,"record_offset":55304,"parent_record_offset":55128,"ancestor_record_offset":48308,"inlinee":7605,"source_line":1975,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":130,"record_offset":52528,"parent_record_offset":52500,"ancestor_record_offset":48308,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":130,"record_offset":54564,"parent_record_offset":54540,"ancestor_record_offset":48308,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":130,"record_offset":49508,"parent_record_offset":49480,"ancestor_record_offset":48308,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":130,"record_offset":49480,"parent_record_offset":49456,"ancestor_record_offset":48308,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":130,"record_offset":49456,"parent_record_offset":49432,"ancestor_record_offset":48308,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":130,"record_offset":49200,"parent_record_offset":48464,"ancestor_record_offset":48308,"inlinee":5345,"source_line":2205,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":130,"record_offset":49108,"parent_record_offset":48464,"ancestor_record_offset":48308,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::from_display
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/error.rs
//     evidence={"module":130,"record_offset":50464,"parent_record_offset":50420,"ancestor_record_offset":48308,"inlinee":12565,"source_line":143,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::impl$1::context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/context.rs
//     evidence={"module":130,"record_offset":50420,"parent_record_offset":48464,"ancestor_record_offset":48308,"inlinee":12564,"source_line":91,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":130,"record_offset":55360,"parent_record_offset":55332,"ancestor_record_offset":48308,"inlinee":5033,"source_line":219,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":130,"record_offset":51296,"parent_record_offset":51268,"ancestor_record_offset":48308,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":130,"record_offset":51436,"parent_record_offset":51412,"ancestor_record_offset":48308,"inlinee":5126,"source_line":703,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":130,"record_offset":49072,"parent_record_offset":49052,"ancestor_record_offset":48308,"inlinee":5501,"source_line":600,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":130,"record_offset":55200,"parent_record_offset":55176,"ancestor_record_offset":48308,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":130,"record_offset":55384,"parent_record_offset":55360,"ancestor_record_offset":48308,"inlinee":5034,"source_line":393,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":130,"record_offset":48816,"parent_record_offset":48796,"ancestor_record_offset":48308,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":130,"record_offset":48840,"parent_record_offset":48816,"ancestor_record_offset":48308,"inlinee":4619,"source_line":2579,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":130,"record_offset":48772,"parent_record_offset":48748,"ancestor_record_offset":48308,"inlinee":4518,"source_line":1447,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":130,"record_offset":48992,"parent_record_offset":48720,"ancestor_record_offset":48308,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":130,"record_offset":54540,"parent_record_offset":54516,"ancestor_record_offset":48308,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":130,"record_offset":49028,"parent_record_offset":48464,"ancestor_record_offset":48308,"inlinee":5145,"source_line":1277,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::take
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":130,"record_offset":55176,"parent_record_offset":55128,"ancestor_record_offset":48308,"inlinee":4908,"source_line":1726,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":130,"record_offset":52004,"parent_record_offset":51712,"ancestor_record_offset":48308,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":130,"record_offset":51268,"parent_record_offset":48464,"ancestor_record_offset":48308,"inlinee":5835,"source_line":2186,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=50
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":130,"record_offset":49384,"parent_record_offset":48464,"ancestor_record_offset":48308,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":130,"record_offset":54816,"parent_record_offset":54788,"ancestor_record_offset":48308,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":130,"record_offset":55224,"parent_record_offset":55200,"ancestor_record_offset":48308,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":130,"record_offset":49164,"parent_record_offset":48464,"ancestor_record_offset":48308,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":130,"record_offset":50252,"parent_record_offset":48464,"ancestor_record_offset":48308,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":130,"record_offset":51344,"parent_record_offset":51320,"ancestor_record_offset":48308,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":130,"record_offset":51368,"parent_record_offset":51344,"ancestor_record_offset":48308,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":130,"record_offset":52040,"parent_record_offset":51712,"ancestor_record_offset":48308,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":130,"record_offset":52096,"parent_record_offset":52068,"ancestor_record_offset":48308,"inlinee":4610,"source_line":431,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$6::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":130,"record_offset":52068,"parent_record_offset":52040,"ancestor_record_offset":48308,"inlinee":9466,"source_line":521,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":130,"record_offset":49732,"parent_record_offset":49668,"ancestor_record_offset":48308,"inlinee":4945,"source_line":2871,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":130,"record_offset":49292,"parent_record_offset":49256,"ancestor_record_offset":48308,"inlinee":10004,"source_line":2920,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":130,"record_offset":49756,"parent_record_offset":49732,"ancestor_record_offset":48308,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":130,"record_offset":49316,"parent_record_offset":49292,"ancestor_record_offset":48308,"inlinee":5825,"source_line":3828,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::ImageBuffer::check_image_fits
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/buffer.rs
//     evidence={"module":130,"record_offset":48720,"parent_record_offset":48696,"ancestor_record_offset":48308,"inlinee":7814,"source_line":802,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::ImageBuffer::from_raw
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/buffer.rs
//     evidence={"module":130,"record_offset":48696,"parent_record_offset":48464,"ancestor_record_offset":48308,"inlinee":7813,"source_line":674,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::ImageBuffer::image_buffer_len
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/buffer.rs
//     evidence={"module":130,"record_offset":48748,"parent_record_offset":48720,"ancestor_record_offset":48308,"inlinee":7815,"source_line":807,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::ImageBuffer::inner_pixels
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/buffer.rs
//     evidence={"module":130,"record_offset":51712,"parent_record_offset":51672,"ancestor_record_offset":48308,"inlinee":9503,"source_line":713,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::ImageBuffer::write_to
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/buffer.rs
//     evidence={"module":130,"record_offset":51672,"parent_record_offset":48464,"ancestor_record_offset":48308,"inlinee":9522,"source_line":1049,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::impl$34::image_buffer_len::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/buffer.rs
//     evidence={"module":130,"record_offset":48796,"parent_record_offset":48772,"ancestor_record_offset":48308,"inlinee":7817,"source_line":809,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::impl$34::image_buffer_len::closure$1
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/buffer.rs
//     evidence={"module":130,"record_offset":48904,"parent_record_offset":48880,"ancestor_record_offset":48308,"inlinee":7818,"source_line":810,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::impl$41::deref
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/buffer.rs
//     evidence={"module":130,"record_offset":49136,"parent_record_offset":48464,"ancestor_record_offset":48308,"inlinee":9524,"source_line":1110,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] image::dynimage::write_buffer_with_format
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/dynimage.rs
//     evidence={"module":130,"record_offset":52144,"parent_record_offset":51672,"ancestor_record_offset":48308,"inlinee":9510,"source_line":1156,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] image::image::ImageFormat::from_path
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/image.rs
//     evidence={"module":130,"record_offset":51412,"parent_record_offset":48464,"ancestor_record_offset":48308,"inlinee":12567,"source_line":126,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":130,"record_offset":49228,"parent_record_offset":48464,"ancestor_record_offset":48308,"inlinee":10002,"source_line":213,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":130,"record_offset":49640,"parent_record_offset":49612,"ancestor_record_offset":48308,"inlinee":9996,"source_line":653,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":130,"record_offset":49256,"parent_record_offset":49228,"ancestor_record_offset":48308,"inlinee":10003,"source_line":66,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":130,"record_offset":49668,"parent_record_offset":49640,"ancestor_record_offset":48308,"inlinee":4941,"source_line":100,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":130,"record_offset":49348,"parent_record_offset":49256,"ancestor_record_offset":48308,"inlinee":5827,"source_line":1104,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":130,"record_offset":49704,"parent_record_offset":49668,"ancestor_record_offset":48308,"inlinee":4943,"source_line":1120,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$44::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":130,"record_offset":51320,"parent_record_offset":51296,"ancestor_record_offset":48308,"inlinee":12384,"source_line":1427,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$59::as_ref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":130,"record_offset":50076,"parent_record_offset":50052,"ancestor_record_offset":48308,"inlinee":5134,"source_line":1639,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":130,"record_offset":50124,"parent_record_offset":50100,"ancestor_record_offset":48308,"inlinee":5138,"source_line":599,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":130,"record_offset":50100,"parent_record_offset":50076,"ancestor_record_offset":48308,"inlinee":5136,"source_line":617,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::DirBuilder::create
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":130,"record_offset":50220,"parent_record_offset":50168,"ancestor_record_offset":48308,"inlinee":8210,"source_line":3032,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::DirBuilder::recursive
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":130,"record_offset":50192,"parent_record_offset":50168,"ancestor_record_offset":48308,"inlinee":8209,"source_line":3008,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::create_dir_all
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":130,"record_offset":50168,"parent_record_offset":48464,"ancestor_record_offset":48308,"inlinee":8207,"source_line":2766,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] std::io::buffered::bufwriter::impl$6::flush
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\buffered\bufwriter.rs
//     evidence={"module":130,"record_offset":53092,"parent_record_offset":48464,"ancestor_record_offset":48308,"inlinee":8698,"source_line":651,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] std::os::windows::io::handle::impl$25::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\os\windows\io\handle.rs
//     evidence={"module":130,"record_offset":52344,"parent_record_offset":52320,"ancestor_record_offset":48308,"inlinee":5147,"source_line":386,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":130,"record_offset":50052,"parent_record_offset":50028,"ancestor_record_offset":48308,"inlinee":5132,"source_line":2182,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$45::deref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":130,"record_offset":50028,"parent_record_offset":48464,"ancestor_record_offset":48308,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$83::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":130,"record_offset":51460,"parent_record_offset":51436,"ancestor_record_offset":48308,"inlinee":5128,"source_line":3362,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] tempfile::Builder::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"tempfile","version":"3.20.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tempfile-3.20.0/src/lib.rs
//     evidence={"module":130,"record_offset":50520,"parent_record_offset":50496,"ancestor_record_offset":48308,"inlinee":7537,"source_line":314,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] tempfile::Builder::tempfile_in
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"tempfile","version":"3.20.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tempfile-3.20.0/src/lib.rs
//     evidence={"module":130,"record_offset":50576,"parent_record_offset":50496,"ancestor_record_offset":48308,"inlinee":5219,"source_line":558,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] tempfile::file::NamedTempFile::new_in
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"tempfile","version":"3.20.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tempfile-3.20.0/src/file/mod.rs
//     evidence={"module":130,"record_offset":50496,"parent_record_offset":48464,"ancestor_record_offset":48308,"inlinee":7564,"source_line":574,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] tempfile::impl$0::default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"tempfile","version":"3.20.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tempfile-3.20.0/src/lib.rs
//     evidence={"module":130,"record_offset":50544,"parent_record_offset":50520,"ancestor_record_offset":48308,"inlinee":7539,"source_line":235,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::screenshot::write_image_impl::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/screenshot.rs
//     evidence={"module":130,"record_offset":49052,"parent_record_offset":49028,"ancestor_record_offset":48308,"inlinee":12562,"source_line":821,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::screenshot::write_image_impl
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 815
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3110
// 首条位置: {"module":130,"inlinee":12560}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":130,"record_offset":49200,"parent_record_offset":48464,"ancestor_record_offset":48464,"inlinee":5345,"source_line":2205,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":130,"record_offset":49108,"parent_record_offset":48464,"ancestor_record_offset":48464,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_direct] anyhow::context::impl$1::context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/context.rs
//     evidence={"module":130,"record_offset":50420,"parent_record_offset":48464,"ancestor_record_offset":48464,"inlinee":12564,"source_line":91,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::ok_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":130,"record_offset":49028,"parent_record_offset":48464,"ancestor_record_offset":48464,"inlinee":5145,"source_line":1277,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_direct] core::option::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":130,"record_offset":51268,"parent_record_offset":48464,"ancestor_record_offset":48464,"inlinee":5835,"source_line":2186,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":130,"record_offset":49384,"parent_record_offset":48464,"ancestor_record_offset":48464,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":130,"record_offset":49164,"parent_record_offset":48464,"ancestor_record_offset":48464,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":130,"record_offset":50252,"parent_record_offset":48464,"ancestor_record_offset":48464,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_direct] image::buffer_::ImageBuffer::from_raw
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/buffer.rs
//     evidence={"module":130,"record_offset":48696,"parent_record_offset":48464,"ancestor_record_offset":48464,"inlinee":7813,"source_line":674,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_direct] image::buffer_::ImageBuffer::write_to
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/buffer.rs
//     evidence={"module":130,"record_offset":51672,"parent_record_offset":48464,"ancestor_record_offset":48464,"inlinee":9522,"source_line":1049,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_direct] image::buffer_::impl$41::deref
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/buffer.rs
//     evidence={"module":130,"record_offset":49136,"parent_record_offset":48464,"ancestor_record_offset":48464,"inlinee":9524,"source_line":1110,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_direct] image::image::ImageFormat::from_path
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/image.rs
//     evidence={"module":130,"record_offset":51412,"parent_record_offset":48464,"ancestor_record_offset":48464,"inlinee":12567,"source_line":126,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":130,"record_offset":49228,"parent_record_offset":48464,"ancestor_record_offset":48464,"inlinee":10002,"source_line":213,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_direct] std::fs::create_dir_all
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":130,"record_offset":50168,"parent_record_offset":48464,"ancestor_record_offset":48464,"inlinee":8207,"source_line":2766,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_direct] std::io::buffered::bufwriter::impl$6::flush
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\buffered\bufwriter.rs
//     evidence={"module":130,"record_offset":53092,"parent_record_offset":48464,"ancestor_record_offset":48464,"inlinee":8698,"source_line":651,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$45::deref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":130,"record_offset":50028,"parent_record_offset":48464,"ancestor_record_offset":48464,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_direct] tempfile::file::NamedTempFile::new_in
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"tempfile","version":"3.20.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tempfile-3.20.0/src/file/mod.rs
//     evidence={"module":130,"record_offset":50496,"parent_record_offset":48464,"ancestor_record_offset":48464,"inlinee":7564,"source_line":574,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":130,"record_offset":49560,"parent_record_offset":49536,"ancestor_record_offset":48464,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":130,"record_offset":49536,"parent_record_offset":49480,"ancestor_record_offset":48464,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":130,"record_offset":52528,"parent_record_offset":52500,"ancestor_record_offset":48464,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":130,"record_offset":49508,"parent_record_offset":49480,"ancestor_record_offset":48464,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":130,"record_offset":49480,"parent_record_offset":49456,"ancestor_record_offset":48464,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":130,"record_offset":49456,"parent_record_offset":49432,"ancestor_record_offset":48464,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::from_display
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/error.rs
//     evidence={"module":130,"record_offset":50464,"parent_record_offset":50420,"ancestor_record_offset":48464,"inlinee":12565,"source_line":143,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":130,"record_offset":51296,"parent_record_offset":51268,"ancestor_record_offset":48464,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":130,"record_offset":51436,"parent_record_offset":51412,"ancestor_record_offset":48464,"inlinee":5126,"source_line":703,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":130,"record_offset":49072,"parent_record_offset":49052,"ancestor_record_offset":48464,"inlinee":5501,"source_line":600,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":130,"record_offset":48816,"parent_record_offset":48796,"ancestor_record_offset":48464,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":130,"record_offset":48840,"parent_record_offset":48816,"ancestor_record_offset":48464,"inlinee":4619,"source_line":2579,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":130,"record_offset":48772,"parent_record_offset":48748,"ancestor_record_offset":48464,"inlinee":4518,"source_line":1447,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":130,"record_offset":48992,"parent_record_offset":48720,"ancestor_record_offset":48464,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":130,"record_offset":52004,"parent_record_offset":51712,"ancestor_record_offset":48464,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=32
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":130,"record_offset":49408,"parent_record_offset":49384,"ancestor_record_offset":48464,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":130,"record_offset":51344,"parent_record_offset":51320,"ancestor_record_offset":48464,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":130,"record_offset":51368,"parent_record_offset":51344,"ancestor_record_offset":48464,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":130,"record_offset":52040,"parent_record_offset":51712,"ancestor_record_offset":48464,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":130,"record_offset":52096,"parent_record_offset":52068,"ancestor_record_offset":48464,"inlinee":4610,"source_line":431,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$6::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":130,"record_offset":52068,"parent_record_offset":52040,"ancestor_record_offset":48464,"inlinee":9466,"source_line":521,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":130,"record_offset":49732,"parent_record_offset":49668,"ancestor_record_offset":48464,"inlinee":4945,"source_line":2871,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":130,"record_offset":49292,"parent_record_offset":49256,"ancestor_record_offset":48464,"inlinee":10004,"source_line":2920,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":130,"record_offset":49756,"parent_record_offset":49732,"ancestor_record_offset":48464,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":130,"record_offset":49316,"parent_record_offset":49292,"ancestor_record_offset":48464,"inlinee":5825,"source_line":3828,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::ImageBuffer::check_image_fits
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/buffer.rs
//     evidence={"module":130,"record_offset":48720,"parent_record_offset":48696,"ancestor_record_offset":48464,"inlinee":7814,"source_line":802,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::ImageBuffer::image_buffer_len
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/buffer.rs
//     evidence={"module":130,"record_offset":48748,"parent_record_offset":48720,"ancestor_record_offset":48464,"inlinee":7815,"source_line":807,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::ImageBuffer::inner_pixels
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/buffer.rs
//     evidence={"module":130,"record_offset":51712,"parent_record_offset":51672,"ancestor_record_offset":48464,"inlinee":9503,"source_line":713,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::impl$34::image_buffer_len::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/buffer.rs
//     evidence={"module":130,"record_offset":48796,"parent_record_offset":48772,"ancestor_record_offset":48464,"inlinee":7817,"source_line":809,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::impl$34::image_buffer_len::closure$1
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/buffer.rs
//     evidence={"module":130,"record_offset":48904,"parent_record_offset":48880,"ancestor_record_offset":48464,"inlinee":7818,"source_line":810,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] image::dynimage::write_buffer_with_format
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/dynimage.rs
//     evidence={"module":130,"record_offset":52144,"parent_record_offset":51672,"ancestor_record_offset":48464,"inlinee":9510,"source_line":1156,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":130,"record_offset":49640,"parent_record_offset":49612,"ancestor_record_offset":48464,"inlinee":9996,"source_line":653,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":130,"record_offset":49256,"parent_record_offset":49228,"ancestor_record_offset":48464,"inlinee":10003,"source_line":66,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":130,"record_offset":49668,"parent_record_offset":49640,"ancestor_record_offset":48464,"inlinee":4941,"source_line":100,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":130,"record_offset":49348,"parent_record_offset":49256,"ancestor_record_offset":48464,"inlinee":5827,"source_line":1104,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":130,"record_offset":49704,"parent_record_offset":49668,"ancestor_record_offset":48464,"inlinee":4943,"source_line":1120,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$44::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":130,"record_offset":51320,"parent_record_offset":51296,"ancestor_record_offset":48464,"inlinee":12384,"source_line":1427,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$59::as_ref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":130,"record_offset":50076,"parent_record_offset":50052,"ancestor_record_offset":48464,"inlinee":5134,"source_line":1639,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":130,"record_offset":50124,"parent_record_offset":50100,"ancestor_record_offset":48464,"inlinee":5138,"source_line":599,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":130,"record_offset":50100,"parent_record_offset":50076,"ancestor_record_offset":48464,"inlinee":5136,"source_line":617,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::DirBuilder::create
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":130,"record_offset":50220,"parent_record_offset":50168,"ancestor_record_offset":48464,"inlinee":8210,"source_line":3032,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::DirBuilder::recursive
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":130,"record_offset":50192,"parent_record_offset":50168,"ancestor_record_offset":48464,"inlinee":8209,"source_line":3008,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] std::os::windows::io::handle::impl$25::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\os\windows\io\handle.rs
//     evidence={"module":130,"record_offset":52344,"parent_record_offset":52320,"ancestor_record_offset":48464,"inlinee":5147,"source_line":386,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":130,"record_offset":50052,"parent_record_offset":50028,"ancestor_record_offset":48464,"inlinee":5132,"source_line":2182,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$45::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":130,"record_offset":51484,"parent_record_offset":51460,"ancestor_record_offset":48464,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$83::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":130,"record_offset":51460,"parent_record_offset":51436,"ancestor_record_offset":48464,"inlinee":5128,"source_line":3362,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] tempfile::Builder::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"tempfile","version":"3.20.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tempfile-3.20.0/src/lib.rs
//     evidence={"module":130,"record_offset":50520,"parent_record_offset":50496,"ancestor_record_offset":48464,"inlinee":7537,"source_line":314,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] tempfile::Builder::tempfile_in
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"tempfile","version":"3.20.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tempfile-3.20.0/src/lib.rs
//     evidence={"module":130,"record_offset":50576,"parent_record_offset":50496,"ancestor_record_offset":48464,"inlinee":5219,"source_line":558,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] tempfile::impl$0::default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"tempfile","version":"3.20.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tempfile-3.20.0/src/lib.rs
//     evidence={"module":130,"record_offset":50544,"parent_record_offset":50520,"ancestor_record_offset":48464,"inlinee":7539,"source_line":235,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::screenshot::write_image_impl::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/screenshot.rs
//     evidence={"module":130,"record_offset":49052,"parent_record_offset":49028,"ancestor_record_offset":48464,"inlinee":12562,"source_line":821,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::screenshot::write_image_impl::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 821
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3112
// 首条位置: {"module":130,"inlinee":12562}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":130,"record_offset":49072,"parent_record_offset":49052,"ancestor_record_offset":49052,"inlinee":5501,"source_line":600,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::screenshot::progressive_blur_downsample
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 874
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3103
// 首条位置: {"module":130,"inlinee":12547}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":130,"record_offset":24328,"parent_record_offset":24024,"ancestor_record_offset":24024,"inlinee":4974,"source_line":261,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":130,"record_offset":24472,"parent_record_offset":24024,"ancestor_record_offset":24024,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":130,"record_offset":24144,"parent_record_offset":24024,"ancestor_record_offset":24024,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":130,"record_offset":24556,"parent_record_offset":24024,"ancestor_record_offset":24024,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::ComputePass::write_texture
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":130,"record_offset":24428,"parent_record_offset":24024,"ancestor_record_offset":24024,"inlinee":12552,"source_line":171,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::impl$3::read_texture
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":130,"record_offset":24384,"parent_record_offset":24024,"ancestor_record_offset":24024,"inlinee":12551,"source_line":704,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::recorder::RenderGraphRecorder::create
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/recorder.rs
//     evidence={"module":130,"record_offset":24204,"parent_record_offset":24024,"ancestor_record_offset":24024,"inlinee":9694,"source_line":34,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::resource::Handle::desc
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource.rs
//     evidence={"module":130,"record_offset":24176,"parent_record_offset":24024,"ancestor_record_offset":24024,"inlinee":8273,"source_line":315,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":130,"record_offset":24520,"parent_record_offset":24496,"ancestor_record_offset":24024,"inlinee":4690,"source_line":1824,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":130,"record_offset":24268,"parent_record_offset":24204,"ancestor_record_offset":24024,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":130,"record_offset":24496,"parent_record_offset":24472,"ancestor_record_offset":24024,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::texture::impl$24::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/texture.rs
//     evidence={"module":130,"record_offset":24352,"parent_record_offset":24328,"ancestor_record_offset":24024,"inlinee":12549,"source_line":49,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::texture::impl$26::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/texture.rs
//     evidence={"module":130,"record_offset":24240,"parent_record_offset":24204,"ancestor_record_offset":24024,"inlinee":9664,"source_line":67,"archive":"_pdb/B/modules/00130.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::util::impl$0::same
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/util.rs
//     evidence={"module":130,"record_offset":24296,"parent_record_offset":24204,"ancestor_record_offset":24024,"inlinee":9666,"source_line":8,"archive":"_pdb/B/modules/00130.jsonl.gz"}
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
// 完整结构及原始类型记录: screenshot.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tiny_glade::screenshot::ActiveScreenshotRequest
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 5
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec::retain<tiny_glade::screenshot::ActiveScreenshotRequest,alloc::alloc::Global,tiny_glade::screenshot::finish_screenshot_capture::closure_env$0>", "constructor": "alloc::vec::Vec::retain", "arguments": ["tiny_glade::screenshot::ActiveScreenshotRequest", "alloc::alloc::Global", "tiny_glade::screenshot::finish_screenshot_capture::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<tiny_glade::screenshot::ActiveScreenshotRequest,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["tiny_glade::screenshot::ActiveScreenshotRequest", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "lock_api::mutex::MutexGuard<parking_lot::raw_mutex::RawMutex,alloc::vec::Vec<tiny_glade::screenshot::ActiveScreenshotRequest,alloc::alloc::Global> >", "constructor": "lock_api::mutex::MutexGuard", "arguments": ["parking_lot::raw_mutex::RawMutex", "alloc::vec::Vec<tiny_glade::screenshot::ActiveScreenshotRequest,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<lock_api::mutex::MutexGuard<parking_lot::raw_mutex::RawMutex,alloc::vec::Vec<tiny_glade::screenshot::ActiveScreenshotRequest,alloc::alloc::Global> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["lock_api::mutex::MutexGuard<parking_lot::raw_mutex::RawMutex,alloc::vec::Vec<tiny_glade::screenshot::ActiveScreenshotRequest,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::impl$20::extend_trusted::closure_env$0<tiny_glade::screenshot::ActiveScreenshotRequest,alloc::alloc::Global,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tiny_glade::screenshot::ScreenshotRequest,alloc::alloc::Global>,tiny_glade::screenshot::gather_screenshot_requests::closure_env$2> >", "constructor": "alloc::vec::impl$20::extend_trusted::closure_env$0", "arguments": ["tiny_glade::screenshot::ActiveScreenshotRequest", "alloc::alloc::Global", "core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tiny_glade::screenshot::ScreenshotRequest,alloc::alloc::Global>,tiny_glade::screenshot::gather_screenshot_requests::closure_env$2>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tiny_glade::screenshot::ActiveScreenshotRequest,alloc::vec::impl$20::extend_trusted::closure_env$0<tiny_glade::screenshot::ActiveScreenshotRequest,alloc::alloc::Global,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tiny_glade::screenshot::ScreenshotRequest,alloc::alloc::Global>,tiny_glade::screenshot::gather_screenshot_requests::closure_env$2> > >", "constructor": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0", "arguments": ["tiny_glade::screenshot::ActiveScreenshotRequest", "alloc::vec::impl$20::extend_trusted::closure_env$0<tiny_glade::screenshot::ActiveScreenshotRequest,alloc::alloc::Global,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tiny_glade::screenshot::ScreenshotRequest,alloc::alloc::Global>,tiny_glade::screenshot::gather_screenshot_requests::closure_env$2> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::impl$2::fold<tiny_glade::screenshot::ActiveScreenshotRequest,alloc::vec::into_iter::IntoIter<tiny_glade::screenshot::ScreenshotRequest,alloc::alloc::Global>,tiny_glade::screenshot::gather_screenshot_requests::closure_env$2,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tiny_glade::screenshot::ActiveScreenshotRequest,alloc::vec::impl$20::extend_trusted::closure_env$0<tiny_glade::screenshot::ActiveScreenshotRequest,alloc::alloc::Global,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tiny_glade::screenshot::ScreenshotRequest,alloc::alloc::Global>,tiny_glade::screenshot::gather_screenshot_requests::closure_env$2> > > >", "constructor": "core::iter::adapters::map::impl$2::fold", "arguments": ["tiny_glade::screenshot::ActiveScreenshotRequest", "alloc::vec::into_iter::IntoIter<tiny_glade::screenshot::ScreenshotRequest,alloc::alloc::Global>", "tiny_glade::screenshot::gather_screenshot_requests::closure_env$2", "tuple$<>", "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tiny_glade::screenshot::ActiveScreenshotRequest,alloc::vec::impl$20::extend_trusted::closure_env$0<tiny_glade::screenshot::ActiveScreenshotRequest,alloc::alloc::Global,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tiny_glade::screenshot::ScreenshotRequest,alloc::alloc::Global>,tiny_glade::screenshot::gather_screenshot_requests::closure_env$2> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::grow_one<tiny_glade::screenshot::ActiveScreenshotRequest,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::grow_one", "arguments": ["tiny_glade::screenshot::ActiveScreenshotRequest", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<tiny_glade::screenshot::ActiveScreenshotRequest>", "constructor": "core::ptr::drop_in_place", "arguments": ["tiny_glade::screenshot::ActiveScreenshotRequest"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tiny_glade::screenshot::BeautyShotState
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 32
// [PDB 泛型关系，非字段关系] {"type": "enum2$<tiny_glade::screenshot::BeautyShotState>", "constructor": "enum2$", "arguments": ["tiny_glade::screenshot::BeautyShotState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >", "constructor": "bevy_ecs::system::system_param::Local", "arguments": ["enum2$<tiny_glade::screenshot::BeautyShotState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)>", "constructor": "bevy_ecs::system::function_system::FunctionSystem", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)", "void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::system_type<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::system_type", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::dyn_hash<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::dyn_hash", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::as_dyn_eq<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::as_dyn_eq", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::dyn_clone<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::dyn_clone", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)> >", "constructor": "bevy_ecs::schedule::set::SystemTypeSet", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)> > >", "constructor": "core::any::impl$0::type_id", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$0::as_any<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)> > >", "constructor": "bevy_ecs::label::impl$0::as_any", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$0::dyn_eq<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)> > >", "constructor": "bevy_ecs::label::impl$0::dyn_eq", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$1::fmt<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)> >", "constructor": "bevy_ecs::schedule::set::impl$1::fmt", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::initialize<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::initialize", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::run_unsafe", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::get_last_run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::get_last_run", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::has_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::has_deferred", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::is_exclusive<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::is_exclusive", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::set_last_run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::set_last_run", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::apply_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::apply_deferred", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::queue_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::queue_deferred", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::component_access<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::component_access", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::check_change_tick<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::check_change_tick", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::validate_param_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::validate_param_unsafe", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::archetype_component_access<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::archetype_component_access", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::update_archetype_component_access<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::update_archetype_component_access", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::name<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::name", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::is_send<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::is_send", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)> >", "constructor": "bevy_ecs::system::schedule_system::InfallibleSystemWrapper", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::type_id<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)> > >", "constructor": "bevy_ecs::system::system::System::type_id", "arguments": ["bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)>", "constructor": "tuple$", "arguments": ["bevy_ecs::system::function_system::IsFunctionSystem", "void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)> >", "constructor": "tuple$", "arguments": ["bevy_ecs::schedule::config::Infallible", "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::config::IntoScheduleConfigs::in_set<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,enum2$<core::result::Result<tuple$<>,bevy_ecs::error::bevy_error::BevyError> > > > >,alloc::alloc::Global>,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)> >,country_core::systems::GameSystemSet>", "constructor": "bevy_ecs::schedule::config::IntoScheduleConfigs::in_set", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)", "alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,enum2$<core::result::Result<tuple$<>,bevy_ecs::error::bevy_error::BevyError> > > > >,alloc::alloc::Global>", "tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)> >", "country_core::systems::GameSystemSet"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::default_system_sets<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::default_system_sets", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::validate_param<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)> > >", "constructor": "bevy_ecs::system::system::System::validate_param", "arguments": ["bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)> > >", "constructor": "bevy_ecs::system::system::System::run_without_applying_deferred", "arguments": ["bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::run<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)> > >", "constructor": "bevy_ecs::system::system::System::run", "arguments": ["bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tiny_glade::screenshot::ScreenshotCapturedCallback
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 3; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<tiny_glade::screenshot::ScreenshotCapturedCallback>", "constructor": "core::ptr::drop_in_place", "arguments": ["tiny_glade::screenshot::ScreenshotCapturedCallback"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tiny_glade::screenshot::ScreenshotPlugin
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 9
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<tiny_glade::screenshot::ScreenshotPlugin>", "constructor": "core::any::impl$0::type_id", "arguments": ["tiny_glade::screenshot::ScreenshotPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "downcast_rs::impl$0::as_any_mut<tiny_glade::screenshot::ScreenshotPlugin>", "constructor": "downcast_rs::impl$0::as_any_mut", "arguments": ["tiny_glade::screenshot::ScreenshotPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "downcast_rs::impl$0::into_any_rc<tiny_glade::screenshot::ScreenshotPlugin>", "constructor": "downcast_rs::impl$0::into_any_rc", "arguments": ["tiny_glade::screenshot::ScreenshotPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "downcast_rs::impl$0::as_any<tiny_glade::screenshot::ScreenshotPlugin>", "constructor": "downcast_rs::impl$0::as_any", "arguments": ["tiny_glade::screenshot::ScreenshotPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::Plugin::name<tiny_glade::screenshot::ScreenshotPlugin>", "constructor": "bevy_app::plugin::Plugin::name", "arguments": ["tiny_glade::screenshot::ScreenshotPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::Plugin::ready<tiny_glade::screenshot::ScreenshotPlugin>", "constructor": "bevy_app::plugin::Plugin::ready", "arguments": ["tiny_glade::screenshot::ScreenshotPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::Plugin::finish<tiny_glade::screenshot::ScreenshotPlugin>", "constructor": "bevy_app::plugin::Plugin::finish", "arguments": ["tiny_glade::screenshot::ScreenshotPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::Plugin::is_unique<tiny_glade::screenshot::ScreenshotPlugin>", "constructor": "bevy_app::plugin::Plugin::is_unique", "arguments": ["tiny_glade::screenshot::ScreenshotPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::sealed::impl$15::add_to_app<bevy_app::plugin::sealed::PluginMarker,tiny_glade::run_game::ReflectionListingPlugin,bevy_app::plugin::sealed::PluginMarker,tiny_glade::game_loop::GameLoopPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::main_camera::MainCameraPlugin,bevy_app::plugin::sealed::PluginMarker,toolbar::Ui2dPlugin,bevy_app::plugin::sealed::PluginMarker,toolbar::mei::core::MeiPlugin,bevy_app::plugin::sealed::PluginMarker,toolbar::ToolbarPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::resources::session::SessionFlowPlugin,bevy_app::plugin::sealed::PluginMarker,toolbar::menu::MenuFlowPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::render_loop::painterly::PainterlyPlugin,bevy_app::plugin::sealed::PluginMarker,tiny_glade::screenshot::ScreenshotPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::render_loop::session_selection::SessionThumbnailsPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::render_loop::lut_gallery::LutGalleryPlugin,bevy_app::plugin::sealed::PluginMarker,tiny_glade::gui::settings::SettingsMenuPlugin>", "constructor": "bevy_app::plugin::sealed::impl$15::add_to_app", "arguments": ["bevy_app::plugin::sealed::PluginMarker", "tiny_glade::run_game::ReflectionListingPlugin", "bevy_app::plugin::sealed::PluginMarker", "tiny_glade::game_loop::GameLoopPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::main_camera::MainCameraPlugin", "bevy_app::plugin::sealed::PluginMarker", "toolbar::Ui2dPlugin", "bevy_app::plugin::sealed::PluginMarker", "toolbar::mei::core::MeiPlugin", "bevy_app::plugin::sealed::PluginMarker", "toolbar::ToolbarPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::resources::session::SessionFlowPlugin", "bevy_app::plugin::sealed::PluginMarker", "toolbar::menu::MenuFlowPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::render_loop::painterly::PainterlyPlugin", "bevy_app::plugin::sealed::PluginMarker", "tiny_glade::screenshot::ScreenshotPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::render_loop::session_selection::SessionThumbnailsPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::render_loop::lut_gallery::LutGalleryPlugin", "bevy_app::plugin::sealed::PluginMarker", "tiny_glade::gui::settings::SettingsMenuPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tiny_glade::screenshot::ScreenshotRequest
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 5
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::into_iter::IntoIter<tiny_glade::screenshot::ScreenshotRequest,alloc::alloc::Global>", "constructor": "alloc::vec::into_iter::IntoIter", "arguments": ["tiny_glade::screenshot::ScreenshotRequest", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tiny_glade::screenshot::ScreenshotRequest,alloc::alloc::Global>,tiny_glade::screenshot::gather_screenshot_requests::closure_env$2>", "constructor": "core::iter::adapters::map::Map", "arguments": ["alloc::vec::into_iter::IntoIter<tiny_glade::screenshot::ScreenshotRequest,alloc::alloc::Global>", "tiny_glade::screenshot::gather_screenshot_requests::closure_env$2"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::impl$20::extend_trusted::closure_env$0<tiny_glade::screenshot::ActiveScreenshotRequest,alloc::alloc::Global,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tiny_glade::screenshot::ScreenshotRequest,alloc::alloc::Global>,tiny_glade::screenshot::gather_screenshot_requests::closure_env$2> >", "constructor": "alloc::vec::impl$20::extend_trusted::closure_env$0", "arguments": ["tiny_glade::screenshot::ActiveScreenshotRequest", "alloc::alloc::Global", "core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tiny_glade::screenshot::ScreenshotRequest,alloc::alloc::Global>,tiny_glade::screenshot::gather_screenshot_requests::closure_env$2>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tiny_glade::screenshot::ActiveScreenshotRequest,alloc::vec::impl$20::extend_trusted::closure_env$0<tiny_glade::screenshot::ActiveScreenshotRequest,alloc::alloc::Global,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tiny_glade::screenshot::ScreenshotRequest,alloc::alloc::Global>,tiny_glade::screenshot::gather_screenshot_requests::closure_env$2> > >", "constructor": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0", "arguments": ["tiny_glade::screenshot::ActiveScreenshotRequest", "alloc::vec::impl$20::extend_trusted::closure_env$0<tiny_glade::screenshot::ActiveScreenshotRequest,alloc::alloc::Global,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tiny_glade::screenshot::ScreenshotRequest,alloc::alloc::Global>,tiny_glade::screenshot::gather_screenshot_requests::closure_env$2> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::impl$2::fold<tiny_glade::screenshot::ActiveScreenshotRequest,alloc::vec::into_iter::IntoIter<tiny_glade::screenshot::ScreenshotRequest,alloc::alloc::Global>,tiny_glade::screenshot::gather_screenshot_requests::closure_env$2,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tiny_glade::screenshot::ActiveScreenshotRequest,alloc::vec::impl$20::extend_trusted::closure_env$0<tiny_glade::screenshot::ActiveScreenshotRequest,alloc::alloc::Global,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tiny_glade::screenshot::ScreenshotRequest,alloc::alloc::Global>,tiny_glade::screenshot::gather_screenshot_requests::closure_env$2> > > >", "constructor": "core::iter::adapters::map::impl$2::fold", "arguments": ["tiny_glade::screenshot::ActiveScreenshotRequest", "alloc::vec::into_iter::IntoIter<tiny_glade::screenshot::ScreenshotRequest,alloc::alloc::Global>", "tiny_glade::screenshot::gather_screenshot_requests::closure_env$2", "tuple$<>", "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tiny_glade::screenshot::ActiveScreenshotRequest,alloc::vec::impl$20::extend_trusted::closure_env$0<tiny_glade::screenshot::ActiveScreenshotRequest,alloc::alloc::Global,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tiny_glade::screenshot::ScreenshotRequest,alloc::alloc::Global>,tiny_glade::screenshot::gather_screenshot_requests::closure_env$2> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::grow_one<tiny_glade::screenshot::ScreenshotRequest,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::grow_one", "arguments": ["tiny_glade::screenshot::ScreenshotRequest", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::into_iter::IntoIter<tiny_glade::screenshot::ScreenshotRequest,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::into_iter::IntoIter<tiny_glade::screenshot::ScreenshotRequest,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<tiny_glade::screenshot::ScreenshotRequest>", "constructor": "core::ptr::drop_in_place", "arguments": ["tiny_glade::screenshot::ScreenshotRequest"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<tiny_glade::screenshot::ScreenshotRequest,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["tiny_glade::screenshot::ScreenshotRequest", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<tiny_glade::screenshot::ScreenshotRequest,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<tiny_glade::screenshot::ScreenshotRequest,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tiny_glade::screenshot::ScreenshotSink
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 2
// [PDB 泛型关系，非字段关系] {"type": "enum2$<tiny_glade::screenshot::ScreenshotSink>", "constructor": "enum2$", "arguments": ["tiny_glade::screenshot::ScreenshotSink"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<tiny_glade::screenshot::ScreenshotSink> >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<tiny_glade::screenshot::ScreenshotSink>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: screenshot.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: screenshot.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 95; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] glam::f32::vec2::Vec2 (*)(glam::f32::vec3::Vec3)
// 候选参数类型: ["glam::f32::vec3::Vec3"]
// [文件级接口候选，具体函数归属 unknown] iced_core::rectangle::Rectangle<f32> (*)(ref$<enum2$<iced_graphics::primitive::Primitive<enum2$<iced_tiny_skia::primitive::Custom> > > >)
// 候选参数类型: ["ref$<enum2$<iced_graphics::primitive::Primitive<enum2$<iced_tiny_skia::primitive::Custom> > > >"]
// [文件级接口候选，具体函数归属 unknown] country_core::render::render_config::VideoMode (*)(ref$<winit::monitor::VideoMode>)
// 候选参数类型: ["ref$<winit::monitor::VideoMode>"]
// [文件级接口候选，具体函数归属 unknown] std::ffi::os_str::OsString (*)(ref$<alloc::string::String>)
// 候选参数类型: ["ref$<alloc::string::String>"]
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::event::reader::EventReader<country_core::resources::events::ShutdownGameCmd>)
// 候选参数类型: ["bevy_ecs::event::reader::EventReader<country_core::resources::events::ShutdownGameCmd>"]
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::events::ShutdownGameCmd>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>", "bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >", "bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>", "enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >", "enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >", "bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>", "bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >", "bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>", "bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>", "bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>", "bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>", "bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>", "tuple$<bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >", "bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>", "bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>", "bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>", "bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::StairsState>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::structural_graph::StairsStructuralGraph>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::visual::StairsVisualState>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>", "bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>", "bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::StairsState>", "bevy_ecs::change_detection::ResMut<country_core::resources::stairs::structural_graph::StairsStructuralGraph>", "bevy_ecs::change_detection::ResMut<country_core::resources::stairs::visual::StairsVisualState>", "bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>", "bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::StairsState>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::stairs::structural_graph::StairsStructuralGraph>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::stairs::visual::StairsVisualState>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: tiny_glade::screenshot::ScreenshotCapturedCallback::call
// 信息 ID: cb8ed5a930824ec5f6270cbf
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::screenshot::read_back_screenshot; module=130, inline_record=38412; 不是独立调用边。
// [PDB 内联宿主] tiny_glade::screenshot::write_image_with_callback_impl; module=130, inline_record=55128; 不是独立调用边。
// [PDB 内联宿主] tiny_glade::screenshot::write_image_with_callback_impl; module=130, inline_record=55628; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::screenshot::ScreenshotCapturedCallback::new
// 信息 ID: ae8e636aff04073e68a05dd9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::screenshot::gather_screenshot_requests; module=130, inline_record=12496; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::screenshot::add_captured_screenshots_to_steam_library
// 信息 ID: d80fcb40d102f741ca6ed0af
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::screenshot::gather_screenshot_requests; module=130, inline_record=8340; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::screenshot::add_captured_screenshots_to_steam_library::closure$0
// 信息 ID: ffba8bdff031e42a1a9ea85f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::screenshot::add_captured_screenshots_to_steam_library", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::screenshot::add_captured_screenshots_to_steam_library::closure_env$0 <- alloc::vec::Vec::retain_mut<enum2$<country_core::systems::steam::SteamScreenshotRequest>,alloc::alloc::Global,tiny_glade::screenshot::add_captured_screenshots_to_steam_library::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 1 条。
// [静态直接引用] alloc::vec::Vec::retain_mut<enum2$<country_core::systems::steam::SteamScreenshotRequest>,alloc::alloc::Global,tiny_glade::screenshot::add_captured_screenshots_to_steam_library::closure_env$0> @ 0x2b5553 -> 0x2c13d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] alloc::vec::Vec::retain_mut<enum2$<country_core::systems::steam::SteamScreenshotRequest>,alloc::alloc::Global,tiny_glade::screenshot::add_captured_screenshots_to_steam_library::closure_env$0> @ 0x2b55ee -> 0x2c13d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::screenshot::consume_steam_screenshot_request
// 信息 ID: 61301738f09a78d0bfca0abe
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::screenshot::gather_screenshot_requests; module=130, inline_record=5852; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::screenshot::consume_steam_screenshot_request::closure$0
// 信息 ID: 7b6dc496c4a0b6abd4e42fe6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::screenshot::consume_steam_screenshot_request", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::screenshot::gather_screenshot_requests; module=130, inline_record=6280; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::screenshot::finish_screenshot_capture
// 信息 ID: 8777a5d0fcfbcd50e3c5a8dc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::render @ 0x4e9f8e -> 0x31ea20 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::screenshot::finish_screenshot_capture::closure$0
// 信息 ID: 16088b541f9b4da3e6b78362
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::screenshot::finish_screenshot_capture", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::screenshot::finish_screenshot_capture::closure_env$0 <- alloc::vec::Vec::retain<tiny_glade::screenshot::ActiveScreenshotRequest,alloc::alloc::Global,tiny_glade::screenshot::finish_screenshot_capture::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::Vec::retain<tiny_glade::screenshot::ActiveScreenshotRequest,alloc::alloc::Global,tiny_glade::screenshot::finish_screenshot_capture::closure_env$0>; module=104, inline_record=19740; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::Vec::retain<tiny_glade::screenshot::ActiveScreenshotRequest,alloc::alloc::Global,tiny_glade::screenshot::finish_screenshot_capture::closure_env$0>; module=104, inline_record=20536; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::screenshot::gather_screenshot_requests
// 信息 ID: 6fabfd64e0b427a3e9b8567e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 989303e797fa2f7dc010cb86: void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)> > @ 0x2cd31a -> 0x31af10 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>),void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::systems::render_loop::menu_background::MenuBackgroundResources>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamScreenshotRequests> > >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::system::system_param::Local<enum2$<tiny_glade::screenshot::BeautyShotState> >,bevy_ecs::change_detection::ResMut<country_core::render::render_config::FrameRenderConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::grading_lut::LutGalleryState>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,tuple$<bevy_ecs::event::reader::EventReader<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<enum2$<country_core::resources::photo_mode::PhotoModeCmd> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>)> > > @ 0x47849d -> 0x31af10 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::screenshot::gather_screenshot_requests::closure$0
// 信息 ID: cdf6301a318f775ed9cd03c8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::screenshot::gather_screenshot_requests", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::screenshot::gather_screenshot_requests; module=130, inline_record=8300; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::screenshot::gather_screenshot_requests::closure$1
// 信息 ID: 459be2f9f57dafeac793aa51
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "tiny_glade::screenshot::gather_screenshot_requests", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::screenshot::gather_screenshot_requests::closure_env$1 <- core::ops::function::FnOnce::call_once<tiny_glade::screenshot::gather_screenshot_requests::closure_env$1,tuple$<enum2$<core::result::Result<array$<u32,2>,alloc::boxed::Box<dyn$<core::error::Error,core::marker::Send,core::marker::Sync>,alloc::alloc::Global> > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::screenshot::gather_screenshot_requests::closure_env$1 <- core::ptr::drop_in_place<tiny_glade::screenshot::gather_screenshot_requests::closure_env$1>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<tiny_glade::screenshot::gather_screenshot_requests::closure_env$1,tuple$<enum2$<core::result::Result<array$<u32,2>,alloc::boxed::Box<dyn$<core::error::Error,core::marker::Send,core::marker::Sync>,alloc::alloc::Global> > > > >; module=109, inline_record=84492; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::screenshot::handle_screenshot_opportunity
// 信息 ID: 91e8c611237a240b0771c0b8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 5 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::state::RenderGraphState::compile<rhapsody::rg::resource::ResourceExportHandle<rhapsody::raw::texture::Texture>,country_core::render::pass_builder::pass_resources::PassResources,tiny_glade::render_loop::compile_main_render_graph::closure_env$0> @ 0x163091 -> 0x31d140 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::state::RenderGraphState::compile<rhapsody::rg::resource::ResourceExportHandle<rhapsody::raw::texture::Texture>,country_core::render::pass_builder::pass_resources::PassResources,tiny_glade::render_loop::compile_main_render_graph::closure_env$0> @ 0x1630d3 -> 0x31d140 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::state::RenderGraphState::compile<rhapsody::rg::resource::ResourceExportHandle<rhapsody::raw::texture::Texture>,country_core::render::pass_builder::pass_resources::PassResources,tiny_glade::render_loop::compile_main_render_graph::closure_env$0> @ 0x163494 -> 0x31d140 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::state::RenderGraphState::compile<rhapsody::rg::resource::ResourceExportHandle<rhapsody::raw::texture::Texture>,country_core::render::pass_builder::pass_resources::PassResources,tiny_glade::render_loop::compile_main_render_graph::closure_env$0> @ 0x163531 -> 0x31d140 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::state::RenderGraphState::compile<rhapsody::rg::resource::ResourceExportHandle<rhapsody::raw::texture::Texture>,country_core::render::pass_builder::pass_resources::PassResources,tiny_glade::render_loop::compile_main_render_graph::closure_env$0> @ 0x1632fa -> 0x31d140 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::screenshot::handle_screenshot_opportunity::closure$0
// 信息 ID: 7626d2d0f442623621cb731a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::screenshot::handle_screenshot_opportunity", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::screenshot::handle_screenshot_opportunity::closure_env$0 <- rhapsody::rg::recorder::impl$1::render<tiny_glade::screenshot::handle_screenshot_opportunity::closure_env$0,country_core::render::pass_builder::pass_resources::PassResources>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::screenshot::handle_screenshot_opportunity::closure_env$0 <- rhapsody::rg::pass_builder::PassBuilder::render<country_core::render::pass_builder::pass_resources::PassResources,tiny_glade::screenshot::handle_screenshot_opportunity::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] rhapsody::rg::recorder::impl$1::render<tiny_glade::screenshot::handle_screenshot_opportunity::closure_env$0,country_core::render::pass_builder::pass_resources::PassResources>; module=109, inline_record=89392; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::screenshot::hurry_active_screenshots_on_shutdown
// 信息 ID: aa1b57c1726938a4bcd91024
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 154068c32af70375db82354b: void (*)(bevy_ecs::event::reader::EventReader<country_core::resources::events::ShutdownGameCmd>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::event::reader::EventReader<country_core::resources::events::ShutdownGameCmd>),void (*)(bevy_ecs::event::reader::EventReader<country_core::resources::events::ShutdownGameCmd>)> > @ 0x2cc875 -> 0x31eed0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::event::reader::EventReader<country_core::resources::events::ShutdownGameCmd>),void (*)(bevy_ecs::event::reader::EventReader<country_core::resources::events::ShutdownGameCmd>)> > > @ 0x44ee13 -> 0x31eed0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::screenshot::impl$0::build
// 信息 ID: d87d120ba4d909944ba1b362
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::screenshot::impl$10::clone
// 信息 ID: 04f8059c0d26237b1f1e1843
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::screenshot::read_back_screenshot; module=130, inline_record=42760; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::screenshot::impl$1::drop
// 信息 ID: 6e9a8921c53acd58ecc85c10
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::ptr::drop_in_place<tiny_glade::screenshot::ScreenshotCapturedCallback>; module=253, inline_record=1052240; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::screenshot::impl$6::eq
// 信息 ID: a4eef5e601c817b6974e3c3e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::screenshot::handle_screenshot_opportunity; module=130, inline_record=23816; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::screenshot::next_requested_screenshot_stage
// 信息 ID: 9326f8c1a8540c2b7c871f0d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::rg::state::RenderGraphState::compile<rhapsody::rg::resource::ResourceExportHandle<rhapsody::raw::texture::Texture>,country_core::render::pass_builder::pass_resources::PassResources,tiny_glade::render_loop::compile_main_render_graph::closure_env$0>; module=66, inline_record=17432; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::screenshot::progressive_blur_downsample
// 信息 ID: 352276f61cf58a4eb2ab566d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::screenshot::handle_screenshot_opportunity; module=130, inline_record=24024; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::screenshot::read_back_screenshot
// 信息 ID: 5a7ba14624399561b6a736a5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::screenshot::finish_screenshot_capture @ 0x31ece2 -> 0x31f2e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::screenshot::finish_screenshot_capture @ 0x31ecf5 -> 0x31f2e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::screenshot::write_image
// 信息 ID: edc0d35800c4f63581d55629
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::screenshot::read_back_screenshot; module=130, inline_record=38900; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::screenshot::write_image::closure$0
// 信息 ID: d3e2c51049ad86f33b85dd1a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::screenshot::write_image", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::screenshot::write_image::closure_env$0 <- std::sys::backtrace::__rust_begin_short_backtrace<tiny_glade::screenshot::write_image::closure_env$0,tuple$<> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::screenshot::write_image::closure_env$0 <- std::thread::Builder::spawn<tiny_glade::screenshot::write_image::closure_env$0,tuple$<> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::screenshot::write_image::closure_env$0 <- core::ops::function::FnOnce::call_once<std::thread::impl$0::spawn_unchecked_::closure_env$1<tiny_glade::screenshot::write_image::closure_env$0,tuple$<> >,tuple$<> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::screenshot::write_image::closure_env$0 <- core::ptr::drop_in_place<std::thread::impl$0::spawn_unchecked_::closure_env$1<tiny_glade::screenshot::write_image::closure_env$0,tuple$<> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::screenshot::write_image::closure_env$0 <- core::ptr::drop_in_place<tiny_glade::screenshot::write_image::closure_env$0>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::screenshot::write_image::closure_env$0 <- core::ptr::drop_in_place<std::thread::impl$0::spawn_unchecked_::MaybeDangling<tiny_glade::screenshot::write_image::closure_env$0> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 6 条。
// [PDB 内联宿主] std::sys::backtrace::__rust_begin_short_backtrace<tiny_glade::screenshot::write_image::closure_env$0,tuple$<> >; module=31, inline_record=16476; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::screenshot::write_image::closure$1
// 信息 ID: d726bbb80a55ee43026e8457
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "tiny_glade::screenshot::write_image", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::screenshot::write_image::closure_env$1 <- alloc::vec::Vec::retain<std::thread::JoinHandle<tuple$<> >,alloc::alloc::Global,tiny_glade::screenshot::write_image::closure_env$1>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::Vec::retain<std::thread::JoinHandle<tuple$<> >,alloc::alloc::Global,tiny_glade::screenshot::write_image::closure_env$1>; module=104, inline_record=21620; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::Vec::retain<std::thread::JoinHandle<tuple$<> >,alloc::alloc::Global,tiny_glade::screenshot::write_image::closure_env$1>; module=104, inline_record=22852; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::screenshot::write_image_impl
// 信息 ID: e86864bc0a75078fd7d2c920
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::screenshot::write_image_with_callback_impl; module=130, inline_record=48464; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::screenshot::write_image_impl::closure$0
// 信息 ID: bd312182b8cc18d5713e72c0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::screenshot::write_image_impl", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::screenshot::write_image_with_callback_impl; module=130, inline_record=49052; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::screenshot::write_image_with_callback_impl
// 信息 ID: d7bff4b45b800b167759ed45
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] std::sys::backtrace::__rust_begin_short_backtrace<tiny_glade::screenshot::write_image::closure_env$0,tuple$<> > @ 0xd7fa1 -> 0x3206a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::screenshot::read_back_screenshot @ 0x31f745 -> 0x3206a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===

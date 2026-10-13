// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/systems/backend/configwatch.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 38d7b3bb527e0314b48ea2141e529e476e70007b5a2e3e24d353bacb8c8d3cdc
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: configwatch.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 479
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.27"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::backend::configwatch::configwatch
// 记录形式: compiled_procedure; instances=1
// 源码行: 12
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":479,"record_offset":40676,"rva":8761792,"va":5377470912}
// 编译实例 RVAs: 0x85b1c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t5173306","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377470912,"raw_hex":"e83af04e00","size":5,"soft_fail":false,"target_va":5382644528,"va":5377471217,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140043000
//     owner=unresolved_namespace; count=1
//     target_aliases=30 candidates; complete list: configwatch.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8488023","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377470912,"raw_hex":"e8a97b7eff","size":5,"soft_fail":false,"target_va":5368983552,"va":5377471570,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405d2870
//     owner=unresolved_namespace; count=1
//     target_aliases=102 candidates; complete list: configwatch.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2657315","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377470912,"raw_hex":"e8dd73d7ff","size":5,"soft_fail":false,"target_va":5374814320,"va":5377471630,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: configwatch.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2389804","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377470912,"raw_hex":"e8d488dbff","size":5,"soft_fail":false,"target_va":5375081504,"va":5377471303,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067dd80
//     owner=unresolved_namespace; count=1
//     target_aliases=69 candidates; complete list: configwatch.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1955059","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377470912,"raw_hex":"e80d2be2ff","size":5,"soft_fail":false,"target_va":5375516032,"va":5377471086,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t34071384","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377470912,"raw_hex":"e858e30702","size":5,"soft_fail":false,"target_va":5411542880,"va":5377471491,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::color_library::update_color_library
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t918027","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377470912,"raw_hex":"e80b020e00","size":5,"soft_fail":false,"target_va":5378389296,"va":5377471264,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::panicking::panic_count::is_zero_slow_path
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t34067184","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377470912,"raw_hex":"e8f0d20702","size":5,"soft_fail":false,"target_va":5411538608,"va":5377471419,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::mutex::futex::Mutex::lock_contended
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t34067372","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377470912,"raw_hex":"e8acd30702","size":5,"soft_fail":false,"target_va":5411538768,"va":5377471391,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::mutex::futex::Mutex::wake
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t34067454","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377470912,"raw_hex":"e8fed30702","size":5,"soft_fail":false,"target_va":5411538960,"va":5377471501,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":479,"record_offset":40848,"parent_record_offset":40676,"ancestor_record_offset":40676,"inlinee":5345,"source_line":2205,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":479,"record_offset":41700,"parent_record_offset":40676,"ancestor_record_offset":40676,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":479,"record_offset":41492,"parent_record_offset":40676,"ancestor_record_offset":40676,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":479,"record_offset":41380,"parent_record_offset":40676,"ancestor_record_offset":40676,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":479,"record_offset":41760,"parent_record_offset":40676,"ancestor_record_offset":40676,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":479,"record_offset":41336,"parent_record_offset":40676,"ancestor_record_offset":40676,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":479,"record_offset":41524,"parent_record_offset":40676,"ancestor_record_offset":40676,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":479,"record_offset":41408,"parent_record_offset":40676,"ancestor_record_offset":40676,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_direct] std::sync::poison::mutex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\mutex.rs
//     evidence={"module":479,"record_offset":40876,"parent_record_offset":40676,"ancestor_record_offset":40676,"inlinee":6007,"source_line":435,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":479,"record_offset":41960,"parent_record_offset":41936,"ancestor_record_offset":40676,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":479,"record_offset":41936,"parent_record_offset":41880,"ancestor_record_offset":40676,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":479,"record_offset":41908,"parent_record_offset":41880,"ancestor_record_offset":40676,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":479,"record_offset":41880,"parent_record_offset":41856,"ancestor_record_offset":40676,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":479,"record_offset":41856,"parent_record_offset":41832,"ancestor_record_offset":40676,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":479,"record_offset":41728,"parent_record_offset":41700,"ancestor_record_offset":40676,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":479,"record_offset":41580,"parent_record_offset":41548,"ancestor_record_offset":40676,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":479,"record_offset":41608,"parent_record_offset":41548,"ancestor_record_offset":40676,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":479,"record_offset":41784,"parent_record_offset":41760,"ancestor_record_offset":40676,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":479,"record_offset":41256,"parent_record_offset":41228,"ancestor_record_offset":40676,"inlinee":5623,"source_line":623,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":479,"record_offset":42268,"parent_record_offset":42080,"ancestor_record_offset":40676,"inlinee":5912,"source_line":651,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":479,"record_offset":40944,"parent_record_offset":40912,"ancestor_record_offset":40676,"inlinee":4945,"source_line":2871,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":479,"record_offset":42360,"parent_record_offset":42328,"ancestor_record_offset":40676,"inlinee":5910,"source_line":2753,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":479,"record_offset":41160,"parent_record_offset":41128,"ancestor_record_offset":40676,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":479,"record_offset":40968,"parent_record_offset":40944,"ancestor_record_offset":40676,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":479,"record_offset":41184,"parent_record_offset":41160,"ancestor_record_offset":40676,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":479,"record_offset":42292,"parent_record_offset":42268,"ancestor_record_offset":40676,"inlinee":5913,"source_line":3712,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":479,"record_offset":42384,"parent_record_offset":42360,"ancestor_record_offset":40676,"inlinee":5911,"source_line":3743,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":479,"record_offset":41636,"parent_record_offset":41548,"ancestor_record_offset":40676,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":479,"record_offset":41548,"parent_record_offset":41524,"ancestor_record_offset":40676,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":479,"record_offset":41660,"parent_record_offset":41636,"ancestor_record_offset":40676,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\panicking.rs
//     evidence={"module":479,"record_offset":41128,"parent_record_offset":41100,"ancestor_record_offset":40676,"inlinee":5907,"source_line":469,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\panicking.rs
//     evidence={"module":479,"record_offset":41100,"parent_record_offset":41072,"ancestor_record_offset":40676,"inlinee":5905,"source_line":621,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::done
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":479,"record_offset":42080,"parent_record_offset":42044,"ancestor_record_offset":40676,"inlinee":5901,"source_line":132,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":479,"record_offset":41228,"parent_record_offset":41032,"ancestor_record_offset":40676,"inlinee":5621,"source_line":144,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::guard
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":479,"record_offset":41032,"parent_record_offset":41004,"ancestor_record_offset":40676,"inlinee":6011,"source_line":122,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::MutexGuard::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\mutex.rs
//     evidence={"module":479,"record_offset":41004,"parent_record_offset":40876,"ancestor_record_offset":40676,"inlinee":6010,"source_line":647,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::impl$14::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\mutex.rs
//     evidence={"module":479,"record_offset":42044,"parent_record_offset":42016,"ancestor_record_offset":40676,"inlinee":5900,"source_line":671,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":479,"record_offset":40912,"parent_record_offset":40876,"ancestor_record_offset":40676,"inlinee":6008,"source_line":27,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::unlock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":479,"record_offset":42328,"parent_record_offset":42044,"ancestor_record_offset":40676,"inlinee":5909,"source_line":83,"archive":"_pdb/B/modules/00479.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\mod.rs
//     evidence={"module":479,"record_offset":41072,"parent_record_offset":41032,"ancestor_record_offset":40676,"inlinee":5903,"source_line":814,"archive":"_pdb/B/modules/00479.jsonl.gz"}
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
// 完整结构及原始类型记录: configwatch.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: configwatch.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: configwatch.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::configwatch::ConfigWatch>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::color_library::ColorLibrarySsbo>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>", "bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::configwatch::ConfigWatch>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::color_library::ColorLibrarySsbo>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::configwatch::ConfigWatch>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::color_library::ColorLibrarySsbo>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>

// ------------------------------------------------------------------
// 函数: country_core::systems::backend::configwatch::configwatch
// 信息 ID: 308ef74133117acbf5c89cb3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 874f0e528c73d064d3940c5f: void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::configwatch::ConfigWatch>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::color_library::ColorLibrarySsbo>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::configwatch::ConfigWatch>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::color_library::ColorLibrarySsbo>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::configwatch::ConfigWatch>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::color_library::ColorLibrarySsbo>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)> > @ 0x13f9626 -> 0x85b1c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::configwatch::ConfigWatch>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::color_library::ColorLibrarySsbo>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::configwatch::ConfigWatch>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::color_library::ColorLibrarySsbo>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)> > > @ 0x14bb308 -> 0x85b1c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===

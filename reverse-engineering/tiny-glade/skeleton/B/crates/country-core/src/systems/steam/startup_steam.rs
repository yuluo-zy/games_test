// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/systems/steam/startup_steam.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: d726a70a7c738d0c1379ad158ad9b3a8fef280ae487d435e5e98e11bad5c293b
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: startup_steam.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 505, 559, 1729
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use core; // namespace candidate
//   package identities: []
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.27"}]
// use std; // namespace candidate
//   package identities: []
// use steamworks; // namespace candidate
//   package identities: [{"name":"steamworks","source":"git+https://github.com/PounceLight/steamworks-rs.git?rev=d05eaa60db328156aeee172b0b157f3f69c70b30#d05eaa60db328156aeee172b0b157f3f69c70b30","version":"0.11.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::steam::startup_steam::impl$0::check_get_action_set_handle
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 12
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7033
// 首条位置: {"module":1729,"inlinee":28723}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::steam::startup_steam::impl$0::check_get_analog_action_handle
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 22
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7034
// 首条位置: {"module":1729,"inlinee":28724}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1729,"record_offset":77112,"parent_record_offset":76996,"ancestor_record_offset":76996,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1729,"record_offset":77144,"parent_record_offset":76996,"ancestor_record_offset":76996,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1729,"record_offset":77028,"parent_record_offset":76996,"ancestor_record_offset":76996,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":1729,"record_offset":77200,"parent_record_offset":77168,"ancestor_record_offset":76996,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":1729,"record_offset":77228,"parent_record_offset":77168,"ancestor_record_offset":76996,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1729,"record_offset":77052,"parent_record_offset":77028,"ancestor_record_offset":76996,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1729,"record_offset":77076,"parent_record_offset":77052,"ancestor_record_offset":76996,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1729,"record_offset":77256,"parent_record_offset":77168,"ancestor_record_offset":76996,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1729,"record_offset":77168,"parent_record_offset":77144,"ancestor_record_offset":76996,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1729,"record_offset":77280,"parent_record_offset":77256,"ancestor_record_offset":76996,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::steam::startup_steam::impl$0::check_get_digital_action_handle
// 记录形式: inline_instance, inlinee_declaration; instances=35
// 源码行: 32
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7035
// 首条位置: {"module":1729,"inlinee":28725}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=34
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1729,"record_offset":78096,"parent_record_offset":77980,"ancestor_record_offset":77980,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=34
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1729,"record_offset":78128,"parent_record_offset":77980,"ancestor_record_offset":77980,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=34
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1729,"record_offset":78012,"parent_record_offset":77980,"ancestor_record_offset":77980,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=34
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":1729,"record_offset":78184,"parent_record_offset":78152,"ancestor_record_offset":77980,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=34
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":1729,"record_offset":78212,"parent_record_offset":78152,"ancestor_record_offset":77980,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=34
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1729,"record_offset":78036,"parent_record_offset":78012,"ancestor_record_offset":77980,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=34
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1729,"record_offset":78060,"parent_record_offset":78036,"ancestor_record_offset":77980,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=34
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1729,"record_offset":78240,"parent_record_offset":78152,"ancestor_record_offset":77980,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=34
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1729,"record_offset":78152,"parent_record_offset":78128,"ancestor_record_offset":77980,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=34
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1729,"record_offset":78264,"parent_record_offset":78240,"ancestor_record_offset":77980,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/01729.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::steam::startup_steam::startup_steam
// 记录形式: compiled_procedure; instances=1
// 源码行: 42
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":559,"record_offset":74396,"rva":11123696,"va":5379832816}
// 编译实例 RVAs: 0xa9bbf0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1429236e0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: startup_steam.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32012963","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379832816,"raw_hex":"e8a37ae801","size":5,"soft_fail":false,"target_va":5411845856,"va":5379832888,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923700
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: startup_steam.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32013032","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379832816,"raw_hex":"e8e87ae801","size":5,"soft_fail":false,"target_va":5411845888,"va":5379832851,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t956002","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379832816,"raw_hex":"e862960e00","size":5,"soft_fail":false,"target_va":5380789072,"va":5379833065,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::system::commands::Commands::insert_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t8669579","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379832816,"raw_hex":"e88b498400","size":5,"soft_fail":false,"target_va":5388502624,"va":5379833040,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::hash::random::RandomState::new::KEYS::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2824958","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379832816,"raw_hex":"e8fe1a2b00","size":5,"soft_fail":false,"target_va":5382657856,"va":5379832893,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::random::hashmap_random_keys
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2920864","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379832816,"raw_hex":"e8a0912c00","size":5,"soft_fail":false,"target_va":5382753792,"va":5379832923,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":559,"record_offset":75420,"parent_record_offset":74396,"ancestor_record_offset":74396,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":559,"record_offset":74748,"parent_record_offset":74396,"ancestor_record_offset":74396,"inlinee":7397,"source_line":1406,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] steamworks::Client::input
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"steamworks","version":"0.11.0","source":"git+https://github.com/PounceLight/steamworks-rs.git?rev=d05eaa60db328156aeee172b0b157f3f69c70b30#d05eaa60db328156aeee172b0b157f3f69c70b30"}]
//     definition_source=/home/h3/.cargo/git/checkouts/steamworks-rs-f13bba748a20c02a/d05eaa6/src/lib.rs
//     evidence={"module":559,"record_offset":74552,"parent_record_offset":74396,"ancestor_record_offset":74396,"inlinee":30543,"source_line":345,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] steamworks::input::Input::init
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"steamworks","version":"0.11.0","source":"git+https://github.com/PounceLight/steamworks-rs.git?rev=d05eaa60db328156aeee172b0b157f3f69c70b30#d05eaa60db328156aeee172b0b157f3f69c70b30"}]
//     definition_source=/home/h3/.cargo/git/checkouts/steamworks-rs-f13bba748a20c02a/d05eaa6/src/input.rs
//     evidence={"module":559,"record_offset":74720,"parent_record_offset":74396,"ancestor_record_offset":74396,"inlinee":29671,"source_line":33,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":559,"record_offset":74620,"parent_record_offset":74584,"ancestor_record_offset":74396,"inlinee":4776,"source_line":1888,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":559,"record_offset":74584,"parent_record_offset":74552,"ancestor_record_offset":74396,"inlinee":6804,"source_line":2163,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$37::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":559,"record_offset":75468,"parent_record_offset":75444,"ancestor_record_offset":74396,"inlinee":4774,"source_line":2550,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":559,"record_offset":75248,"parent_record_offset":75220,"ancestor_record_offset":74396,"inlinee":4798,"source_line":541,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":559,"record_offset":75308,"parent_record_offset":75276,"ancestor_record_offset":74396,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":559,"record_offset":75112,"parent_record_offset":75088,"ancestor_record_offset":74396,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":559,"record_offset":75276,"parent_record_offset":75220,"ancestor_record_offset":74396,"inlinee":6331,"source_line":1980,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":559,"record_offset":74876,"parent_record_offset":74844,"ancestor_record_offset":74396,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":559,"record_offset":74980,"parent_record_offset":74956,"ancestor_record_offset":74396,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":559,"record_offset":75444,"parent_record_offset":75420,"ancestor_record_offset":74396,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":559,"record_offset":75064,"parent_record_offset":74956,"ancestor_record_offset":74396,"inlinee":7403,"source_line":1581,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":559,"record_offset":74652,"parent_record_offset":74620,"ancestor_record_offset":74396,"inlinee":4777,"source_line":425,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":559,"record_offset":75136,"parent_record_offset":75112,"ancestor_record_offset":74396,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":559,"record_offset":75088,"parent_record_offset":75064,"ancestor_record_offset":74396,"inlinee":7404,"source_line":1212,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":559,"record_offset":75164,"parent_record_offset":75112,"ancestor_record_offset":74396,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":559,"record_offset":74680,"parent_record_offset":74620,"ancestor_record_offset":74396,"inlinee":4128,"source_line":3760,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":559,"record_offset":75564,"parent_record_offset":75504,"ancestor_record_offset":74396,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":559,"record_offset":75596,"parent_record_offset":75468,"ancestor_record_offset":74396,"inlinee":4779,"source_line":4075,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":559,"record_offset":74796,"parent_record_offset":74772,"ancestor_record_offset":74396,"inlinee":4794,"source_line":56,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__init
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":559,"record_offset":75024,"parent_record_offset":75004,"ancestor_record_offset":74396,"inlinee":7402,"source_line":83,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::constant$0::closure$1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":559,"record_offset":74900,"parent_record_offset":74876,"ancestor_record_offset":74396,"inlinee":7399,"source_line":97,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":559,"record_offset":75220,"parent_record_offset":74844,"ancestor_record_offset":74396,"inlinee":4797,"source_line":72,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":559,"record_offset":74772,"parent_record_offset":74748,"ancestor_record_offset":74396,"inlinee":4792,"source_line":151,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":559,"record_offset":74928,"parent_record_offset":74900,"ancestor_record_offset":74396,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::initialize
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":559,"record_offset":74956,"parent_record_offset":74928,"ancestor_record_offset":74396,"inlinee":7400,"source_line":61,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":559,"record_offset":74844,"parent_record_offset":74820,"ancestor_record_offset":74396,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":559,"record_offset":74820,"parent_record_offset":74796,"ancestor_record_offset":74396,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/00559.jsonl.gz"}
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
// 完整结构及原始类型记录: startup_steam.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: startup_steam.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: startup_steam.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 13; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>", "bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_watch::TextureWatch>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_watch::TextureWatch>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_watch::TextureWatch>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::draw_passes::final_post::FinalPostResources>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>", "bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::draw_passes::final_post::FinalPostResources>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::draw_passes::final_post::FinalPostResources>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::writer::EventWriter<country_core::resources::mouse_capture::CustomCursorSsboCreatedCmd>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::event::writer::EventWriter<country_core::resources::mouse_capture::CustomCursorSsboCreatedCmd>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::mouse_capture::CustomCursorSsboCreatedCmd>
// [文件级接口候选，具体函数归属 unknown] void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::startup::load_prefabs::PrefabWatcher> > >,bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>)
// 候选参数类型: ["enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::startup::load_prefabs::PrefabWatcher> > >", "bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::startup::load_prefabs::PrefabWatcher>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::configwatch::ConfigWatch>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::configwatch::ConfigWatch>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::configwatch::ConfigWatch>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::system::commands::Commands"]
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::systems::steam::startup_steam::impl$0::check_get_action_set_handle
// 信息 ID: d4f1b4a0be05202c12626268
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::steam::SteamMainActionSet::try_load<steamworks::input::Input<steamworks::ClientManager> >; module=1729, inline_record=76908; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::steam::SteamMenuActionSet::try_load<steamworks::input::Input<steamworks::ClientManager> >; module=1729, inline_record=86396; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::steam::startup_steam::impl$0::check_get_analog_action_handle
// 信息 ID: 79c71a8da25220b584a1f136
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::steam::SteamMainActionSet::try_load<steamworks::input::Input<steamworks::ClientManager> >; module=1729, inline_record=76996; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::steam::SteamMainActionSet::try_load<steamworks::input::Input<steamworks::ClientManager> >; module=1729, inline_record=77324; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::steam::SteamMainActionSet::try_load<steamworks::input::Input<steamworks::ClientManager> >; module=1729, inline_record=77652; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::steam::SteamMenuActionSet::try_load<steamworks::input::Input<steamworks::ClientManager> >; module=1729, inline_record=86484; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::steam::startup_steam::impl$0::check_get_digital_action_handle
// 信息 ID: 7134f17372009cf95c1ff774
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 34 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::steam::SteamMainActionSet::try_load<steamworks::input::Input<steamworks::ClientManager> >; module=1729, inline_record=77980; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::steam::SteamMainActionSet::try_load<steamworks::input::Input<steamworks::ClientManager> >; module=1729, inline_record=78308; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::steam::SteamMainActionSet::try_load<steamworks::input::Input<steamworks::ClientManager> >; module=1729, inline_record=78636; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::steam::SteamMainActionSet::try_load<steamworks::input::Input<steamworks::ClientManager> >; module=1729, inline_record=78964; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::steam::SteamMainActionSet::try_load<steamworks::input::Input<steamworks::ClientManager> >; module=1729, inline_record=79292; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::steam::SteamMainActionSet::try_load<steamworks::input::Input<steamworks::ClientManager> >; module=1729, inline_record=79620; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::steam::SteamMainActionSet::try_load<steamworks::input::Input<steamworks::ClientManager> >; module=1729, inline_record=79948; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::steam::SteamMainActionSet::try_load<steamworks::input::Input<steamworks::ClientManager> >; module=1729, inline_record=80276; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::steam::startup_steam::startup_steam
// 信息 ID: a33084708526e605e17f797d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 19a238a0b080cc5fdb23afff: void (*)(bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource>,bevy_ecs::system::commands::Commands)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource>,bevy_ecs::system::commands::Commands)> > > @ 0x14c64c5 -> 0xa9bbf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource>,bevy_ecs::system::commands::Commands)> > @ 0x166af2f -> 0xa9bbf0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===

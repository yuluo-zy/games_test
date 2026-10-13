// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/rhapsody/src/raw/swapchain.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: fef5d0b5905b50e640fca657914029820d1ced5f84e29d2ca04d2f909b554953
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: swapchain.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 219, 569, 570, 1622, 1975, 3027
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use append_only_vec; // namespace candidate
//   package identities: []
// use ash; // namespace candidate
//   package identities: [{"name":"ash","source":"PDB_registry_path_only","version":"0.38.0+1.3.281"}]
// use core; // namespace candidate
//   package identities: []
// use lock_api; // namespace candidate
//   package identities: [{"name":"lock_api","source":"PDB_registry_path_only","version":"0.4.9"}]
// use parking_lot; // namespace candidate
//   package identities: [{"name":"parking_lot","source":"PDB_registry_path_only","version":"0.12.1"}]
// use rhapsody; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::swapchain::impl$5::fmt
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=9
// 源码行: 13
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x32E4
// 首条位置: {"module":219,"inlinee":13028}
// 编译实例 RVAs: 0x339d90, 0x118b5e0, 0x12f1640
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::fmt::Formatter::debug_struct_field4_finish
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t5492761","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372091792,"raw_hex":"e819d05300","size":5,"soft_fail":false,"target_va":5377584768,"va":5372092002,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::swapchain::impl$6::fmt
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=5
// 源码行: 27
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x32E6
// 首条位置: {"module":219,"inlinee":13030}
// 编译实例 RVAs: 0x118b6c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::fmt::Formatter::debug_struct_field1_finish
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-9522102","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387105984,"raw_hex":"e84ab46eff","size":5,"soft_fail":false,"target_va":5377584016,"va":5387106113,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::fmt::Formatter::debug_struct_field2_finish
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-9522090","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387105984,"raw_hex":"e856b46eff","size":5,"soft_fail":false,"target_va":5377584208,"va":5387106293,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::fmt::Formatter::debug_tuple_field2_finish
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-9520093","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387105984,"raw_hex":"e823bc6eff","size":5,"soft_fail":false,"target_va":5377586096,"va":5387106184,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] alias_set_at_0x140876b30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: swapchain.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t-9522133","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5387105984,"raw_hex":"e92bb46eff","size":5,"soft_fail":false,"target_va":5377583920,"va":5387106048,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::swapchain::Swapchain::enumerate_surface_formats
// 记录形式: compiled_procedure; instances=1
// 源码行: 51
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":569,"record_offset":804,"rva":7567888,"va":5376277008}
// 编译实例 RVAs: 0x737a10
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] ash::prelude::read_into_uninitialized_vector
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t10788667","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376277008,"raw_hex":"e83b9fa400","size":5,"soft_fail":false,"target_va":5387065760,"va":5376277088,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":569,"record_offset":956,"parent_record_offset":804,"ancestor_record_offset":804,"inlinee":4586,"source_line":2105,"archive":"_pdb/A/modules/00569.jsonl.gz"}
//   [pdb_inline_direct] ash::extensions_generated::khr::surface::Instance::get_physical_device_surface_formats
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/extensions/khr/surface.rs
//     evidence={"module":569,"record_offset":984,"parent_record_offset":804,"ancestor_record_offset":804,"inlinee":30875,"source_line":64,"archive":"_pdb/A/modules/00569.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::swapchain::Swapchain::on_focus
// 记录形式: compiled_procedure; instances=1
// 源码行: 62
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":569,"record_offset":1016,"rva":7567984,"va":5376277104}
// 编译实例 RVAs: 0x737a70
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26701529","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376277104,"raw_hex":"e8d96e9701","size":5,"soft_fail":false,"target_va":5402978880,"va":5376277346,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t26949188","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376277104,"raw_hex":"e844369b01","size":5,"soft_fail":false,"target_va":5403226480,"va":5376277287,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::lock_exclusive_slow
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t26686686","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376277104,"raw_hex":"e8de349701","size":5,"soft_fail":false,"target_va":5402963904,"va":5376277213,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] parking_lot::raw_rwlock::RawRwLock::unlock_exclusive_slow
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tjmp\t26688101","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5376277104,"raw_hex":"e9653a9701","size":5,"soft_fail":false,"target_va":5402965344,"va":5376277238,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":569,"record_offset":1276,"parent_record_offset":1016,"ancestor_record_offset":1016,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00569.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::resources::impl$4::get
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":569,"record_offset":1156,"parent_record_offset":1016,"ancestor_record_offset":1016,"inlinee":7658,"source_line":314,"archive":"_pdb/A/modules/00569.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::swapchain::ManagedSwapchain::on_focus
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/swapchain.rs
//     evidence={"module":569,"record_offset":1312,"parent_record_offset":1016,"ancestor_record_offset":1016,"inlinee":30877,"source_line":54,"archive":"_pdb/A/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":569,"record_offset":1548,"parent_record_offset":1312,"ancestor_record_offset":1016,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":569,"record_offset":1456,"parent_record_offset":1368,"ancestor_record_offset":1016,"inlinee":5288,"source_line":581,"archive":"_pdb/A/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":569,"record_offset":1480,"parent_record_offset":1456,"ancestor_record_offset":1016,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":569,"record_offset":1632,"parent_record_offset":1600,"ancestor_record_offset":1016,"inlinee":5796,"source_line":2580,"archive":"_pdb/A/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":569,"record_offset":1400,"parent_record_offset":1368,"ancestor_record_offset":1016,"inlinee":5726,"source_line":2629,"archive":"_pdb/A/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":569,"record_offset":1656,"parent_record_offset":1632,"ancestor_record_offset":1016,"inlinee":5298,"source_line":3369,"archive":"_pdb/A/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":569,"record_offset":1424,"parent_record_offset":1400,"ancestor_record_offset":1016,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::RwLock::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":569,"record_offset":1340,"parent_record_offset":1312,"ancestor_record_offset":1016,"inlinee":5786,"source_line":479,"archive":"_pdb/A/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::impl$26::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":569,"record_offset":1572,"parent_record_offset":1548,"ancestor_record_offset":1016,"inlinee":9025,"source_line":1667,"archive":"_pdb/A/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::lock_exclusive
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":569,"record_offset":1368,"parent_record_offset":1340,"ancestor_record_offset":1016,"inlinee":5787,"source_line":67,"archive":"_pdb/A/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::unlock_exclusive
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":569,"record_offset":1600,"parent_record_offset":1572,"ancestor_record_offset":1016,"inlinee":5795,"source_line":94,"archive":"_pdb/A/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$32::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":569,"record_offset":1192,"parent_record_offset":1156,"ancestor_record_offset":1016,"inlinee":7648,"source_line":14,"archive":"_pdb/A/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$40::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":569,"record_offset":1244,"parent_record_offset":1156,"ancestor_record_offset":1016,"inlinee":7652,"source_line":23,"archive":"_pdb/A/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$47::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":569,"record_offset":1220,"parent_record_offset":1156,"ancestor_record_offset":1016,"inlinee":7650,"source_line":32,"archive":"_pdb/A/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::swapchain::SwapchainInner::on_focus
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/swapchain.rs
//     evidence={"module":569,"record_offset":1520,"parent_record_offset":1312,"ancestor_record_offset":1016,"inlinee":30879,"source_line":372,"archive":"_pdb/A/modules/00569.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::swapchain::Swapchain::format
// 记录形式: compiled_procedure; instances=1
// 源码行: 71
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":569,"record_offset":1736,"rva":7568240,"va":5376277360}
// 编译实例 RVAs: 0x737b70
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::swapchain::Swapchain::resize
// 记录形式: compiled_procedure; instances=1
// 源码行: 74
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":569,"record_offset":1856,"rva":7568256,"va":5376277376}
// 编译实例 RVAs: 0x737b80
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26701333","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376277376,"raw_hex":"e8156e9701","size":5,"soft_fail":false,"target_va":5402978880,"va":5376277542,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t26948992","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376277376,"raw_hex":"e880359b01","size":5,"soft_fail":false,"target_va":5403226480,"va":5376277483,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] rhapsody::raw::managed::swapchain::ManagedSwapchain::resize
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tjmp\t10822273","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5376277376,"raw_hex":"e98122a500","size":5,"soft_fail":false,"target_va":5387099712,"va":5376277434,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":569,"record_offset":2108,"parent_record_offset":1856,"ancestor_record_offset":1856,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00569.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::resources::impl$4::get
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":569,"record_offset":1988,"parent_record_offset":1856,"ancestor_record_offset":1856,"inlinee":7658,"source_line":314,"archive":"_pdb/A/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$32::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":569,"record_offset":2024,"parent_record_offset":1988,"ancestor_record_offset":1856,"inlinee":7648,"source_line":14,"archive":"_pdb/A/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$40::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":569,"record_offset":2076,"parent_record_offset":1988,"ancestor_record_offset":1856,"inlinee":7652,"source_line":23,"archive":"_pdb/A/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$47::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":569,"record_offset":2052,"parent_record_offset":1988,"ancestor_record_offset":1856,"inlinee":7650,"source_line":32,"archive":"_pdb/A/modules/00569.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::swapchain::Swapchain::set_vsync
// 记录形式: compiled_procedure; instances=1
// 源码行: 78
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":569,"record_offset":2144,"rva":7568432,"va":5376277552}
// 编译实例 RVAs: 0x737c30
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26701157","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376277552,"raw_hex":"e8656d9701","size":5,"soft_fail":false,"target_va":5402978880,"va":5376277718,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t26948816","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376277552,"raw_hex":"e8d0349b01","size":5,"soft_fail":false,"target_va":5403226480,"va":5376277659,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] rhapsody::raw::managed::swapchain::ManagedSwapchain::set_vsync
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tjmp\t10822465","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5376277552,"raw_hex":"e94123a500","size":5,"soft_fail":false,"target_va":5387100080,"va":5376277610,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":569,"record_offset":2400,"parent_record_offset":2144,"ancestor_record_offset":2144,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00569.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::resources::impl$4::get
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":569,"record_offset":2280,"parent_record_offset":2144,"ancestor_record_offset":2144,"inlinee":7658,"source_line":314,"archive":"_pdb/A/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$32::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":569,"record_offset":2316,"parent_record_offset":2280,"ancestor_record_offset":2144,"inlinee":7648,"source_line":14,"archive":"_pdb/A/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$40::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":569,"record_offset":2368,"parent_record_offset":2280,"ancestor_record_offset":2144,"inlinee":7652,"source_line":23,"archive":"_pdb/A/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$47::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":569,"record_offset":2344,"parent_record_offset":2280,"ancestor_record_offset":2144,"inlinee":7650,"source_line":32,"archive":"_pdb/A/modules/00569.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::swapchain::Swapchain::mark_stale
// 记录形式: compiled_procedure; instances=1
// 源码行: 82
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":569,"record_offset":2436,"rva":7568608,"va":5376277728}
// 编译实例 RVAs: 0x737ce0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26700866","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376277728,"raw_hex":"e8426c9701","size":5,"soft_fail":false,"target_va":5402978880,"va":5376278009,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t26948525","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376277728,"raw_hex":"e8ad339b01","size":5,"soft_fail":false,"target_va":5403226480,"va":5376277950,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::lock_exclusive_slow
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t26686063","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376277728,"raw_hex":"e86f329701","size":5,"soft_fail":false,"target_va":5402963904,"va":5376277836,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] parking_lot::raw_rwlock::RawRwLock::unlock_exclusive_slow
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tjmp\t26687436","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5376277728,"raw_hex":"e9cc379701","size":5,"soft_fail":false,"target_va":5402965344,"va":5376277903,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":569,"record_offset":2700,"parent_record_offset":2436,"ancestor_record_offset":2436,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00569.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::resources::impl$4::get
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":569,"record_offset":2576,"parent_record_offset":2436,"ancestor_record_offset":2436,"inlinee":7658,"source_line":314,"archive":"_pdb/A/modules/00569.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::swapchain::ManagedSwapchain::mark_stale
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/swapchain.rs
//     evidence={"module":569,"record_offset":2736,"parent_record_offset":2436,"ancestor_record_offset":2436,"inlinee":30880,"source_line":70,"archive":"_pdb/A/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::get_or_insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":569,"record_offset":2972,"parent_record_offset":2944,"ancestor_record_offset":2436,"inlinee":5444,"source_line":1603,"archive":"_pdb/A/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":569,"record_offset":3012,"parent_record_offset":2736,"ancestor_record_offset":2436,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":569,"record_offset":2880,"parent_record_offset":2792,"ancestor_record_offset":2436,"inlinee":5288,"source_line":581,"archive":"_pdb/A/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":569,"record_offset":2904,"parent_record_offset":2880,"ancestor_record_offset":2436,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":569,"record_offset":3100,"parent_record_offset":3064,"ancestor_record_offset":2436,"inlinee":5796,"source_line":2580,"archive":"_pdb/A/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":569,"record_offset":2824,"parent_record_offset":2792,"ancestor_record_offset":2436,"inlinee":5726,"source_line":2629,"archive":"_pdb/A/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":569,"record_offset":3128,"parent_record_offset":3100,"ancestor_record_offset":2436,"inlinee":5298,"source_line":3369,"archive":"_pdb/A/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":569,"record_offset":2848,"parent_record_offset":2824,"ancestor_record_offset":2436,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::RwLock::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":569,"record_offset":2764,"parent_record_offset":2736,"ancestor_record_offset":2436,"inlinee":5786,"source_line":479,"archive":"_pdb/A/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::impl$26::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":569,"record_offset":3036,"parent_record_offset":3012,"ancestor_record_offset":2436,"inlinee":9025,"source_line":1667,"archive":"_pdb/A/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::lock_exclusive
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":569,"record_offset":2792,"parent_record_offset":2764,"ancestor_record_offset":2436,"inlinee":5787,"source_line":67,"archive":"_pdb/A/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::unlock_exclusive
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":569,"record_offset":3064,"parent_record_offset":3036,"ancestor_record_offset":2436,"inlinee":5795,"source_line":94,"archive":"_pdb/A/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$32::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":569,"record_offset":2616,"parent_record_offset":2576,"ancestor_record_offset":2436,"inlinee":7648,"source_line":14,"archive":"_pdb/A/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$40::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":569,"record_offset":2668,"parent_record_offset":2576,"ancestor_record_offset":2436,"inlinee":7652,"source_line":23,"archive":"_pdb/A/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$47::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":569,"record_offset":2644,"parent_record_offset":2576,"ancestor_record_offset":2436,"inlinee":7650,"source_line":32,"archive":"_pdb/A/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::swapchain::SwapchainInner::mark_stale
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/swapchain.rs
//     evidence={"module":569,"record_offset":2944,"parent_record_offset":2736,"ancestor_record_offset":2436,"inlinee":30881,"source_line":427,"archive":"_pdb/A/modules/00569.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::swapchain::Swapchain::extent
// 记录形式: compiled_procedure; instances=1
// 源码行: 94
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":569,"record_offset":3216,"rva":7568896,"va":5376278016}
// 编译实例 RVAs: 0x737e00
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26700605","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376278016,"raw_hex":"e83d6b9701","size":5,"soft_fail":false,"target_va":5402978880,"va":5376278270,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t26948264","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376278016,"raw_hex":"e8a8329b01","size":5,"soft_fail":false,"target_va":5403226480,"va":5376278211,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::lock_shared_slow
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t26688593","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376278016,"raw_hex":"e8513c9701","size":5,"soft_fail":false,"target_va":5402966736,"va":5376278138,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::unlock_shared_slow
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t26690192","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376278016,"raw_hex":"e890429701","size":5,"soft_fail":false,"target_va":5402968352,"va":5376278155,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":569,"record_offset":3472,"parent_record_offset":3216,"ancestor_record_offset":3216,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00569.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::resources::impl$4::get
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":569,"record_offset":3352,"parent_record_offset":3216,"ancestor_record_offset":3216,"inlinee":7658,"source_line":314,"archive":"_pdb/A/modules/00569.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::swapchain::ManagedSwapchain::desc
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/swapchain.rs
//     evidence={"module":569,"record_offset":3508,"parent_record_offset":3216,"ancestor_record_offset":3216,"inlinee":30884,"source_line":58,"archive":"_pdb/A/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":569,"record_offset":3688,"parent_record_offset":3600,"ancestor_record_offset":3216,"inlinee":5724,"source_line":460,"archive":"_pdb/A/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":569,"record_offset":3720,"parent_record_offset":3688,"ancestor_record_offset":3216,"inlinee":5725,"source_line":1975,"archive":"_pdb/A/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":569,"record_offset":3824,"parent_record_offset":3508,"ancestor_record_offset":3216,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":569,"record_offset":3756,"parent_record_offset":3600,"ancestor_record_offset":3216,"inlinee":5726,"source_line":2629,"archive":"_pdb/A/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":569,"record_offset":3908,"parent_record_offset":3876,"ancestor_record_offset":3216,"inlinee":4708,"source_line":2695,"archive":"_pdb/A/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":569,"record_offset":3632,"parent_record_offset":3600,"ancestor_record_offset":3216,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":569,"record_offset":3780,"parent_record_offset":3756,"ancestor_record_offset":3216,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":569,"record_offset":3656,"parent_record_offset":3632,"ancestor_record_offset":3216,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":569,"record_offset":3932,"parent_record_offset":3908,"ancestor_record_offset":3216,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::RwLock::read
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":569,"record_offset":3544,"parent_record_offset":3508,"ancestor_record_offset":3216,"inlinee":5719,"source_line":447,"archive":"_pdb/A/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::impl$17::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":569,"record_offset":3848,"parent_record_offset":3824,"ancestor_record_offset":3216,"inlinee":9020,"source_line":1332,"archive":"_pdb/A/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::try_lock_shared_fast
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":569,"record_offset":3600,"parent_record_offset":3572,"ancestor_record_offset":3216,"inlinee":5723,"source_line":508,"archive":"_pdb/A/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::lock_shared
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":569,"record_offset":3572,"parent_record_offset":3544,"ancestor_record_offset":3216,"inlinee":5721,"source_line":107,"archive":"_pdb/A/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::unlock_shared
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":569,"record_offset":3876,"parent_record_offset":3848,"ancestor_record_offset":3216,"inlinee":5735,"source_line":129,"archive":"_pdb/A/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$32::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":569,"record_offset":3388,"parent_record_offset":3352,"ancestor_record_offset":3216,"inlinee":7648,"source_line":14,"archive":"_pdb/A/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$40::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":569,"record_offset":3440,"parent_record_offset":3352,"ancestor_record_offset":3216,"inlinee":7652,"source_line":23,"archive":"_pdb/A/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$47::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":569,"record_offset":3416,"parent_record_offset":3352,"ancestor_record_offset":3216,"inlinee":7650,"source_line":32,"archive":"_pdb/A/modules/00569.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::swapchain::Swapchain::acquire_next_texture
// 记录形式: compiled_procedure; instances=1
// 源码行: 98
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":569,"record_offset":3984,"rva":7569168,"va":5376278288}
// 编译实例 RVAs: 0x737f10
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1407379d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: swapchain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1773","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376278288,"raw_hex":"e813f9ffff","size":5,"soft_fail":false,"target_va":5376276944,"va":5376278712,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26700273","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376278288,"raw_hex":"e8f1699701","size":5,"soft_fail":false,"target_va":5402978880,"va":5376278602,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t26947930","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376278288,"raw_hex":"e85a319b01","size":5,"soft_fail":false,"target_va":5403226480,"va":5376278545,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26700263","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376278288,"raw_hex":"e8e7699701","size":5,"soft_fail":false,"target_va":5402978944,"va":5376278676,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::managed::swapchain::ManagedSwapchain::acquire_next
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t10822045","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376278288,"raw_hex":"e89d21a500","size":5,"soft_fail":false,"target_va":5387100432,"va":5376278382,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":569,"record_offset":4268,"parent_record_offset":3984,"ancestor_record_offset":3984,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00569.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::is_ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":569,"record_offset":4304,"parent_record_offset":3984,"ancestor_record_offset":3984,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/00569.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::handle::impl$17::track
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":569,"record_offset":4332,"parent_record_offset":3984,"ancestor_record_offset":3984,"inlinee":7654,"source_line":250,"archive":"_pdb/A/modules/00569.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::resources::impl$4::get
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":569,"record_offset":4140,"parent_record_offset":3984,"ancestor_record_offset":3984,"inlinee":7658,"source_line":314,"archive":"_pdb/A/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":569,"record_offset":4540,"parent_record_offset":4448,"ancestor_record_offset":3984,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU64::fetch_max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":569,"record_offset":4484,"parent_record_offset":4448,"ancestor_record_offset":3984,"inlinee":30861,"source_line":2932,"archive":"_pdb/A/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_umax
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":569,"record_offset":4508,"parent_record_offset":4484,"ancestor_record_offset":3984,"inlinee":26109,"source_line":3538,"archive":"_pdb/A/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::command_buffer::CommandBufferSeqTracker::track
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/command_buffer.rs
//     evidence={"module":569,"record_offset":4448,"parent_record_offset":4360,"ancestor_record_offset":3984,"inlinee":30860,"source_line":54,"archive":"_pdb/A/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$32::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":569,"record_offset":4184,"parent_record_offset":4140,"ancestor_record_offset":3984,"inlinee":7648,"source_line":14,"archive":"_pdb/A/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$40::eq
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":569,"record_offset":4236,"parent_record_offset":4140,"ancestor_record_offset":3984,"inlinee":7652,"source_line":23,"archive":"_pdb/A/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$47::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":569,"record_offset":4212,"parent_record_offset":4140,"ancestor_record_offset":3984,"inlinee":7650,"source_line":32,"archive":"_pdb/A/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$4::track
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":569,"record_offset":4360,"parent_record_offset":4332,"ancestor_record_offset":3984,"inlinee":30867,"source_line":94,"archive":"_pdb/A/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::resources::ManagedResourceMeta::checksum
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":569,"record_offset":4392,"parent_record_offset":4360,"ancestor_record_offset":3984,"inlinee":30448,"source_line":43,"archive":"_pdb/A/modules/00569.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::swapchain::Swapchain::any_present_successful
// 记录形式: compiled_procedure; instances=1
// 源码行: 117
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":569,"record_offset":4584,"rva":7569616,"va":5376278736}
// 编译实例 RVAs: 0x7380d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::swapchain::Swapchain::present
// 记录形式: compiled_procedure; instances=1
// 源码行: 121
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":569,"record_offset":4720,"rva":7569632,"va":5376278752}
// 编译实例 RVAs: 0x7380e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14072e930
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: swapchain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-39190","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376278752,"raw_hex":"e8ea66ffff","size":5,"soft_fail":false,"target_va":5376239920,"va":5376279105,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26699817","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376278752,"raw_hex":"e829689701","size":5,"soft_fail":false,"target_va":5402978880,"va":5376279058,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t26947486","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376278752,"raw_hex":"e89e2f9b01","size":5,"soft_fail":false,"target_va":5403226480,"va":5376278989,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::managed::swapchain::ManagedSwapchain::present
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t10823638","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376278752,"raw_hex":"e8d627a500","size":5,"soft_fail":false,"target_va":5387102560,"va":5376278917,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":569,"record_offset":4980,"parent_record_offset":4720,"ancestor_record_offset":4720,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00569.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":569,"record_offset":5016,"parent_record_offset":4720,"ancestor_record_offset":4720,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00569.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::resources::impl$4::get
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":569,"record_offset":4856,"parent_record_offset":4720,"ancestor_record_offset":4720,"inlinee":7658,"source_line":314,"archive":"_pdb/A/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":569,"record_offset":5040,"parent_record_offset":5016,"ancestor_record_offset":4720,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$32::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":569,"record_offset":4896,"parent_record_offset":4856,"ancestor_record_offset":4720,"inlinee":7648,"source_line":14,"archive":"_pdb/A/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$40::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":569,"record_offset":4948,"parent_record_offset":4856,"ancestor_record_offset":4720,"inlinee":7652,"source_line":23,"archive":"_pdb/A/modules/00569.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$47::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":569,"record_offset":4924,"parent_record_offset":4856,"ancestor_record_offset":4720,"inlinee":7650,"source_line":32,"archive":"_pdb/A/modules/00569.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::device::GraphicsDevice::create_swapchain
// 记录形式: compiled_procedure; instances=1
// 源码行: 138
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":570,"record_offset":77972,"rva":7618016,"va":5376327136}
// 编译实例 RVAs: 0x743de0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: swapchain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t27265971","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376327136,"raw_hex":"e8b30ba001","size":5,"soft_fail":false,"target_va":5403593360,"va":5376327384,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] append_only_vec::AppendOnlyVec$LT$T$GT$::push
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t10763691","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376327136,"raw_hex":"e8ab3da400","size":5,"soft_fail":false,"target_va":5387091248,"va":5376327552,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] append_only_vec::indices
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t35331","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376327136,"raw_hex":"e8038a0000","size":5,"soft_fail":false,"target_va":5376362912,"va":5376327576,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26651088","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376327136,"raw_hex":"e8d0a99601","size":5,"soft_fail":false,"target_va":5402978880,"va":5376327787,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t26898750","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376327136,"raw_hex":"e83e719a01","size":5,"soft_fail":false,"target_va":5403226480,"va":5376327725,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26651336","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376327136,"raw_hex":"e8c8aa9601","size":5,"soft_fail":false,"target_va":5402979152,"va":5376327811,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26651664","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376327136,"raw_hex":"e810ac9601","size":5,"soft_fail":false,"target_va":5402979344,"va":5376327675,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::managed::swapchain::ManagedSwapchain::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t10768985","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376327136,"raw_hex":"e85952a400","size":5,"soft_fail":false,"target_va":5387096272,"va":5376327282,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::managed::swapchain::ManagedSwapchain::retire
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t10777475","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376327136,"raw_hex":"e88373a400","size":5,"soft_fail":false,"target_va":5387104704,"va":5376327224,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":570,"record_offset":78124,"parent_record_offset":77972,"ancestor_record_offset":77972,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":570,"record_offset":78356,"parent_record_offset":77972,"ancestor_record_offset":77972,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":570,"record_offset":78392,"parent_record_offset":77972,"ancestor_record_offset":77972,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::resources::impl$4::create
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":570,"record_offset":78420,"parent_record_offset":77972,"ancestor_record_offset":77972,"inlinee":11133,"source_line":297,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] append_only_vec::AppendOnlyVec::len
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/append-only-vec-75e049b16865f40a/4031d67/src/lib.rs
//     evidence={"module":570,"record_offset":78776,"parent_record_offset":78728,"ancestor_record_offset":77972,"inlinee":13428,"source_line":104,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] append_only_vec::impl$4::index
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/append-only-vec-75e049b16865f40a/4031d67/src/lib.rs
//     evidence={"module":570,"record_offset":78728,"parent_record_offset":78496,"ancestor_record_offset":77972,"inlinee":30795,"source_line":225,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":570,"record_offset":78312,"parent_record_offset":78156,"ancestor_record_offset":77972,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":570,"record_offset":78860,"parent_record_offset":78728,"ancestor_record_offset":77972,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::is_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":570,"record_offset":78616,"parent_record_offset":78496,"ancestor_record_offset":77972,"inlinee":30790,"source_line":35,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::is_null::runtime_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":570,"record_offset":78640,"parent_record_offset":78616,"ancestor_record_offset":77972,"inlinee":30792,"source_line":37,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicPtr::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":570,"record_offset":78672,"parent_record_offset":78496,"ancestor_record_offset":77972,"inlinee":30793,"source_line":1638,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicPtr::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":570,"record_offset":78560,"parent_record_offset":78496,"ancestor_record_offset":77972,"inlinee":6265,"source_line":1431,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":570,"record_offset":78800,"parent_record_offset":78776,"ancestor_record_offset":77972,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":570,"record_offset":78696,"parent_record_offset":78672,"ancestor_record_offset":77972,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":570,"record_offset":78584,"parent_record_offset":78560,"ancestor_record_offset":77972,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$32::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":570,"record_offset":78228,"parent_record_offset":78184,"ancestor_record_offset":77972,"inlinee":7648,"source_line":14,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$40::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":570,"record_offset":78280,"parent_record_offset":78184,"ancestor_record_offset":77972,"inlinee":7652,"source_line":23,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$47::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":570,"record_offset":78256,"parent_record_offset":78184,"ancestor_record_offset":77972,"inlinee":7650,"source_line":32,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::resources::ManagedResourceStore::allocate
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":570,"record_offset":78496,"parent_record_offset":78464,"ancestor_record_offset":77972,"inlinee":30789,"source_line":71,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::resources::impl$4::get
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":570,"record_offset":78184,"parent_record_offset":78156,"ancestor_record_offset":77972,"inlinee":7658,"source_line":314,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::resources::impl$8::allocate
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":570,"record_offset":78464,"parent_record_offset":78420,"ancestor_record_offset":77972,"inlinee":31051,"source_line":388,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::swapchain::impl$1::create_swapchain::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/swapchain.rs
//     evidence={"module":570,"record_offset":78156,"parent_record_offset":78124,"ancestor_record_offset":77972,"inlinee":31049,"source_line":143,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::swapchain::impl$1::create_swapchain::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 143
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7949
// 首条位置: {"module":570,"inlinee":31049}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":570,"record_offset":78312,"parent_record_offset":78156,"ancestor_record_offset":78156,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::resources::impl$4::get
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":570,"record_offset":78184,"parent_record_offset":78156,"ancestor_record_offset":78156,"inlinee":7658,"source_line":314,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$32::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":570,"record_offset":78228,"parent_record_offset":78184,"ancestor_record_offset":78156,"inlinee":7648,"source_line":14,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$40::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":570,"record_offset":78280,"parent_record_offset":78184,"ancestor_record_offset":78156,"inlinee":7652,"source_line":23,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$47::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":570,"record_offset":78256,"parent_record_offset":78184,"ancestor_record_offset":78156,"inlinee":7650,"source_line":32,"archive":"_pdb/A/modules/00570.jsonl.gz"}
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
// 完整结构及原始类型记录: swapchain.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::raw::swapchain::Swapchain
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 12; 过程引用数: 11
// [PDB 泛型关系，非字段关系] {"type": "enum2$<rhapsody::raw::swapchain::SwapchainError>", "constructor": "enum2$", "arguments": ["rhapsody::raw::swapchain::SwapchainError"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::result::Result<rhapsody::raw::swapchain::Swapchain,enum2$<rhapsody::raw::swapchain::SwapchainError> >", "constructor": "core::result::Result", "arguments": ["rhapsody::raw::swapchain::Swapchain", "enum2$<rhapsody::raw::swapchain::SwapchainError>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::result::Result<rhapsody::raw::swapchain::Swapchain,enum2$<rhapsody::raw::swapchain::SwapchainError> > >", "constructor": "enum2$", "arguments": ["core::result::Result<rhapsody::raw::swapchain::Swapchain,enum2$<rhapsody::raw::swapchain::SwapchainError> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::result::Result<rhapsody::raw::swapchain::Swapchain,enum2$<rhapsody::raw::swapchain::SwapchainError> > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::result::Result<rhapsody::raw::swapchain::Swapchain,enum2$<rhapsody::raw::swapchain::SwapchainError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::raw::swapchain::SwapchainDesc
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 3
// [PDB 泛型关系，非字段关系] {"type": "core::fmt::impl$51::fmt<rhapsody::raw::swapchain::SwapchainDesc>", "constructor": "core::fmt::impl$51::fmt", "arguments": ["rhapsody::raw::swapchain::SwapchainDesc"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::raw::swapchain::SwapchainError
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 4
// [PDB 泛型关系，非字段关系] {"type": "enum2$<rhapsody::raw::swapchain::SwapchainError>", "constructor": "enum2$", "arguments": ["rhapsody::raw::swapchain::SwapchainError"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::fmt::impl$51::fmt<enum2$<rhapsody::raw::swapchain::SwapchainError> >", "constructor": "core::fmt::impl$51::fmt", "arguments": ["enum2$<rhapsody::raw::swapchain::SwapchainError>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::result::Result<rhapsody::raw::swapchain::Swapchain,enum2$<rhapsody::raw::swapchain::SwapchainError> >", "constructor": "core::result::Result", "arguments": ["rhapsody::raw::swapchain::Swapchain", "enum2$<rhapsody::raw::swapchain::SwapchainError>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::result::Result<rhapsody::raw::swapchain::Swapchain,enum2$<rhapsody::raw::swapchain::SwapchainError> > >", "constructor": "enum2$", "arguments": ["core::result::Result<rhapsody::raw::swapchain::Swapchain,enum2$<rhapsody::raw::swapchain::SwapchainError> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::result::Result<rhapsody::raw::swapchain::Swapchain,enum2$<rhapsody::raw::swapchain::SwapchainError> > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::result::Result<rhapsody::raw::swapchain::Swapchain,enum2$<rhapsody::raw::swapchain::SwapchainError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::result::Result<rhapsody::raw::swapchain::SwapchainTextureLease,enum2$<rhapsody::raw::swapchain::SwapchainError> >", "constructor": "core::result::Result", "arguments": ["rhapsody::raw::swapchain::SwapchainTextureLease", "enum2$<rhapsody::raw::swapchain::SwapchainError>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::result::Result<rhapsody::raw::swapchain::SwapchainTextureLease,enum2$<rhapsody::raw::swapchain::SwapchainError> > >", "constructor": "enum2$", "arguments": ["core::result::Result<rhapsody::raw::swapchain::SwapchainTextureLease,enum2$<rhapsody::raw::swapchain::SwapchainError> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::result::Result<rhapsody::raw::swapchain::SwapchainTextureLease,enum2$<rhapsody::raw::swapchain::SwapchainError> > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::result::Result<rhapsody::raw::swapchain::SwapchainTextureLease,enum2$<rhapsody::raw::swapchain::SwapchainError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::raw::swapchain::SwapchainTextureLease
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 2
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<rhapsody::raw::swapchain::SwapchainTextureLease>", "constructor": "core::ptr::drop_in_place", "arguments": ["rhapsody::raw::swapchain::SwapchainTextureLease"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::result::Result<rhapsody::raw::swapchain::SwapchainTextureLease,enum2$<rhapsody::raw::swapchain::SwapchainError> >", "constructor": "core::result::Result", "arguments": ["rhapsody::raw::swapchain::SwapchainTextureLease", "enum2$<rhapsody::raw::swapchain::SwapchainError>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::result::Result<rhapsody::raw::swapchain::SwapchainTextureLease,enum2$<rhapsody::raw::swapchain::SwapchainError> > >", "constructor": "enum2$", "arguments": ["core::result::Result<rhapsody::raw::swapchain::SwapchainTextureLease,enum2$<rhapsody::raw::swapchain::SwapchainError> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::result::Result<rhapsody::raw::swapchain::SwapchainTextureLease,enum2$<rhapsody::raw::swapchain::SwapchainError> > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::result::Result<rhapsody::raw::swapchain::SwapchainTextureLease,enum2$<rhapsody::raw::swapchain::SwapchainError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: swapchain.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: swapchain.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::create_swapchain
// 信息 ID: 13945ef373ab5482b79b88dd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::run_game::run_game @ 0x3c805a -> 0x743de0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::run_game::run_game @ 0x3c823c -> 0x743de0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::run_game::run_game @ 0x3c85dd -> 0x743de0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::executor::validate_swapchain_and_acquire_image @ 0x73706b -> 0x743de0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::swapchain::Swapchain::acquire_next_texture
// 信息 ID: 74ac0b05f8cb11390f72ff07
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::executor::validate_swapchain_and_acquire_image @ 0x736df9 -> 0x737f10 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::executor::validate_swapchain_and_acquire_image @ 0x7370a7 -> 0x737f10 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::swapchain::Swapchain::any_present_successful
// 信息 ID: 8bbcb33ef59795835379eb47
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::game_loop::GameLoop::main_loop @ 0x24c2f5 -> 0x7380d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::swapchain::Swapchain::enumerate_surface_formats
// 信息 ID: 3dc93649b2d3df0806833f0b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::run_game::run_game @ 0x3c7e96 -> 0x737a10 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::swapchain::Swapchain::extent
// 信息 ID: ccb47ff54613fd02949c66e6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::game_loop::GameLoop::main_loop @ 0x24a83e -> 0x737e00 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::render_loop::render @ 0x30217d -> 0x737e00 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::swapchain::Swapchain::format
// 信息 ID: df0cf439832587621e2cb585
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::render @ 0x302165 -> 0x737b70 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::swapchain::Swapchain::mark_stale
// 信息 ID: 4764c16e879b86df878c6ed9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::swapchain::Swapchain::on_focus
// 信息 ID: 08cfbc06affd064cfb6a56ea
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::swapchain::Swapchain::present
// 信息 ID: 208c5409ee52328dad956785
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::executor::GraphExecutor::execute<country_core::render::pass_builder::pass_resources::PassResources> @ 0x2e0339 -> 0x7380e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::swapchain::Swapchain::resize
// 信息 ID: c40a17e7eddfc33edf3c7e87
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::game_loop::GameLoop::main_loop @ 0x24a858 -> 0x737b80 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::swapchain::Swapchain::set_vsync
// 信息 ID: 3a56995c586d81f6e6e8a7b1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::game_loop::GameLoop::main_loop @ 0x24c69f -> 0x737c30 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::swapchain::impl$1::create_swapchain::closure$0
// 信息 ID: 7eb672419b62d8911732c07c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "rhapsody::raw::swapchain::impl$1::create_swapchain", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::create_swapchain; module=570, inline_record=78156; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::swapchain::impl$5::fmt
// 信息 ID: 46224a871ffac3cd259a4c6c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::fmt::impl$51::fmt<rhapsody::raw::swapchain::SwapchainDesc>; module=219, inline_record=6136; 不是独立调用边。
// [PDB 内联宿主] core::fmt::impl$51::fmt<rhapsody::raw::swapchain::SwapchainDesc>; module=1975, inline_record=500; 不是独立调用边。
// [PDB 内联宿主] core::fmt::impl$51::fmt<rhapsody::raw::swapchain::SwapchainDesc>; module=3027, inline_record=496; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::swapchain::impl$6::fmt
// 信息 ID: 3e864d73bc88a9a0c3d4231b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::fmt::impl$51::fmt<enum2$<rhapsody::raw::swapchain::SwapchainError> >; module=219, inline_record=6336; 不是独立调用边。
// [PDB 内联宿主] core::fmt::impl$51::fmt<enum2$<rhapsody::raw::swapchain::SwapchainError> >; module=1975, inline_record=700; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===

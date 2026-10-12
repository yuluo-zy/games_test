// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/systems/wall-constructor/src/construct_fences/mod.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 5546b516aed62c4d9f64a9d7e52e459d074d0e9f1054e208edfaafa0c57bcd2b
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: mod.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 398, 1454
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
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.27"}]
// use parking_lot; // namespace candidate
//   package identities: [{"name":"parking_lot","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.11.2"},{"name":"parking_lot","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.1"}]
// use parking_lot_core; // namespace candidate
//   package identities: [{"name":"parking_lot_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.8.6"},{"name":"parking_lot_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.9.3"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.1"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_fences::impl$0::default
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 15
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11599
// 首条位置: {"module":1454,"inlinee":71065}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_fences::construct_fences
// 记录形式: compiled_procedure; instances=1
// 源码行: 28
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":398,"record_offset":1660,"rva":7755824,"va":5376464944}
// 编译实例 RVAs: 0x765830
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t6177784","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376464944,"raw_hex":"e8f8435e00","size":5,"soft_fail":false,"target_va":5382644528,"va":5376466739,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b0d70
//     owner=unresolved_namespace; count=1
//     target_aliases=7 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1789622","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376464944,"raw_hex":"e84ab1e4ff","size":5,"soft_fail":false,"target_va":5374676336,"va":5376465953,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405dec30
//     owner=unresolved_namespace; count=1
//     target_aliases=7 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1602970","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376464944,"raw_hex":"e8668ae7ff","size":5,"soft_fail":false,"target_va":5374864432,"va":5376467397,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1384933","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376464944,"raw_hex":"e81bdeeaff","size":5,"soft_fail":false,"target_va":5375081504,"va":5376466432,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067dd80
//     owner=unresolved_namespace; count=1
//     target_aliases=69 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-950587","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376464944,"raw_hex":"e8c57ef1ff","size":5,"soft_fail":false,"target_va":5375516032,"va":5376466614,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141209c40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11157912","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376464944,"raw_hex":"e89841aa00","size":5,"soft_fail":false,"target_va":5387623488,"va":5376465571,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1412388f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11348751","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376464944,"raw_hex":"e80f2bad00","size":5,"soft_fail":false,"target_va":5387815152,"va":5376466396,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923bf0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t35381506","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376464944,"raw_hex":"e802e11b02","size":5,"soft_fail":false,"target_va":5411847152,"va":5376465641,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t35074617","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376464944,"raw_hex":"e839321702","size":5,"soft_fail":false,"target_va":5411541584,"va":5376466962,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t35075238","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376464944,"raw_hex":"e8a6341702","size":5,"soft_fail":false,"target_va":5411541808,"va":5376466565,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t35074797","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376464944,"raw_hex":"e8ed321702","size":5,"soft_fail":false,"target_va":5411541776,"va":5376466974,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t35015480","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376464944,"raw_hex":"e8384b1602","size":5,"soft_fail":false,"target_va":5411482433,"va":5376466948,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-1158","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376464944,"raw_hex":"e87afbffff","size":5,"soft_fail":false,"target_va":5376463872,"va":5376465025,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t35078253","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376464944,"raw_hex":"e86d401702","size":5,"soft_fail":false,"target_va":5411543360,"va":5376465102,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::material_stats::MaterialStats::set_fences_segments_count
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3287022","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376464944,"raw_hex":"e8ee273200","size":5,"soft_fail":false,"target_va":5379753504,"va":5376466477,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::material_stats::MaterialStats::set_fences_segments_seed
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3287027","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376464944,"raw_hex":"e8f3273200","size":5,"soft_fail":false,"target_va":5379753520,"va":5376466488,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t35025842","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376464944,"raw_hex":"e8b2731602","size":5,"soft_fail":false,"target_va":5411492368,"va":5376466521,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t35028256","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376464944,"raw_hex":"e8207d1602","size":5,"soft_fail":false,"target_va":5411494800,"va":5376466539,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::acquire_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t5357404","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376464944,"raw_hex":"e85cbf5100","size":5,"soft_fail":false,"target_va":5381823600,"va":5376466191,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t5357563","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376464944,"raw_hex":"e8fbbf5100","size":5,"soft_fail":false,"target_va":5381824016,"va":5376466448,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4920005","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376464944,"raw_hex":"e8c5124b00","size":5,"soft_fail":false,"target_va":5381385536,"va":5376465526,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4925194","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376464944,"raw_hex":"e80a274b00","size":5,"soft_fail":false,"target_va":5381390640,"va":5376465441,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4919666","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376464944,"raw_hex":"e872114b00","size":5,"soft_fail":false,"target_va":5381386480,"va":5376466809,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4919700","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376464944,"raw_hex":"e894114b00","size":5,"soft_fail":false,"target_va":5381384704,"va":5376464999,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::native::lazy::Storage$LT$T$C$D$GT$::initialize
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t35173905","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376464944,"raw_hex":"e811b61802","size":5,"soft_fail":false,"target_va":5411640768,"va":5376466858,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t35070188","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376464944,"raw_hex":"e8ec201702","size":5,"soft_fail":false,"target_va":5411537104,"va":5376466911,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*24(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5376464944,"raw_hex":"ff5018","size":3,"soft_fail":false,"va":5376465661,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::event::iterators::impl$0::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":398,"record_offset":5272,"parent_record_offset":1660,"ancestor_record_offset":1660,"inlinee":12537,"source_line":14,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::reader::EventReader::read
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/reader.rs
//     evidence={"module":398,"record_offset":4560,"parent_record_offset":1660,"ancestor_record_offset":1660,"inlinee":8034,"source_line":27,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":398,"record_offset":7664,"parent_record_offset":1660,"ancestor_record_offset":1660,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":398,"record_offset":4524,"parent_record_offset":1660,"ancestor_record_offset":1660,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":398,"record_offset":7128,"parent_record_offset":1660,"ancestor_record_offset":1660,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":398,"record_offset":4284,"parent_record_offset":1660,"ancestor_record_offset":1660,"inlinee":4445,"source_line":242,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::sparse_instance_buffer::SparseInstanceBuffer::write_allocation
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":398,"record_offset":6692,"parent_record_offset":1660,"ancestor_record_offset":1660,"inlinee":12277,"source_line":158,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":398,"record_offset":7696,"parent_record_offset":1660,"ancestor_record_offset":1660,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":398,"record_offset":7580,"parent_record_offset":1660,"ancestor_record_offset":1660,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":398,"record_offset":3300,"parent_record_offset":1660,"ancestor_record_offset":1660,"inlinee":4416,"source_line":589,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":398,"record_offset":1852,"parent_record_offset":1660,"ancestor_record_offset":1660,"inlinee":4368,"source_line":626,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":398,"record_offset":2312,"parent_record_offset":1660,"ancestor_record_offset":1660,"inlinee":4385,"source_line":659,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$9::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":398,"record_offset":5612,"parent_record_offset":1660,"ancestor_record_offset":1660,"inlinee":11287,"source_line":1426,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":398,"record_offset":7340,"parent_record_offset":7316,"ancestor_record_offset":1660,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":398,"record_offset":7316,"parent_record_offset":7224,"ancestor_record_offset":1660,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":398,"record_offset":7252,"parent_record_offset":7224,"ancestor_record_offset":1660,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":398,"record_offset":7224,"parent_record_offset":7200,"ancestor_record_offset":1660,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":398,"record_offset":7200,"parent_record_offset":7176,"ancestor_record_offset":1660,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":398,"record_offset":4900,"parent_record_offset":4876,"ancestor_record_offset":1660,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":398,"record_offset":4876,"parent_record_offset":4648,"ancestor_record_offset":1660,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::event_cursor::EventCursor::read_with_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/event_cursor.rs
//     evidence={"module":398,"record_offset":4624,"parent_record_offset":4584,"ancestor_record_offset":1660,"inlinee":8037,"source_line":88,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::iterators::EventIteratorWithId::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":398,"record_offset":4648,"parent_record_offset":4624,"ancestor_record_offset":1660,"inlinee":8039,"source_line":54,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::iterators::impl$3::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":398,"record_offset":5296,"parent_record_offset":5272,"ancestor_record_offset":1660,"inlinee":12538,"source_line":86,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$19::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/system_param.rs
//     evidence={"module":398,"record_offset":4584,"parent_record_offset":4560,"ancestor_record_offset":1660,"inlinee":10713,"source_line":1234,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":398,"record_offset":3636,"parent_record_offset":3608,"ancestor_record_offset":1660,"inlinee":4432,"source_line":1817,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":398,"record_offset":3688,"parent_record_offset":3664,"ancestor_record_offset":1660,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":398,"record_offset":3664,"parent_record_offset":3636,"ancestor_record_offset":1660,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":398,"record_offset":3584,"parent_record_offset":3556,"ancestor_record_offset":1660,"inlinee":4429,"source_line":1089,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":398,"record_offset":3608,"parent_record_offset":3584,"ancestor_record_offset":1660,"inlinee":4430,"source_line":1121,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":398,"record_offset":3780,"parent_record_offset":3608,"ancestor_record_offset":1660,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$44::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":398,"record_offset":3892,"parent_record_offset":3868,"ancestor_record_offset":1660,"inlinee":4443,"source_line":1808,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":398,"record_offset":4352,"parent_record_offset":4312,"ancestor_record_offset":1660,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":398,"record_offset":5952,"parent_record_offset":5928,"ancestor_record_offset":1660,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":398,"record_offset":6036,"parent_record_offset":6012,"ancestor_record_offset":1660,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":398,"record_offset":5976,"parent_record_offset":5952,"ancestor_record_offset":1660,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":398,"record_offset":6512,"parent_record_offset":5816,"ancestor_record_offset":1660,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::and_then_or_clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":398,"record_offset":5348,"parent_record_offset":5324,"ancestor_record_offset":1660,"inlinee":5541,"source_line":332,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":398,"record_offset":5324,"parent_record_offset":5296,"ancestor_record_offset":1660,"inlinee":4570,"source_line":83,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":398,"record_offset":5428,"parent_record_offset":5404,"ancestor_record_offset":1660,"inlinee":4573,"source_line":84,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":398,"record_offset":3712,"parent_record_offset":3688,"ancestor_record_offset":1660,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":398,"record_offset":4832,"parent_record_offset":4804,"ancestor_record_offset":1660,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":398,"record_offset":4692,"parent_record_offset":4648,"ancestor_record_offset":1660,"inlinee":8040,"source_line":1859,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":398,"record_offset":7284,"parent_record_offset":7252,"ancestor_record_offset":1660,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":398,"record_offset":2004,"parent_record_offset":1972,"ancestor_record_offset":1660,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":398,"record_offset":6184,"parent_record_offset":6156,"ancestor_record_offset":1660,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":398,"record_offset":6156,"parent_record_offset":6128,"ancestor_record_offset":1660,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":398,"record_offset":3452,"parent_record_offset":3408,"ancestor_record_offset":1660,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":398,"record_offset":5372,"parent_record_offset":5348,"ancestor_record_offset":1660,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":398,"record_offset":6960,"parent_record_offset":6692,"ancestor_record_offset":1660,"inlinee":4665,"source_line":1892,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":398,"record_offset":5404,"parent_record_offset":5324,"ancestor_record_offset":1660,"inlinee":4571,"source_line":1546,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":398,"record_offset":6772,"parent_record_offset":6692,"ancestor_record_offset":1660,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":398,"record_offset":6996,"parent_record_offset":6692,"ancestor_record_offset":1660,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":398,"record_offset":7752,"parent_record_offset":7720,"ancestor_record_offset":1660,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":398,"record_offset":7780,"parent_record_offset":7720,"ancestor_record_offset":1660,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":398,"record_offset":2260,"parent_record_offset":2232,"ancestor_record_offset":1660,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":398,"record_offset":4240,"parent_record_offset":3408,"ancestor_record_offset":1660,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":398,"record_offset":3844,"parent_record_offset":3556,"ancestor_record_offset":1660,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":398,"record_offset":5164,"parent_record_offset":5140,"ancestor_record_offset":1660,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":398,"record_offset":6304,"parent_record_offset":6280,"ancestor_record_offset":1660,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":398,"record_offset":5552,"parent_record_offset":5528,"ancestor_record_offset":1660,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":398,"record_offset":2728,"parent_record_offset":2704,"ancestor_record_offset":1660,"inlinee":4410,"source_line":533,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":398,"record_offset":2704,"parent_record_offset":2680,"ancestor_record_offset":1660,"inlinee":4409,"source_line":686,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":398,"record_offset":2912,"parent_record_offset":2884,"ancestor_record_offset":1660,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":398,"record_offset":3736,"parent_record_offset":3712,"ancestor_record_offset":1660,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":398,"record_offset":4436,"parent_record_offset":4408,"ancestor_record_offset":1660,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":398,"record_offset":4464,"parent_record_offset":4436,"ancestor_record_offset":1660,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":398,"record_offset":4748,"parent_record_offset":4648,"ancestor_record_offset":1660,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":398,"record_offset":5116,"parent_record_offset":4648,"ancestor_record_offset":1660,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::last
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":398,"record_offset":6932,"parent_record_offset":6692,"ancestor_record_offset":1660,"inlinee":4716,"source_line":279,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":398,"record_offset":4804,"parent_record_offset":4776,"ancestor_record_offset":1660,"inlinee":8042,"source_line":364,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":398,"record_offset":4776,"parent_record_offset":4748,"ancestor_record_offset":1660,"inlinee":8041,"source_line":537,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":398,"record_offset":5140,"parent_record_offset":5116,"ancestor_record_offset":1660,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":398,"record_offset":2656,"parent_record_offset":2632,"ancestor_record_offset":1660,"inlinee":4407,"source_line":80,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":398,"record_offset":5528,"parent_record_offset":5504,"ancestor_record_offset":1660,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":398,"record_offset":2680,"parent_record_offset":2656,"ancestor_record_offset":1660,"inlinee":4408,"source_line":118,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":398,"record_offset":2984,"parent_record_offset":2956,"ancestor_record_offset":1660,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":398,"record_offset":5480,"parent_record_offset":5428,"ancestor_record_offset":1660,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$183::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":398,"record_offset":2632,"parent_record_offset":2516,"ancestor_record_offset":1660,"inlinee":4405,"source_line":392,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":398,"record_offset":5504,"parent_record_offset":5480,"ancestor_record_offset":1660,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":398,"record_offset":1972,"parent_record_offset":1944,"ancestor_record_offset":1660,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":398,"record_offset":2344,"parent_record_offset":2312,"ancestor_record_offset":1660,"inlinee":4386,"source_line":1488,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":398,"record_offset":2484,"parent_record_offset":2452,"ancestor_record_offset":1660,"inlinee":4396,"source_line":128,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":398,"record_offset":2452,"parent_record_offset":2416,"ancestor_record_offset":1660,"inlinee":4394,"source_line":218,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":398,"record_offset":2392,"parent_record_offset":2368,"ancestor_record_offset":1660,"inlinee":4390,"source_line":315,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":398,"record_offset":2416,"parent_record_offset":2392,"ancestor_record_offset":1660,"inlinee":4392,"source_line":709,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":398,"record_offset":2368,"parent_record_offset":2344,"ancestor_record_offset":1660,"inlinee":4388,"source_line":798,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":398,"record_offset":2956,"parent_record_offset":2416,"ancestor_record_offset":1660,"inlinee":4400,"source_line":659,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":398,"record_offset":1944,"parent_record_offset":1912,"ancestor_record_offset":1660,"inlinee":4373,"source_line":433,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":398,"record_offset":1912,"parent_record_offset":1888,"ancestor_record_offset":1660,"inlinee":4372,"source_line":463,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":398,"record_offset":2112,"parent_record_offset":2076,"ancestor_record_offset":1660,"inlinee":4380,"source_line":501,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":398,"record_offset":2208,"parent_record_offset":2112,"ancestor_record_offset":1660,"inlinee":4381,"source_line":521,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":398,"record_offset":2076,"parent_record_offset":2048,"ancestor_record_offset":1660,"inlinee":4379,"source_line":533,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":398,"record_offset":4380,"parent_record_offset":4352,"ancestor_record_offset":1660,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":398,"record_offset":1888,"parent_record_offset":1852,"ancestor_record_offset":1660,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":398,"record_offset":2232,"parent_record_offset":2208,"ancestor_record_offset":1660,"inlinee":4383,"source_line":189,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":398,"record_offset":2516,"parent_record_offset":2484,"ancestor_record_offset":1660,"inlinee":4398,"source_line":79,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":398,"record_offset":2800,"parent_record_offset":2516,"ancestor_record_offset":1660,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":398,"record_offset":2772,"parent_record_offset":2516,"ancestor_record_offset":1660,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":398,"record_offset":2576,"parent_record_offset":2516,"ancestor_record_offset":1660,"inlinee":4403,"source_line":24,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":398,"record_offset":7512,"parent_record_offset":7448,"ancestor_record_offset":1660,"inlinee":4945,"source_line":2871,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":398,"record_offset":6844,"parent_record_offset":6808,"ancestor_record_offset":1660,"inlinee":10004,"source_line":2920,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":398,"record_offset":7604,"parent_record_offset":7580,"ancestor_record_offset":1660,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":398,"record_offset":7536,"parent_record_offset":7512,"ancestor_record_offset":1660,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":398,"record_offset":6868,"parent_record_offset":6844,"ancestor_record_offset":1660,"inlinee":5825,"source_line":3828,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":398,"record_offset":7628,"parent_record_offset":7604,"ancestor_record_offset":1660,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":398,"record_offset":4312,"parent_record_offset":4284,"ancestor_record_offset":1660,"inlinee":4446,"source_line":206,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::sparse_instance_buffer::impl$2::write_allocation::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":398,"record_offset":7024,"parent_record_offset":6996,"ancestor_record_offset":1660,"inlinee":12261,"source_line":175,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::sparse_instance_buffer::impl$5::drop
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":398,"record_offset":7420,"parent_record_offset":7392,"ancestor_record_offset":1660,"inlinee":4939,"source_line":445,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$9::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":398,"record_offset":6468,"parent_record_offset":6444,"ancestor_record_offset":1660,"inlinee":4919,"source_line":69,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":398,"record_offset":6096,"parent_record_offset":6068,"ancestor_record_offset":1660,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":398,"record_offset":6128,"parent_record_offset":6096,"ancestor_record_offset":1660,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":398,"record_offset":6068,"parent_record_offset":5816,"ancestor_record_offset":1660,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":398,"record_offset":5928,"parent_record_offset":5816,"ancestor_record_offset":1660,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":398,"record_offset":6540,"parent_record_offset":5816,"ancestor_record_offset":1660,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":398,"record_offset":6012,"parent_record_offset":5816,"ancestor_record_offset":1660,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":398,"record_offset":5900,"parent_record_offset":5816,"ancestor_record_offset":1660,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\lib.rs
//     evidence={"module":398,"record_offset":6420,"parent_record_offset":6396,"ancestor_record_offset":1660,"inlinee":4499,"source_line":164,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":398,"record_offset":5680,"parent_record_offset":5644,"ancestor_record_offset":1660,"inlinee":4719,"source_line":1304,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":398,"record_offset":5704,"parent_record_offset":5680,"ancestor_record_offset":1660,"inlinee":4475,"source_line":1347,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":398,"record_offset":6396,"parent_record_offset":6228,"ancestor_record_offset":1660,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":398,"record_offset":5736,"parent_record_offset":5704,"ancestor_record_offset":1660,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":398,"record_offset":6336,"parent_record_offset":6252,"ancestor_record_offset":1660,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":398,"record_offset":6280,"parent_record_offset":6252,"ancestor_record_offset":1660,"inlinee":5616,"source_line":261,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":398,"record_offset":6632,"parent_record_offset":5816,"ancestor_record_offset":1660,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":398,"record_offset":6252,"parent_record_offset":6228,"ancestor_record_offset":1660,"inlinee":5615,"source_line":734,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":398,"record_offset":5788,"parent_record_offset":5764,"ancestor_record_offset":1660,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":398,"record_offset":5764,"parent_record_offset":5704,"ancestor_record_offset":1660,"inlinee":4477,"source_line":1179,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":398,"record_offset":5816,"parent_record_offset":5788,"ancestor_record_offset":1660,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":398,"record_offset":5872,"parent_record_offset":5816,"ancestor_record_offset":1660,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":398,"record_offset":6228,"parent_record_offset":5816,"ancestor_record_offset":1660,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":398,"record_offset":7808,"parent_record_offset":7720,"ancestor_record_offset":1660,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":398,"record_offset":7720,"parent_record_offset":7696,"ancestor_record_offset":1660,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":398,"record_offset":7832,"parent_record_offset":7808,"ancestor_record_offset":1660,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":398,"record_offset":6808,"parent_record_offset":6692,"ancestor_record_offset":1660,"inlinee":10003,"source_line":66,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":398,"record_offset":7448,"parent_record_offset":7420,"ancestor_record_offset":1660,"inlinee":4941,"source_line":100,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":398,"record_offset":6900,"parent_record_offset":6808,"ancestor_record_offset":1660,"inlinee":5827,"source_line":1104,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":398,"record_offset":7484,"parent_record_offset":7448,"ancestor_record_offset":1660,"inlinee":4943,"source_line":1120,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":398,"record_offset":3336,"parent_record_offset":3300,"ancestor_record_offset":1660,"inlinee":4418,"source_line":425,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":398,"record_offset":7936,"parent_record_offset":7904,"ancestor_record_offset":1660,"inlinee":4966,"source_line":601,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":398,"record_offset":8436,"parent_record_offset":8176,"ancestor_record_offset":1660,"inlinee":4968,"source_line":602,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":398,"record_offset":3480,"parent_record_offset":3452,"ancestor_record_offset":1660,"inlinee":4423,"source_line":90,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":398,"record_offset":3556,"parent_record_offset":3408,"ancestor_record_offset":1660,"inlinee":4427,"source_line":429,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":398,"record_offset":3816,"parent_record_offset":3556,"ancestor_record_offset":1660,"inlinee":4441,"source_line":591,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":398,"record_offset":5644,"parent_record_offset":5612,"ancestor_record_offset":1660,"inlinee":4718,"source_line":894,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":398,"record_offset":3508,"parent_record_offset":3480,"ancestor_record_offset":1660,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":398,"record_offset":3408,"parent_record_offset":3372,"ancestor_record_offset":1660,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":398,"record_offset":3372,"parent_record_offset":3336,"ancestor_record_offset":1660,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/00398.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_fences::construct_fences::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 47
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1159F
// 首条位置: {"module":1454,"inlinee":71071}
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
// 完整结构及原始类型记录: mod.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: mod.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: mod.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)
// 候选参数类型: ["bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>"]
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_fences::construct_fences
// 信息 ID: 5fa326ae0b7ef5571bd05c0b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 1796e4e99f081b3499152d7e: void (*)(bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),void (*)(bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)> > @ 0x336b43 -> 0x765830 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),void (*)(bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)> > > @ 0x456ef1 -> 0x765830 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_fences::construct_fences::closure$0
// 信息 ID: 8241e0ea16142801370b1738
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_wall_constructor::construct_fences::construct_fences", "captures": "unknown"}
// [闭包环境类型引用] system_wall_constructor::construct_fences::construct_fences::closure_env$0 <- utils::threshold_range::threshold_range<system_wall_constructor::construct_fences::process_segment::emit_fences::closure_env$0<system_wall_constructor::construct_fences::construct_fences::closure_env$0>,system_wall_constructor::construct_fences::process_segment::emit_fences::closure_env$1<system_wall_constructor::construct_fences::construct_fences::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::construct_fences::construct_fences::closure_env$0 <- utils::threshold_range::threshold_range<system_wall_constructor::construct_fences::process_segment::emit_fences::closure_env$0<system_wall_constructor::construct_fences::construct_fences::closure_env$0>,system_wall_constructor::construct_fences::process_segment::emit_fences::closure_env$1<system_wall_constructor::construct_fences::construct_fences::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::construct_fences::construct_fences::closure_env$0 <- system_wall_constructor::construct_fences::process_segment::emit_fences<system_wall_constructor::construct_fences::construct_fences::closure_env$0>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::construct_fences::construct_fences::closure_env$0 <- system_wall_constructor::construct_fences::process_segment::process_segment<system_wall_constructor::construct_fences::construct_fences::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 4 条。
// [PDB 内联宿主] system_wall_constructor::construct_fences::process_segment::process_segment<system_wall_constructor::construct_fences::construct_fences::closure_env$0>; module=1454, inline_record=13536; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_fences::process_segment::process_segment<system_wall_constructor::construct_fences::construct_fences::closure_env$0>; module=1454, inline_record=16928; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_fences::impl$0::default
// 信息 ID: 94eb4e4d23804d90bff88541
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_fences::process_segment::emit_fences<system_wall_constructor::construct_fences::construct_fences::closure_env$0>; module=1454, inline_record=4728; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===

// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/components/transient_mesh.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 87ee798dee589db6a7b7bbac14d647498fb87e62834d89fe77b6b3bf20ab135d
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: transient_mesh.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 485, 486, 562, 1582, 1605
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use bevy_platform; // namespace candidate
//   package identities: [{"name":"bevy_platform","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use equivalent; // namespace candidate
//   package identities: [{"name":"equivalent","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.1"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.27"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::components::transient_mesh::TransientMeshRegistry::drain_meshes_to_delete<country_core::systems::backend::transient_mesh::delete_dropped_transient_meshes::closure_env$0>
// 记录形式: compiled_procedure; instances=1
// 源码行: 12
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1582,"record_offset":82792,"rva":19832432,"va":5388541552}
// 编译实例 RVAs: 0x12e9e70
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-5897362","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388541552,"raw_hex":"e86e03a6ff","size":5,"soft_fail":false,"target_va":5382644528,"va":5388541885,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140043000
//     owner=unresolved_namespace; count=1
//     target_aliases=30 candidates; complete list: transient_mesh.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-19558653","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388541552,"raw_hex":"e8038fd5fe","size":5,"soft_fail":false,"target_va":5368983552,"va":5388542200,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405d2870
//     owner=unresolved_namespace; count=1
//     target_aliases=102 candidates; complete list: transient_mesh.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13727962","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388541552,"raw_hex":"e826872eff","size":5,"soft_fail":false,"target_va":5374814320,"va":5388542277,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067dd80
//     owner=unresolved_namespace; count=1
//     target_aliases=69 candidates; complete list: transient_mesh.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13025736","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388541552,"raw_hex":"e8383e39ff","size":5,"soft_fail":false,"target_va":5375516032,"va":5388541763,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t23000764","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388541552,"raw_hex":"e8bcf65e01","size":5,"soft_fail":false,"target_va":5411542880,"va":5388542111,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::mesh_library::AssetMeshLibrary::remove
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-10209414","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388541552,"raw_hex":"e87a3764ff","size":5,"soft_fail":false,"target_va":5378332496,"va":5388541905,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::vao_library::AssetVAOLibrary::remove
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-10246212","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388541552,"raw_hex":"e8bca763ff","size":5,"soft_fail":false,"target_va":5378295712,"va":5388541919,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::panicking::panic_count::is_zero_slow_path
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t22996559","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388541552,"raw_hex":"e84fe65e01","size":5,"soft_fail":false,"target_va":5411538608,"va":5388542044,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::mutex::futex::Mutex::lock_contended
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t22996741","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388541552,"raw_hex":"e805e75e01","size":5,"soft_fail":false,"target_va":5411538768,"va":5388542022,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::mutex::futex::Mutex::wake
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t22996834","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388541552,"raw_hex":"e862e75e01","size":5,"soft_fail":false,"target_va":5411538960,"va":5388542121,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1582,"record_offset":84104,"parent_record_offset":82792,"ancestor_record_offset":82792,"inlinee":4586,"source_line":2652,"archive":"_pdb/B/modules/01582.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1582,"record_offset":84132,"parent_record_offset":82792,"ancestor_record_offset":82792,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01582.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1582,"record_offset":83536,"parent_record_offset":82792,"ancestor_record_offset":82792,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/01582.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::backend::transient_mesh::delete_dropped_transient_meshes::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/backend/transient_mesh.rs
//     evidence={"module":1582,"record_offset":83580,"parent_record_offset":82792,"ancestor_record_offset":82792,"inlinee":72241,"source_line":16,"archive":"_pdb/B/modules/01582.jsonl.gz"}
//   [pdb_inline_direct] std::sync::poison::mutex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\mutex.rs
//     evidence={"module":1582,"record_offset":83076,"parent_record_offset":82792,"ancestor_record_offset":82792,"inlinee":6007,"source_line":435,"archive":"_pdb/B/modules/01582.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":1582,"record_offset":84012,"parent_record_offset":83988,"ancestor_record_offset":82792,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/01582.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":1582,"record_offset":83988,"parent_record_offset":83580,"ancestor_record_offset":82792,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/01582.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1582,"record_offset":83780,"parent_record_offset":83580,"ancestor_record_offset":82792,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/01582.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":1582,"record_offset":83868,"parent_record_offset":83836,"ancestor_record_offset":82792,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/01582.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":1582,"record_offset":83896,"parent_record_offset":83836,"ancestor_record_offset":82792,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/01582.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1582,"record_offset":83660,"parent_record_offset":83620,"ancestor_record_offset":82792,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/01582.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1582,"record_offset":83620,"parent_record_offset":83580,"ancestor_record_offset":82792,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/01582.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1582,"record_offset":83456,"parent_record_offset":83428,"ancestor_record_offset":82792,"inlinee":5623,"source_line":623,"archive":"_pdb/B/modules/01582.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1582,"record_offset":84384,"parent_record_offset":84196,"ancestor_record_offset":82792,"inlinee":5912,"source_line":651,"archive":"_pdb/B/modules/01582.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1582,"record_offset":83144,"parent_record_offset":83112,"ancestor_record_offset":82792,"inlinee":4945,"source_line":2871,"archive":"_pdb/B/modules/01582.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1582,"record_offset":84476,"parent_record_offset":84444,"ancestor_record_offset":82792,"inlinee":5910,"source_line":2753,"archive":"_pdb/B/modules/01582.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1582,"record_offset":83360,"parent_record_offset":83328,"ancestor_record_offset":82792,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/01582.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1582,"record_offset":83168,"parent_record_offset":83144,"ancestor_record_offset":82792,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/01582.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1582,"record_offset":83384,"parent_record_offset":83360,"ancestor_record_offset":82792,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/01582.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1582,"record_offset":84408,"parent_record_offset":84384,"ancestor_record_offset":82792,"inlinee":5913,"source_line":3712,"archive":"_pdb/B/modules/01582.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1582,"record_offset":84500,"parent_record_offset":84476,"ancestor_record_offset":82792,"inlinee":5911,"source_line":3743,"archive":"_pdb/B/modules/01582.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1582,"record_offset":83924,"parent_record_offset":83836,"ancestor_record_offset":82792,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/01582.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1582,"record_offset":83812,"parent_record_offset":83580,"ancestor_record_offset":82792,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/01582.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1582,"record_offset":83836,"parent_record_offset":83812,"ancestor_record_offset":82792,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/01582.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1582,"record_offset":83948,"parent_record_offset":83924,"ancestor_record_offset":82792,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/01582.jsonl.gz"}
//   [pdb_inline_transitive] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1582,"record_offset":83696,"parent_record_offset":83580,"ancestor_record_offset":82792,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/01582.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\panicking.rs
//     evidence={"module":1582,"record_offset":83328,"parent_record_offset":83300,"ancestor_record_offset":82792,"inlinee":5907,"source_line":469,"archive":"_pdb/B/modules/01582.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\panicking.rs
//     evidence={"module":1582,"record_offset":83300,"parent_record_offset":83272,"ancestor_record_offset":82792,"inlinee":5905,"source_line":621,"archive":"_pdb/B/modules/01582.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::done
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":1582,"record_offset":84196,"parent_record_offset":84160,"ancestor_record_offset":82792,"inlinee":5901,"source_line":132,"archive":"_pdb/B/modules/01582.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":1582,"record_offset":83428,"parent_record_offset":83232,"ancestor_record_offset":82792,"inlinee":5621,"source_line":144,"archive":"_pdb/B/modules/01582.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::guard
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":1582,"record_offset":83232,"parent_record_offset":83204,"ancestor_record_offset":82792,"inlinee":6011,"source_line":122,"archive":"_pdb/B/modules/01582.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::MutexGuard::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\mutex.rs
//     evidence={"module":1582,"record_offset":83204,"parent_record_offset":83076,"ancestor_record_offset":82792,"inlinee":6010,"source_line":647,"archive":"_pdb/B/modules/01582.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::impl$14::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\mutex.rs
//     evidence={"module":1582,"record_offset":84160,"parent_record_offset":84132,"ancestor_record_offset":82792,"inlinee":5900,"source_line":671,"archive":"_pdb/B/modules/01582.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":1582,"record_offset":83112,"parent_record_offset":83076,"ancestor_record_offset":82792,"inlinee":6008,"source_line":27,"archive":"_pdb/B/modules/01582.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::unlock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":1582,"record_offset":84444,"parent_record_offset":84160,"ancestor_record_offset":82792,"inlinee":5909,"source_line":83,"archive":"_pdb/B/modules/01582.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\mod.rs
//     evidence={"module":1582,"record_offset":83272,"parent_record_offset":83232,"ancestor_record_offset":82792,"inlinee":5903,"source_line":814,"archive":"_pdb/B/modules/01582.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::components::transient_mesh::TransientMeshRegistry::deferred_delete_mesh
// 记录形式: compiled_procedure; instances=1
// 源码行: 18
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":562,"record_offset":20552,"rva":11174000,"va":5379883120}
// 编译实例 RVAs: 0xaa8070
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140043000
//     owner=unresolved_namespace; count=1
//     target_aliases=30 candidates; complete list: transient_mesh.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10899903","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379883120,"raw_hex":"e841ae59ff","size":5,"soft_fail":false,"target_va":5368983552,"va":5379883450,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405d2870
//     owner=unresolved_namespace; count=1
//     target_aliases=102 candidates; complete list: transient_mesh.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5069180","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379883120,"raw_hex":"e884a6b2ff","size":5,"soft_fail":false,"target_va":5374814320,"va":5379883495,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-5527660","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379883120,"raw_hex":"e894a7abff","size":5,"soft_fail":false,"target_va":5374355568,"va":5379883223,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31659512","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379883120,"raw_hex":"e8f815e301","size":5,"soft_fail":false,"target_va":5411542880,"va":5379883363,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::panicking::panic_count::is_zero_slow_path
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t31655304","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379883120,"raw_hex":"e88805e301","size":5,"soft_fail":false,"target_va":5411538608,"va":5379883299,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::mutex::futex::Mutex::lock_contended
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31655474","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379883120,"raw_hex":"e83206e301","size":5,"soft_fail":false,"target_va":5411538768,"va":5379883289,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] std::sys::sync::mutex::futex::Mutex::wake
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tjmp\t31655572","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5379883120,"raw_hex":"e99406e301","size":5,"soft_fail":false,"target_va":5411538960,"va":5379883383,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":562,"record_offset":21220,"parent_record_offset":20552,"ancestor_record_offset":20552,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":562,"record_offset":21400,"parent_record_offset":20552,"ancestor_record_offset":20552,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":562,"record_offset":21180,"parent_record_offset":20552,"ancestor_record_offset":20552,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_direct] std::sync::poison::mutex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\mutex.rs
//     evidence={"module":562,"record_offset":20736,"parent_record_offset":20552,"ancestor_record_offset":20552,"inlinee":6007,"source_line":435,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":562,"record_offset":21280,"parent_record_offset":21256,"ancestor_record_offset":20552,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":562,"record_offset":21328,"parent_record_offset":21304,"ancestor_record_offset":20552,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":562,"record_offset":21304,"parent_record_offset":21280,"ancestor_record_offset":20552,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":562,"record_offset":21256,"parent_record_offset":21220,"ancestor_record_offset":20552,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":562,"record_offset":21368,"parent_record_offset":21220,"ancestor_record_offset":20552,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":562,"record_offset":21100,"parent_record_offset":21072,"ancestor_record_offset":20552,"inlinee":5623,"source_line":623,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":562,"record_offset":21644,"parent_record_offset":21460,"ancestor_record_offset":20552,"inlinee":5912,"source_line":651,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":562,"record_offset":20796,"parent_record_offset":20768,"ancestor_record_offset":20552,"inlinee":4945,"source_line":2871,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":562,"record_offset":21736,"parent_record_offset":21704,"ancestor_record_offset":20552,"inlinee":5910,"source_line":2753,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":562,"record_offset":21004,"parent_record_offset":20976,"ancestor_record_offset":20552,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":562,"record_offset":20820,"parent_record_offset":20796,"ancestor_record_offset":20552,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":562,"record_offset":21028,"parent_record_offset":21004,"ancestor_record_offset":20552,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":562,"record_offset":21668,"parent_record_offset":21644,"ancestor_record_offset":20552,"inlinee":5913,"source_line":3712,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":562,"record_offset":21760,"parent_record_offset":21736,"ancestor_record_offset":20552,"inlinee":5911,"source_line":3743,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\panicking.rs
//     evidence={"module":562,"record_offset":20976,"parent_record_offset":20948,"ancestor_record_offset":20552,"inlinee":5907,"source_line":469,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\panicking.rs
//     evidence={"module":562,"record_offset":20948,"parent_record_offset":20920,"ancestor_record_offset":20552,"inlinee":5905,"source_line":621,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::done
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":562,"record_offset":21460,"parent_record_offset":21424,"ancestor_record_offset":20552,"inlinee":5901,"source_line":132,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":562,"record_offset":21072,"parent_record_offset":20884,"ancestor_record_offset":20552,"inlinee":5621,"source_line":144,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::guard
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":562,"record_offset":20884,"parent_record_offset":20856,"ancestor_record_offset":20552,"inlinee":6011,"source_line":122,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::MutexGuard::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\mutex.rs
//     evidence={"module":562,"record_offset":20856,"parent_record_offset":20736,"ancestor_record_offset":20552,"inlinee":6010,"source_line":647,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::impl$14::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\mutex.rs
//     evidence={"module":562,"record_offset":21424,"parent_record_offset":21400,"ancestor_record_offset":20552,"inlinee":5900,"source_line":671,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":562,"record_offset":20768,"parent_record_offset":20736,"ancestor_record_offset":20552,"inlinee":6008,"source_line":27,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::unlock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":562,"record_offset":21704,"parent_record_offset":21424,"ancestor_record_offset":20552,"inlinee":5909,"source_line":83,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\mod.rs
//     evidence={"module":562,"record_offset":20920,"parent_record_offset":20884,"ancestor_record_offset":20552,"inlinee":5903,"source_line":814,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::components::transient_mesh::impl$4::clone_behavior
// 记录形式: compiled_procedure; instances=1
// 源码行: 24
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":562,"record_offset":79868,"rva":9072,"va":5368718192}
// 编译实例 RVAs: 0x2370
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::components::transient_mesh::impl$4::register_required_components
// 记录形式: compiled_procedure; instances=1
// 源码行: 24
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":562,"record_offset":75320,"rva":11191872,"va":5379900992}
// 编译实例 RVAs: 0xaac640
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140051340
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: transient_mesh.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10860327","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379900992,"raw_hex":"e8d9485aff","size":5,"soft_fail":false,"target_va":5369041728,"va":5379902050,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405f0b50
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: transient_mesh.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4963939","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379900992,"raw_hex":"e89d41b4ff","size":5,"soft_fail":false,"target_va":5374937936,"va":5379901870,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: transient_mesh.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4820618","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379900992,"raw_hex":"e87671b6ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5379902117,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140aac640
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: transient_mesh.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-997","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379900992,"raw_hex":"e81bfcffff","size":5,"soft_fail":false,"target_va":5379900992,"va":5379901984,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1412f4340
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: transient_mesh.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8681932","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379900992,"raw_hex":"e8cc798400","size":5,"soft_fail":false,"target_va":5388583744,"va":5379901807,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: transient_mesh.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t31945312","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379900992,"raw_hex":"e86072e701","size":5,"soft_fail":false,"target_va":5411847168,"va":5379901851,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t873262","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379900992,"raw_hex":"e82e530d00","size":5,"soft_fail":false,"target_va":5380775408,"va":5379902141,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::component::ComponentInfo::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2699035","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379900992,"raw_hex":"e81b2f2900","size":5,"soft_fail":false,"target_va":5382600816,"va":5379901776,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::component::QueuedRegistration::register
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2698999","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379900992,"raw_hex":"e8f72e2900","size":5,"soft_fail":false,"target_va":5382601280,"va":5379902276,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::component::enforce_no_required_components_recursion
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2705127","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379900992,"raw_hex":"e8e7462900","size":5,"soft_fail":false,"target_va":5382606192,"va":5379901060,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] hashbrown::map::HashMap::insert<core::any::TypeId,bevy_ecs::component::ComponentId,bevy_platform::hash::NoOpHash,allocator_api2::stable::alloc::global::Global>
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-10657228","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379900992,"raw_hex":"e834625dff","size":5,"soft_fail":false,"target_va":5369244704,"va":5379901927,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::pop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":562,"record_offset":79828,"parent_record_offset":75320,"ancestor_record_offset":75320,"inlinee":6530,"source_line":2503,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":562,"record_offset":79632,"parent_record_offset":75320,"ancestor_record_offset":75320,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::component::ComponentsRegistrator::register_component
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/component.rs
//     evidence={"module":562,"record_offset":75492,"parent_record_offset":75320,"ancestor_record_offset":75320,"inlinee":5611,"source_line":1715,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":562,"record_offset":79344,"parent_record_offset":79320,"ancestor_record_offset":75320,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":562,"record_offset":79320,"parent_record_offset":79228,"ancestor_record_offset":75320,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":562,"record_offset":78764,"parent_record_offset":78740,"ancestor_record_offset":75320,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":562,"record_offset":79256,"parent_record_offset":79228,"ancestor_record_offset":75320,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":562,"record_offset":79228,"parent_record_offset":79204,"ancestor_record_offset":75320,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":562,"record_offset":78812,"parent_record_offset":78788,"ancestor_record_offset":75320,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":562,"record_offset":78788,"parent_record_offset":78764,"ancestor_record_offset":75320,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":562,"record_offset":79204,"parent_record_offset":79180,"ancestor_record_offset":75320,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":562,"record_offset":78740,"parent_record_offset":78716,"ancestor_record_offset":75320,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":562,"record_offset":78716,"parent_record_offset":78692,"ancestor_record_offset":75320,"inlinee":5037,"source_line":1593,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":562,"record_offset":75536,"parent_record_offset":75492,"ancestor_record_offset":75320,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":562,"record_offset":78692,"parent_record_offset":78404,"ancestor_record_offset":75320,"inlinee":5310,"source_line":3273,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::ComponentDescriptor::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/component.rs
//     evidence={"module":562,"record_offset":78464,"parent_record_offset":78404,"ancestor_record_offset":75320,"inlinee":5647,"source_line":1101,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::ComponentIds::next_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/component.rs
//     evidence={"module":562,"record_offset":78372,"parent_record_offset":75564,"ancestor_record_offset":75320,"inlinee":5644,"source_line":1334,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::Components::register_component_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/component.rs
//     evidence={"module":562,"record_offset":78492,"parent_record_offset":78404,"ancestor_record_offset":75320,"inlinee":5648,"source_line":1972,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::ComponentsRegistrator::register_component_checked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/component.rs
//     evidence={"module":562,"record_offset":75564,"parent_record_offset":75492,"ancestor_record_offset":75320,"inlinee":5612,"source_line":1721,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::ComponentsRegistrator::register_component_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/component.rs
//     evidence={"module":562,"record_offset":78404,"parent_record_offset":75564,"ancestor_record_offset":75320,"inlinee":5645,"source_line":1755,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::impl$17::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/component.rs
//     evidence={"module":562,"record_offset":78628,"parent_record_offset":78404,"ancestor_record_offset":75320,"inlinee":13108,"source_line":1614,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::impl$90::default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/component.rs
//     evidence={"module":562,"record_offset":78664,"parent_record_offset":78404,"ancestor_record_offset":75320,"inlinee":5662,"source_line":2812,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] bevy_platform::collections::hash_map::HashMap::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_platform","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_platform-0.16.0/src/collections/hash_map.rs
//     evidence={"module":562,"record_offset":75632,"parent_record_offset":75564,"ancestor_record_offset":75320,"inlinee":5614,"source_line":933,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] bevy_platform::collections::hash_map::HashMap::remove
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_platform","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_platform-0.16.0/src/collections/hash_map.rs
//     evidence={"module":562,"record_offset":76672,"parent_record_offset":75564,"ancestor_record_offset":75320,"inlinee":5624,"source_line":1141,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$7::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\any.rs
//     evidence={"module":562,"record_offset":76412,"parent_record_offset":76388,"ancestor_record_offset":75320,"inlinee":5068,"source_line":719,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$25::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":562,"record_offset":76460,"parent_record_offset":76436,"ancestor_record_offset":75320,"inlinee":5070,"source_line":1740,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_cmpeq_epi8
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":562,"record_offset":77932,"parent_record_offset":77904,"ancestor_record_offset":75320,"inlinee":5636,"source_line":842,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":562,"record_offset":75860,"parent_record_offset":75836,"ancestor_record_offset":75320,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":562,"record_offset":75944,"parent_record_offset":75920,"ancestor_record_offset":75320,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":562,"record_offset":75884,"parent_record_offset":75860,"ancestor_record_offset":75320,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":562,"record_offset":79288,"parent_record_offset":79256,"ancestor_record_offset":75320,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":562,"record_offset":77792,"parent_record_offset":77744,"ancestor_record_offset":75320,"inlinee":5005,"source_line":2021,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$7::leading_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":562,"record_offset":78168,"parent_record_offset":78140,"ancestor_record_offset":75320,"inlinee":5639,"source_line":135,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$7::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":562,"record_offset":78196,"parent_record_offset":78140,"ancestor_record_offset":75320,"inlinee":5640,"source_line":161,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":562,"record_offset":76096,"parent_record_offset":76068,"ancestor_record_offset":75320,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":562,"record_offset":76068,"parent_record_offset":76040,"ancestor_record_offset":75320,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::offset_from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":562,"record_offset":77704,"parent_record_offset":77680,"ancestor_record_offset":75320,"inlinee":5634,"source_line":682,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":562,"record_offset":78596,"parent_record_offset":78492,"ancestor_record_offset":75320,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":562,"record_offset":78292,"parent_record_offset":78268,"ancestor_record_offset":75320,"inlinee":4834,"source_line":1354,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":562,"record_offset":76328,"parent_record_offset":76304,"ancestor_record_offset":75320,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":562,"record_offset":78316,"parent_record_offset":78292,"ancestor_record_offset":75320,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":562,"record_offset":79792,"parent_record_offset":79632,"ancestor_record_offset":75320,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":562,"record_offset":78540,"parent_record_offset":78492,"ancestor_record_offset":75320,"inlinee":5649,"source_line":619,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":562,"record_offset":78564,"parent_record_offset":78540,"ancestor_record_offset":75320,"inlinee":5650,"source_line":232,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":562,"record_offset":76604,"parent_record_offset":76580,"ancestor_record_offset":75320,"inlinee":5623,"source_line":623,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":562,"record_offset":76628,"parent_record_offset":76604,"ancestor_record_offset":75320,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] core::tuple::impl$10::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\tuple.rs
//     evidence={"module":562,"record_offset":76436,"parent_record_offset":76412,"ancestor_record_offset":75320,"inlinee":4662,"source_line":30,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] equivalent::impl$0::equivalent
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"equivalent","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/equivalent-1.0.1/src/lib.rs
//     evidence={"module":562,"record_offset":76388,"parent_record_offset":76364,"ancestor_record_offset":75320,"inlinee":4660,"source_line":88,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::leading_zeros
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/bitmask.rs
//     evidence={"module":562,"record_offset":78140,"parent_record_offset":77744,"ancestor_record_offset":75320,"inlinee":5637,"source_line":86,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":562,"record_offset":76004,"parent_record_offset":75976,"ancestor_record_offset":75320,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":562,"record_offset":76040,"parent_record_offset":76004,"ancestor_record_offset":75320,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":562,"record_offset":75976,"parent_record_offset":75756,"ancestor_record_offset":75320,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":562,"record_offset":75836,"parent_record_offset":75756,"ancestor_record_offset":75320,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":562,"record_offset":76140,"parent_record_offset":75756,"ancestor_record_offset":75320,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":562,"record_offset":75920,"parent_record_offset":75756,"ancestor_record_offset":75320,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/map.rs
//     evidence={"module":562,"record_offset":75656,"parent_record_offset":75632,"ancestor_record_offset":75320,"inlinee":4719,"source_line":1304,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":562,"record_offset":75680,"parent_record_offset":75656,"ancestor_record_offset":75320,"inlinee":4475,"source_line":1347,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::remove
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/map.rs
//     evidence={"module":562,"record_offset":76704,"parent_record_offset":76672,"ancestor_record_offset":75320,"inlinee":5625,"source_line":1954,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::remove_entry
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":562,"record_offset":76736,"parent_record_offset":76704,"ancestor_record_offset":75320,"inlinee":5626,"source_line":1993,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":562,"record_offset":76364,"parent_record_offset":76256,"ancestor_record_offset":75320,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":562,"record_offset":76304,"parent_record_offset":76280,"ancestor_record_offset":75320,"inlinee":5616,"source_line":261,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::read
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":562,"record_offset":78268,"parent_record_offset":77576,"ancestor_record_offset":75320,"inlinee":5642,"source_line":488,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::to_base_index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":562,"record_offset":77656,"parent_record_offset":77632,"ancestor_record_offset":75320,"inlinee":5631,"source_line":334,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":562,"record_offset":76224,"parent_record_offset":75756,"ancestor_record_offset":75320,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":562,"record_offset":76280,"parent_record_offset":76256,"ancestor_record_offset":75320,"inlinee":5615,"source_line":734,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket_index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":562,"record_offset":77632,"parent_record_offset":77604,"ancestor_record_offset":75320,"inlinee":5630,"source_line":705,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::erase_no_drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":562,"record_offset":77604,"parent_record_offset":77576,"ancestor_record_offset":75320,"inlinee":5629,"source_line":766,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":562,"record_offset":75732,"parent_record_offset":75708,"ancestor_record_offset":75320,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":562,"record_offset":75708,"parent_record_offset":75680,"ancestor_record_offset":75320,"inlinee":4477,"source_line":1179,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::remove
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":562,"record_offset":77576,"parent_record_offset":76760,"ancestor_record_offset":75320,"inlinee":5628,"source_line":785,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::remove_entry
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":562,"record_offset":76760,"parent_record_offset":76736,"ancestor_record_offset":75320,"inlinee":5627,"source_line":797,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::erase
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":562,"record_offset":77744,"parent_record_offset":77604,"ancestor_record_offset":75320,"inlinee":5635,"source_line":3062,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":562,"record_offset":75756,"parent_record_offset":75732,"ancestor_record_offset":75320,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":562,"record_offset":75808,"parent_record_offset":75756,"ancestor_record_offset":75320,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::set_ctrl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":562,"record_offset":78228,"parent_record_offset":77744,"ancestor_record_offset":75320,"inlinee":5641,"source_line":2419,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$17::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":562,"record_offset":78956,"parent_record_offset":78932,"ancestor_record_offset":75320,"inlinee":4995,"source_line":3323,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":562,"record_offset":76256,"parent_record_offset":75756,"ancestor_record_offset":75320,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::offset_from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":562,"record_offset":77680,"parent_record_offset":77656,"ancestor_record_offset":75320,"inlinee":5633,"source_line":18,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::borrow
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":562,"record_offset":76556,"parent_record_offset":76532,"ancestor_record_offset":75320,"inlinee":5620,"source_line":116,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":562,"record_offset":76580,"parent_record_offset":76556,"ancestor_record_offset":75320,"inlinee":5621,"source_line":144,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::rwlock::RwLock::get_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\rwlock.rs
//     evidence={"module":562,"record_offset":76532,"parent_record_offset":75564,"ancestor_record_offset":75320,"inlinee":5618,"source_line":631,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::components::transient_mesh::TransientMesh::new_with_registry
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=5
// 源码行: 32, 35
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6954
// 首条位置: {"module":486,"inlinee":26964}
// 编译实例 RVAs: 0xaa8200
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":486,"record_offset":48212,"parent_record_offset":48188,"ancestor_record_offset":48188,"inlinee":6804,"source_line":2163,"archive":"_pdb/B/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":486,"record_offset":48240,"parent_record_offset":48212,"ancestor_record_offset":48188,"inlinee":6812,"source_line":2956,"archive":"_pdb/B/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":486,"record_offset":48264,"parent_record_offset":48240,"ancestor_record_offset":48188,"inlinee":4128,"source_line":3760,"archive":"_pdb/B/modules/00486.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::components::transient_mesh::impl$2::drop
// 记录形式: compiled_procedure; instances=1
// 源码行: 41
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":485,"record_offset":1134068,"rva":9019280,"va":5377728400}
// 编译实例 RVAs: 0x899f90
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140043000
//     owner=unresolved_namespace; count=1
//     target_aliases=30 candidates; complete list: transient_mesh.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8745215","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377728400,"raw_hex":"e8018f7aff","size":5,"soft_fail":false,"target_va":5368983552,"va":5377728762,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405d2870
//     owner=unresolved_namespace; count=1
//     target_aliases=102 candidates; complete list: transient_mesh.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2914492","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377728400,"raw_hex":"e84487d3ff","size":5,"soft_fail":false,"target_va":5374814320,"va":5377728807,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-3372952","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377728400,"raw_hex":"e86888ccff","size":5,"soft_fail":false,"target_va":5374355568,"va":5377728515,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33814196","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377728400,"raw_hex":"e8b4f60302","size":5,"soft_fail":false,"target_va":5411542880,"va":5377728679,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::panicking::panic_count::is_zero_slow_path
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t33810002","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377728400,"raw_hex":"e852e60302","size":5,"soft_fail":false,"target_va":5411538608,"va":5377728601,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::mutex::futex::Mutex::lock_contended
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33810178","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377728400,"raw_hex":"e802e70302","size":5,"soft_fail":false,"target_va":5411538768,"va":5377728585,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] std::sys::sync::mutex::futex::Mutex::wake
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tjmp\t33810259","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5377728400,"raw_hex":"e953e70302","size":5,"soft_fail":false,"target_va":5411538960,"va":5377728696,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":485,"record_offset":1134224,"parent_record_offset":1134068,"ancestor_record_offset":1134068,"inlinee":5345,"source_line":2205,"archive":"_pdb/B/modules/00485.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":485,"record_offset":1134752,"parent_record_offset":1134068,"ancestor_record_offset":1134068,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00485.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":485,"record_offset":1134932,"parent_record_offset":1134068,"ancestor_record_offset":1134068,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00485.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":485,"record_offset":1134708,"parent_record_offset":1134068,"ancestor_record_offset":1134068,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00485.jsonl.gz"}
//   [pdb_inline_direct] std::sync::poison::mutex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\mutex.rs
//     evidence={"module":485,"record_offset":1134252,"parent_record_offset":1134068,"ancestor_record_offset":1134068,"inlinee":6007,"source_line":435,"archive":"_pdb/B/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":485,"record_offset":1134812,"parent_record_offset":1134788,"ancestor_record_offset":1134068,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":485,"record_offset":1134860,"parent_record_offset":1134836,"ancestor_record_offset":1134068,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":485,"record_offset":1134836,"parent_record_offset":1134812,"ancestor_record_offset":1134068,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":485,"record_offset":1134788,"parent_record_offset":1134752,"ancestor_record_offset":1134068,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":485,"record_offset":1134900,"parent_record_offset":1134752,"ancestor_record_offset":1134068,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":485,"record_offset":1134628,"parent_record_offset":1134600,"ancestor_record_offset":1134068,"inlinee":5623,"source_line":623,"archive":"_pdb/B/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":485,"record_offset":1135184,"parent_record_offset":1134996,"ancestor_record_offset":1134068,"inlinee":5912,"source_line":651,"archive":"_pdb/B/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":485,"record_offset":1134320,"parent_record_offset":1134288,"ancestor_record_offset":1134068,"inlinee":4945,"source_line":2871,"archive":"_pdb/B/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":485,"record_offset":1135276,"parent_record_offset":1135244,"ancestor_record_offset":1134068,"inlinee":5910,"source_line":2753,"archive":"_pdb/B/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":485,"record_offset":1134532,"parent_record_offset":1134500,"ancestor_record_offset":1134068,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":485,"record_offset":1134344,"parent_record_offset":1134320,"ancestor_record_offset":1134068,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":485,"record_offset":1134556,"parent_record_offset":1134532,"ancestor_record_offset":1134068,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":485,"record_offset":1135208,"parent_record_offset":1135184,"ancestor_record_offset":1134068,"inlinee":5913,"source_line":3712,"archive":"_pdb/B/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":485,"record_offset":1135300,"parent_record_offset":1135276,"ancestor_record_offset":1134068,"inlinee":5911,"source_line":3743,"archive":"_pdb/B/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\panicking.rs
//     evidence={"module":485,"record_offset":1134500,"parent_record_offset":1134472,"ancestor_record_offset":1134068,"inlinee":5907,"source_line":469,"archive":"_pdb/B/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\panicking.rs
//     evidence={"module":485,"record_offset":1134472,"parent_record_offset":1134444,"ancestor_record_offset":1134068,"inlinee":5905,"source_line":621,"archive":"_pdb/B/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::done
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":485,"record_offset":1134996,"parent_record_offset":1134960,"ancestor_record_offset":1134068,"inlinee":5901,"source_line":132,"archive":"_pdb/B/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":485,"record_offset":1134600,"parent_record_offset":1134408,"ancestor_record_offset":1134068,"inlinee":5621,"source_line":144,"archive":"_pdb/B/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::guard
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":485,"record_offset":1134408,"parent_record_offset":1134380,"ancestor_record_offset":1134068,"inlinee":6011,"source_line":122,"archive":"_pdb/B/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::MutexGuard::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\mutex.rs
//     evidence={"module":485,"record_offset":1134380,"parent_record_offset":1134252,"ancestor_record_offset":1134068,"inlinee":6010,"source_line":647,"archive":"_pdb/B/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::impl$14::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\mutex.rs
//     evidence={"module":485,"record_offset":1134960,"parent_record_offset":1134932,"ancestor_record_offset":1134068,"inlinee":5900,"source_line":671,"archive":"_pdb/B/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":485,"record_offset":1134288,"parent_record_offset":1134252,"ancestor_record_offset":1134068,"inlinee":6008,"source_line":27,"archive":"_pdb/B/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::unlock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":485,"record_offset":1135244,"parent_record_offset":1134960,"ancestor_record_offset":1134068,"inlinee":5909,"source_line":83,"archive":"_pdb/B/modules/00485.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\mod.rs
//     evidence={"module":485,"record_offset":1134444,"parent_record_offset":1134408,"ancestor_record_offset":1134068,"inlinee":5903,"source_line":814,"archive":"_pdb/B/modules/00485.jsonl.gz"}
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
// 完整结构及原始类型记录: transient_mesh.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::components::transient_mesh::TransientMesh
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 24
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<country_core::components::transient_mesh::TransientMesh>", "constructor": "core::ptr::drop_in_place", "arguments": ["country_core::components::transient_mesh::TransientMesh"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::components::transient_mesh::TransientMesh,country_core::components::WoodenGate>", "constructor": "tuple$", "arguments": ["country_core::components::drawable::DrawableMeshBundle", "country_core::components::transient_mesh::TransientMesh", "country_core::components::WoodenGate"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::commands::entity_command::insert::closure_env$0<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::components::transient_mesh::TransientMesh,country_core::components::WoodenGate> >", "constructor": "bevy_ecs::system::commands::entity_command::insert::closure_env$0", "arguments": ["tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::components::transient_mesh::TransientMesh,country_core::components::WoodenGate>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::entity_command::insert::closure_env$0<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::components::transient_mesh::TransientMesh,country_core::components::WoodenGate> > >", "constructor": "bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0", "arguments": ["bevy_ecs::system::commands::entity_command::insert::closure_env$0<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::components::transient_mesh::TransientMesh,country_core::components::WoodenGate> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::error::command_handling::impl$0::handle_error_with::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::entity_command::insert::closure_env$0<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::components::transient_mesh::TransientMesh,country_core::components::WoodenGate> > >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> >", "constructor": "bevy_ecs::error::command_handling::impl$0::handle_error_with::closure_env$0", "arguments": ["bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::entity_command::insert::closure_env$0<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::components::transient_mesh::TransientMesh,country_core::components::WoodenGate> > >", "tuple$<>", "enum2$<bevy_ecs::world::error::EntityMutableFetchError>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::world::command_queue::impl$4::push::closure_env$0<bevy_ecs::error::command_handling::impl$0::handle_error_with::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::entity_command::insert::closure_env$0<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::components::transient_mesh::TransientMesh,country_core::components::WoodenGate> > >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> > >", "constructor": "bevy_ecs::world::command_queue::impl$4::push::closure_env$0", "arguments": ["bevy_ecs::error::command_handling::impl$0::handle_error_with::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::entity_command::insert::closure_env$0<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::components::transient_mesh::TransientMesh,country_core::components::WoodenGate> > >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<bevy_ecs::world::command_queue::impl$4::push::closure_env$0<bevy_ecs::error::command_handling::impl$0::handle_error_with::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::entity_command::insert::closure_env$0<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::components::transient_mesh::TransientMesh,country_core::components::WoodenGate> > >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> > >,tuple$<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2$<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World> > >,ref_mut$<usize> > >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["bevy_ecs::world::command_queue::impl$4::push::closure_env$0<bevy_ecs::error::command_handling::impl$0::handle_error_with::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::entity_command::insert::closure_env$0<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::components::transient_mesh::TransientMesh,country_core::components::WoodenGate> > >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> > >", "tuple$<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2$<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World> > >,ref_mut$<usize> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::components::transient_mesh::TransientMesh,country_core::components::WoodenGate> >", "constructor": "core::ptr::drop_in_place", "arguments": ["tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::components::transient_mesh::TransientMesh,country_core::components::WoodenGate>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<bevy_ecs::error::command_handling::impl$0::handle_error_with::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::entity_command::insert::closure_env$0<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::components::transient_mesh::TransientMesh,country_core::components::WoodenGate> > >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["bevy_ecs::error::command_handling::impl$0::handle_error_with::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::entity_command::insert::closure_env$0<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::components::transient_mesh::TransientMesh,country_core::components::WoodenGate> > >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::commands::Commands::spawn<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::components::transient_mesh::TransientMesh,country_core::components::WoodenGate> >", "constructor": "bevy_ecs::system::commands::Commands::spawn", "arguments": ["tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::components::transient_mesh::TransientMesh,country_core::components::WoodenGate>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<bevy_ecs::system::commands::entity_command::insert::closure_env$0<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::components::transient_mesh::TransientMesh,country_core::components::WoodenGate> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["bevy_ecs::system::commands::entity_command::insert::closure_env$0<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::components::transient_mesh::TransientMesh,country_core::components::WoodenGate> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::commands::command::impl$0::apply<bevy_ecs::error::command_handling::impl$0::handle_error_with::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::entity_command::insert::closure_env$0<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::components::transient_mesh::TransientMesh,country_core::components::WoodenGate> > >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> >,tuple$<> >", "constructor": "bevy_ecs::system::commands::command::impl$0::apply", "arguments": ["bevy_ecs::error::command_handling::impl$0::handle_error_with::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::entity_command::insert::closure_env$0<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::components::transient_mesh::TransientMesh,country_core::components::WoodenGate> > >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> >", "tuple$<>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::component::ComponentDescriptor::drop_ptr<country_core::components::transient_mesh::TransientMesh>", "constructor": "bevy_ecs::component::ComponentDescriptor::drop_ptr", "arguments": ["country_core::components::transient_mesh::TransientMesh"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::commands::entity_command::insert::closure_env$0<country_core::components::transient_mesh::TransientMesh>", "constructor": "bevy_ecs::system::commands::entity_command::insert::closure_env$0", "arguments": ["country_core::components::transient_mesh::TransientMesh"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::entity_command::insert::closure_env$0<country_core::components::transient_mesh::TransientMesh> >", "constructor": "bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0", "arguments": ["bevy_ecs::system::commands::entity_command::insert::closure_env$0<country_core::components::transient_mesh::TransientMesh>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::error::command_handling::impl$0::handle_error_with::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::entity_command::insert::closure_env$0<country_core::components::transient_mesh::TransientMesh> >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> >", "constructor": "bevy_ecs::error::command_handling::impl$0::handle_error_with::closure_env$0", "arguments": ["bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::entity_command::insert::closure_env$0<country_core::components::transient_mesh::TransientMesh> >", "tuple$<>", "enum2$<bevy_ecs::world::error::EntityMutableFetchError>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::world::command_queue::impl$4::push::closure_env$0<bevy_ecs::error::command_handling::impl$0::handle_error_with::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::entity_command::insert::closure_env$0<country_core::components::transient_mesh::TransientMesh> >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> > >", "constructor": "bevy_ecs::world::command_queue::impl$4::push::closure_env$0", "arguments": ["bevy_ecs::error::command_handling::impl$0::handle_error_with::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::entity_command::insert::closure_env$0<country_core::components::transient_mesh::TransientMesh> >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<bevy_ecs::world::command_queue::impl$4::push::closure_env$0<bevy_ecs::error::command_handling::impl$0::handle_error_with::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::entity_command::insert::closure_env$0<country_core::components::transient_mesh::TransientMesh> >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> > >,tuple$<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2$<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World> > >,ref_mut$<usize> > >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["bevy_ecs::world::command_queue::impl$4::push::closure_env$0<bevy_ecs::error::command_handling::impl$0::handle_error_with::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::entity_command::insert::closure_env$0<country_core::components::transient_mesh::TransientMesh> >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> > >", "tuple$<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2$<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World> > >,ref_mut$<usize> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::commands::EntityCommands::insert<country_core::components::transient_mesh::TransientMesh>", "constructor": "bevy_ecs::system::commands::EntityCommands::insert", "arguments": ["country_core::components::transient_mesh::TransientMesh"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::bundle::impl$8::register_info::closure$0::closure_env$0<country_core::components::transient_mesh::TransientMesh>", "constructor": "bevy_ecs::bundle::impl$8::register_info::closure$0::closure_env$0", "arguments": ["country_core::components::transient_mesh::TransientMesh"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::bundle::impl$0::component_ids<country_core::components::transient_mesh::TransientMesh,bevy_ecs::bundle::impl$8::register_info::closure$0::closure_env$0<country_core::components::transient_mesh::TransientMesh> >", "constructor": "bevy_ecs::bundle::impl$0::component_ids", "arguments": ["country_core::components::transient_mesh::TransientMesh", "bevy_ecs::bundle::impl$8::register_info::closure$0::closure_env$0<country_core::components::transient_mesh::TransientMesh>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::commands::command::impl$0::apply<bevy_ecs::error::command_handling::impl$0::handle_error_with::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::entity_command::insert::closure_env$0<country_core::components::transient_mesh::TransientMesh> >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> >,tuple$<> >", "constructor": "bevy_ecs::system::commands::command::impl$0::apply", "arguments": ["bevy_ecs::error::command_handling::impl$0::handle_error_with::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::entity_command::insert::closure_env$0<country_core::components::transient_mesh::TransientMesh> >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> >", "tuple$<>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::bundle::BundleInfo::write_components<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::components::transient_mesh::TransientMesh,country_core::components::WoodenGate>,bevy_ecs::archetype::ArchetypeAfterBundleInsert,core::slice::iter::Iter<bevy_ecs::component::RequiredComponentConstructor> >", "constructor": "bevy_ecs::bundle::BundleInfo::write_components", "arguments": ["tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::components::transient_mesh::TransientMesh,country_core::components::WoodenGate>", "bevy_ecs::archetype::ArchetypeAfterBundleInsert", "core::slice::iter::Iter<bevy_ecs::component::RequiredComponentConstructor>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::commands::entity_command::impl$0::apply<tuple$<>,bevy_ecs::system::commands::entity_command::insert::closure_env$0<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::components::transient_mesh::TransientMesh,country_core::components::WoodenGate> > >", "constructor": "bevy_ecs::system::commands::entity_command::impl$0::apply", "arguments": ["tuple$<>", "bevy_ecs::system::commands::entity_command::insert::closure_env$0<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::components::transient_mesh::TransientMesh,country_core::components::WoodenGate> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::commands::entity_command::impl$0::apply<tuple$<>,bevy_ecs::system::commands::entity_command::insert::closure_env$0<country_core::components::transient_mesh::TransientMesh> >", "constructor": "bevy_ecs::system::commands::entity_command::impl$0::apply", "arguments": ["tuple$<>", "bevy_ecs::system::commands::entity_command::insert::closure_env$0<country_core::components::transient_mesh::TransientMesh>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::bundle::Bundles::register_info<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::components::transient_mesh::TransientMesh,country_core::components::WoodenGate> >", "constructor": "bevy_ecs::bundle::Bundles::register_info", "arguments": ["tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::components::transient_mesh::TransientMesh,country_core::components::WoodenGate>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::bundle::Bundles::register_info<country_core::components::transient_mesh::TransientMesh>", "constructor": "bevy_ecs::bundle::Bundles::register_info", "arguments": ["country_core::components::transient_mesh::TransientMesh"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::bundle::impl$8::register_info::closure$0::closure_env$0<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::components::transient_mesh::TransientMesh,country_core::components::WoodenGate> >", "constructor": "bevy_ecs::bundle::impl$8::register_info::closure$0::closure_env$0", "arguments": ["tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::components::transient_mesh::TransientMesh,country_core::components::WoodenGate>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::bundle::impl$18::component_ids<country_core::components::drawable::DrawableMeshBundle,country_core::components::transient_mesh::TransientMesh,country_core::components::WoodenGate,bevy_ecs::bundle::impl$8::register_info::closure$0::closure_env$0<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::components::transient_mesh::TransientMesh,country_core::components::WoodenGate> > >", "constructor": "bevy_ecs::bundle::impl$18::component_ids", "arguments": ["country_core::components::drawable::DrawableMeshBundle", "country_core::components::transient_mesh::TransientMesh", "country_core::components::WoodenGate", "bevy_ecs::bundle::impl$8::register_info::closure$0::closure_env$0<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::components::transient_mesh::TransientMesh,country_core::components::WoodenGate> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::bundle::impl$0::component_ids<country_core::components::transient_mesh::TransientMesh,bevy_ecs::bundle::impl$8::register_info::closure$0::closure_env$0<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::components::transient_mesh::TransientMesh,country_core::components::WoodenGate> > >", "constructor": "bevy_ecs::bundle::impl$0::component_ids", "arguments": ["country_core::components::transient_mesh::TransientMesh", "bevy_ecs::bundle::impl$8::register_info::closure$0::closure_env$0<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::components::transient_mesh::TransientMesh,country_core::components::WoodenGate> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::bundle::impl$0::component_ids<country_core::components::WoodenGate,bevy_ecs::bundle::impl$8::register_info::closure$0::closure_env$0<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::components::transient_mesh::TransientMesh,country_core::components::WoodenGate> > >", "constructor": "bevy_ecs::bundle::impl$0::component_ids", "arguments": ["country_core::components::WoodenGate", "bevy_ecs::bundle::impl$8::register_info::closure$0::closure_env$0<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::components::transient_mesh::TransientMesh,country_core::components::WoodenGate> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::components::transient_mesh::TransientMeshRegistry
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 3; 过程引用数: 3
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::Arc::drop_slow<country_core::components::transient_mesh::TransientMeshRegistry,alloc::alloc::Global>", "constructor": "alloc::sync::Arc::drop_slow", "arguments": ["country_core::components::transient_mesh::TransientMeshRegistry", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::components::transient_mesh::TransientMeshRegistry::drain_meshes_to_delete<country_core::systems::backend::transient_mesh::delete_dropped_transient_meshes::closure_env$0>", "constructor": "country_core::components::transient_mesh::TransientMeshRegistry::drain_meshes_to_delete", "arguments": ["country_core::systems::backend::transient_mesh::delete_dropped_transient_meshes::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: transient_mesh.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// [已有模块证据关联；不自动成为字段事实] evidence/building-gates/B-mesh-helpers/report.json; explicit B; [{"kind": "exact_PDB_named_entity_mention", "name": "country_core::components::transient_mesh::TransientMesh", "status": "evidence_association; original field ownership unknown"}, {"kind": "exact_PDB_named_entity_mention", "name": "country_core::components::transient_mesh::TransientMeshRegistry", "status": "evidence_association; original field ownership unknown"}]
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: transient_mesh.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 94; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::event::reader::EventReader<country_core::resources::window::WindowFocusEvent>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>,bevy_ecs::event::reader::EventReader<country_core::resources::events::ShutdownGameCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::brush_preview::OverwriteBrushPreview>,bevy_ecs::event::writer::EventWriter<enum2$<country_core::resources::mouse_capture::OsCursorVisibilityCmd> >,bevy_ecs::event::writer::EventWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>", "bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>", "bevy_ecs::event::reader::EventReader<country_core::resources::window::WindowFocusEvent>", "bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>", "bevy_ecs::event::reader::EventReader<country_core::resources::events::ShutdownGameCmd>", "bevy_ecs::event::writer::EventWriter<country_core::systems::brush_preview::OverwriteBrushPreview>", "bevy_ecs::event::writer::EventWriter<enum2$<country_core::resources::mouse_capture::OsCursorVisibilityCmd> >", "bevy_ecs::event::writer::EventWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::window::WindowFocusEvent>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustOpened>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::events::ShutdownGameCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::brush_preview::OverwriteBrushPreview>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<enum2$<country_core::resources::mouse_capture::OsCursorVisibilityCmd> >
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>", "bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::system::commands::Commands", "bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>", "bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::ResMut<country_core::systems::audio::terrain_audio::TerrainAudioState>,bevy_ecs::event::reader::EventReader<enum2$<country_core::systems::audio::terrain_audio::PlayTerrainSound> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::change_detection::ResMut<country_core::systems::audio::terrain_audio::TerrainAudioState>", "bevy_ecs::event::reader::EventReader<enum2$<country_core::systems::audio::terrain_audio::PlayTerrainSound> >", "bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>", "bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>", "bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::terrain_audio::TerrainAudioState>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<enum2$<country_core::systems::audio::terrain_audio::PlayTerrainSound> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::resources::critters::BirdSpotManager>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>", "bevy_ecs::change_detection::ResMut<country_core::resources::critters::BirdSpotManager>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::critters::BirdSpotManager>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::ResMut<country_core::systems::history::signifier_replay_active::ReplayModeElapsedTime>,bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::history::signifier_replay_active::SignifierReplayIndex> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>", "bevy_ecs::change_detection::ResMut<country_core::systems::history::signifier_replay_active::ReplayModeElapsedTime>", "bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::history::signifier_replay_active::SignifierReplayIndex> >,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>", "bevy_ecs::change_detection::Res<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::history::signifier_replay_active::ReplayModeElapsedTime>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::history::signifier_replay_active::SignifierReplayIndex> >,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::drawable::NaniActiveInstance>,ref_mut$<country_core::render::nani::instance_data::CurvedHalfTimberInstanceData>,ref$<country_core::resources::walls::curved_halftimber::CurvedHalfTimber> >,bevy_ecs::query::filter::With<country_core::render::nani::gpu_world::InstanceDataMarker<country_core::render::nani::instance_data::CurvedHalfTimberInstanceData> > >,bevy_ecs::change_detection::Res<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::drawable::NaniActiveInstance>,ref_mut$<country_core::render::nani::instance_data::CurvedHalfTimberInstanceData>,ref$<country_core::resources::walls::curved_halftimber::CurvedHalfTimber> >,bevy_ecs::query::filter::With<country_core::render::nani::gpu_world::InstanceDataMarker<country_core::render::nani::instance_data::CurvedHalfTimberInstanceData> > >", "bevy_ecs::change_detection::Res<country_core::render::nani::gpu_world::GpuWorld>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>", "bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >"]
// [候选系统接口] query_with_filter: bevy_ecs::query::filter::With<country_core::render::nani::gpu_world::InstanceDataMarker<country_core::render::nani::instance_data::CurvedHalfTimberInstanceData> >
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::drawable::NaniActiveInstance>,ref_mut$<country_core::render::nani::instance_data::CurvedHalfTimberInstanceData>,ref$<country_core::resources::walls::curved_halftimber::CurvedHalfTimber> >,bevy_ecs::query::filter::With<country_core::render::nani::gpu_world::InstanceDataMarker<country_core::render::nani::instance_data::CurvedHalfTimberInstanceData> > >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::nani::gpu_world::GpuWorld>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::rectangle::ui_create_rectangle::PreviewRectangle>,bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::circle::ui_create_circle::PreviewCircle>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::systems::wall::shapes::shape_placement::ShapePlacement>,bevy_ecs::change_detection::ResMut<country_core::systems::wall::shapes::calc_shape_height::TimberframeConstructionParamsCache>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::rectangle::ui_create_rectangle::PreviewRectangle>", "bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::circle::ui_create_circle::PreviewCircle>", "bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >", "bevy_ecs::change_detection::ResMut<country_core::systems::wall::shapes::shape_placement::ShapePlacement>", "bevy_ecs::change_detection::ResMut<country_core::systems::wall::shapes::calc_shape_height::TimberframeConstructionParamsCache>", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::rectangle::ui_create_rectangle::PreviewRectangle>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::circle::ui_create_circle::PreviewCircle>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::wall::shapes::shape_placement::ShapePlacement>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::wall::shapes::calc_shape_height::TimberframeConstructionParamsCache>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::drawable::NaniActiveInstance>,ref$<country_core::resources::walls::plaster::Plaster> >,tuple$<bevy_ecs::query::filter::With<country_core::render::nani::gpu_world::InstanceDataMarker<country_core::render::nani::instance_data::PlasterInstanceData> >,bevy_ecs::query::filter::Or<tuple$<bevy_ecs::query::filter::Changed<country_core::resources::walls::plaster::Plaster>,bevy_ecs::query::filter::Changed<country_core::components::drawable::NaniActiveInstance> > > > >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::change_detection::Res<country_core::resources::walls::plaster::AnimatedPlasterPeel>)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::drawable::NaniActiveInstance>,ref$<country_core::resources::walls::plaster::Plaster> >,tuple$<bevy_ecs::query::filter::With<country_core::render::nani::gpu_world::InstanceDataMarker<country_core::render::nani::instance_data::PlasterInstanceData> >,bevy_ecs::query::filter::Or<tuple$<bevy_ecs::query::filter::Changed<country_core::resources::walls::plaster::Plaster>,bevy_ecs::query::filter::Changed<country_core::components::drawable::NaniActiveInstance> > > > >", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::Res<country_core::render::nani::gpu_world::GpuWorld>", "bevy_ecs::change_detection::Res<country_core::resources::walls::plaster::AnimatedPlasterPeel>"]
// [候选系统接口] query_with_filter: bevy_ecs::query::filter::With<country_core::render::nani::gpu_world::InstanceDataMarker<country_core::render::nani::instance_data::PlasterInstanceData> >
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::drawable::NaniActiveInstance>,ref$<country_core::resources::walls::plaster::Plaster> >,tuple$<bevy_ecs::query::filter::With<country_core::render::nani::gpu_world::InstanceDataMarker<country_core::render::nani::instance_data::PlasterInstanceData> >,bevy_ecs::query::filter::Or<tuple$<bevy_ecs::query::filter::Changed<country_core::resources::walls::plaster::Plaster>,bevy_ecs::query::filter::Changed<country_core::components::drawable::NaniActiveInstance> > > > >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::nani::gpu_world::GpuWorld>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::plaster::AnimatedPlasterPeel>
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::components::transient_mesh::TransientMesh::new_with_registry
// 信息 ID: 44292ea89e67f55063cdc7aa
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_gates::construct_wooden_gate_doors @ 0x1212993 -> 0xaa8200 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::systems::ui::signifiers::SignifierEntityBuilder::build; module=486, inline_record=48188; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::ui::debug_visualize_volumes::debug_visualize_roof_uvs; module=1605, inline_record=25744; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::components::transient_mesh::TransientMeshRegistry::deferred_delete_mesh
// 信息 ID: c46ca107da70c1f8e94b9446
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_roof::visual::switch_roof_type::switch_roof_type @ 0x21ac8c7 -> 0xaa8070 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::components::transient_mesh::TransientMeshRegistry::drain_meshes_to_delete<country_core::systems::backend::transient_mesh::delete_dropped_transient_meshes::closure_env$0>
// 信息 ID: 2fe79875ca5e9b70a51ec473
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 156db0489f893be77b36dd47: void (*)(bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::vao_library::AssetVAOLibrary>)
// [类型关系] country_core::components::transient_mesh::TransientMeshRegistry::drain_meshes_to_delete -> ["country_core::systems::backend::transient_mesh::delete_dropped_transient_meshes::closure_env$0"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::vao_library::AssetVAOLibrary>),void (*)(bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::vao_library::AssetVAOLibrary>)> > @ 0xabddd4 -> 0x12e9e70 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::vao_library::AssetVAOLibrary>),void (*)(bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::vao_library::AssetVAOLibrary>)> > > @ 0x14c8bb5 -> 0x12e9e70 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::components::transient_mesh::impl$2::drop
// 信息 ID: 59ca9cc00e33701a4a463109
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 6 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::ptr::drop_in_place<country_core::components::transient_mesh::TransientMesh> | core::ptr::drop_in_place<country_core::components::transient_mesh::TransientMesh> | bevy_ecs::component::ComponentDescriptor::drop_ptr<country_core::components::transient_mesh::TransientMesh> | bevy_ecs::component::ComponentDescriptor::drop_ptr<country_core::components::transient_mesh::TransientMesh> @ 0x88fe76 -> 0x899f90 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<bevy_ecs::world::command_queue::impl$4::push::closure_env$0<bevy_ecs::error::command_handling::impl$0::handle_error_with::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::entity_command::insert::closure_env$0<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::components::transient_mesh::TransientMesh,country_core::components::WoodenGate> > >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> > >,tuple$<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2$<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World> > >,ref_mut$<usize> > > @ 0x120ebb0 -> 0x899f90 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ptr::drop_in_place<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::components::transient_mesh::TransientMesh,country_core::components::WoodenGate> > | core::ptr::drop_in_place<bevy_ecs::error::command_handling::impl$0::handle_error_with::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::entity_command::insert::closure_env$0<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::components::transient_mesh::TransientMesh,country_core::components::WoodenGate> > >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> > > | core::ptr::drop_in_place<bevy_ecs::system::commands::entity_command::insert::closure_env$0<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::components::transient_mesh::TransientMesh,country_core::components::WoodenGate> > > | core::ptr::drop_in_place<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::components::transient_mesh::TransientMesh,country_core::components::WoodenGate> > @ 0x120f256 -> 0x899f90 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::commands::command::impl$0::apply<bevy_ecs::error::command_handling::impl$0::handle_error_with::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::entity_command::insert::closure_env$0<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::components::transient_mesh::TransientMesh,country_core::components::WoodenGate> > >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> >,tuple$<> > @ 0x120fe53 -> 0x899f90 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<bevy_ecs::world::command_queue::impl$4::push::closure_env$0<bevy_ecs::error::command_handling::impl$0::handle_error_with::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::entity_command::insert::closure_env$0<country_core::components::transient_mesh::TransientMesh> >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> > >,tuple$<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2$<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World> > >,ref_mut$<usize> > > @ 0x12da7d8 -> 0x899f90 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::commands::command::impl$0::apply<bevy_ecs::error::command_handling::impl$0::handle_error_with::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::entity_command::insert::closure_env$0<country_core::components::transient_mesh::TransientMesh> >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> >,tuple$<> > @ 0x1563b1e -> 0x899f90 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::components::transient_mesh::impl$4::clone_behavior
// 信息 ID: 4249d211848014e17fec5227
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 183 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::query::fetch::impl$26::init_state<country_core::systems::look_at_camera::LookAtCamera> @ 0x3bdf7 -> 0x2370 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::query::world_query::impl$2::init_state<ref$<country_core::resources::critters::sheep::UmbrellaHolder>,ref$<country_core::resources::critters::sheep::FloatieHolder> > @ 0x81034 -> 0x2370 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::query::world_query::impl$2::init_state<ref$<country_core::resources::critters::sheep::UmbrellaHolder>,ref$<country_core::resources::critters::sheep::FloatieHolder> > @ 0x814f2 -> 0x2370 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::query::fetch::impl$26::init_state<country_core::resources::critters::sheep::Sheep> @ 0x81f47 -> 0x2370 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::query::fetch::impl$26::init_state<country_core::resources::critters::sheep::SheepAudio> @ 0x825d7 -> 0x2370 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<tiny_glade::gui::gui_debug::do_gui_debug::closure$0::closure$0::closure_env$0,tuple$<ref_mut$<egui::ui::Ui> > > @ 0xba779 -> 0x2370 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::walls::wall_attachment::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > > @ 0xeed7f -> 0x2370 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::walls::wall_attachment::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > > @ 0xeee3a -> 0x2370 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<tiny_glade::gui::gui_debug::do_gui_debug::closure$0::closure$0::closure$13::closure$1::closure_env$1,tuple$<ref_mut$<egui::ui::Ui> > > @ 0xfc149 -> 0x2370 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<tiny_glade::gui::gui_debug::do_gui_debug::closure$0::closure$0::closure$13::closure$1::closure_env$0,tuple$<ref_mut$<egui::ui::Ui> > > @ 0xfe760 -> 0x2370 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::query::fetch::impl$26::init_state<country_core::systems::ui::signifiers::SignifierAnimation> @ 0x157a17 -> 0x2370 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::query::fetch::impl$26::init_state<country_core::systems::ui::signifiers::SignifierMarkedForDeletion> @ 0x1580a7 -> 0x2370 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::query::world_query::impl$3::init_state<ref$<country_core::systems::prefab::PrefabInstanceData>,ref$<country_core::resources::prefab::PrefabInstance>,ref$<country_core::components::TreeComponent> > @ 0x25bfe7 -> 0x2370 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::query::fetch::impl$26::init_state<country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume> @ 0x2b1887 -> 0x2370 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::error::Error::cause<enum2$<bevy_ecs::world::error::EntityMutableFetchError> > | core::error::Error::cause<enum2$<bevy_ecs::world::error::EntityMutableFetchError> > | core::error::Error::cause<enum2$<bevy_ecs::world::error::EntityMutableFetchError> > | core::error::Error::cause<enum2$<bevy_ecs::world::error::EntityMutableFetchError> > | 其余 9 个同地址别名见共享索引 @ 0x32c4f0 -> 0x2370 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::query::fetch::impl$26::init_state<country_core::resources::prefab::PrefabInstance> @ 0x32cbb7 -> 0x2370 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::components::transient_mesh::impl$4::register_required_components
// 信息 ID: adafb5fab6841e87f5b3add8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::components::transient_mesh::impl$4::register_required_components @ 0xaaca20 -> 0xaac640 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::bundle::impl$0::component_ids<country_core::components::transient_mesh::TransientMesh,bevy_ecs::bundle::impl$8::register_info::closure$0::closure_env$0<country_core::components::transient_mesh::TransientMesh> > @ 0x12e718b -> 0xaac640 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::bundle::impl$0::component_ids<country_core::components::transient_mesh::TransientMesh,bevy_ecs::bundle::impl$8::register_info::closure$0::closure_env$0<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::components::transient_mesh::TransientMesh,country_core::components::WoodenGate> > > @ 0x2870767 -> 0xaac640 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===

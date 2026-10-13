// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/startup/load_audio.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: ddab2f451476896bfcd552e5a39726cdd0c35b4f030eb9e68b5856b51f2ceabd
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: load_audio.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 577
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
// use slotmap; // namespace candidate
//   package identities: [{"name":"slotmap","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.6"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_audio::load_audio
// 记录形式: compiled_procedure; instances=1
// 源码行: 9
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":577,"record_offset":16036,"rva":11591920,"va":5380301040}
// 编译实例 RVAs: 0xb0e0f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2343186","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380301040,"raw_hex":"e812c12300","size":5,"soft_fail":false,"target_va":5382644528,"va":5380301337,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: load_audio.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5219876","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380301040,"raw_hex":"e8dc59b0ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5380301359,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067dd80
//     owner=unresolved_namespace; count=1
//     target_aliases=69 candidates; complete list: load_audio.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4785128","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380301040,"raw_hex":"e818fcb6ff","size":5,"soft_fail":false,"target_va":5375516032,"va":5380301155,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t31045420","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380301040,"raw_hex":"e82cb7d901","size":5,"soft_fail":false,"target_va":5411347040,"va":5380301615,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31239282","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380301040,"raw_hex":"e872acdc01","size":5,"soft_fail":false,"target_va":5411540963,"va":5380301676,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::system::commands::Commands::insert_resource
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t8196274","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380301040,"raw_hex":"e8b2107d00","size":5,"soft_fail":false,"target_va":5388497712,"va":5380301433,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::audio_resources::AudioSamplesLoadedPromise::recv
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-749599","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380301040,"raw_hex":"e8e18ff4ff","size":5,"soft_fail":false,"target_va":5379551488,"va":5380301082,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::time::Instant::elapsed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2447315","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380301040,"raw_hex":"e8d3572500","size":5,"soft_fail":false,"target_va":5382748432,"va":5380301112,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::time::Instant::now
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2446555","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380301040,"raw_hex":"e8db542500","size":5,"soft_fail":false,"target_va":5382747616,"va":5380301056,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":577,"record_offset":16196,"parent_record_offset":16036,"ancestor_record_offset":16036,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::commands::Commands::insert_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":577,"record_offset":17052,"parent_record_offset":16036,"ancestor_record_offset":16036,"inlinee":9431,"source_line":851,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::commands::Commands::remove_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":577,"record_offset":17344,"parent_record_offset":16036,"ancestor_record_offset":16036,"inlinee":4958,"source_line":872,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":577,"record_offset":16336,"parent_record_offset":16036,"ancestor_record_offset":16036,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::audio_trigger_library::impl$22::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/audio_trigger_library.rs
//     evidence={"module":577,"record_offset":16544,"parent_record_offset":16036,"ancestor_record_offset":16036,"inlinee":33013,"source_line":42,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::audio::game_loading_audio::impl$0::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/audio/game_loading_audio.rs
//     evidence={"module":577,"record_offset":17316,"parent_record_offset":16036,"ancestor_record_offset":16036,"inlinee":33017,"source_line":11,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":577,"record_offset":16368,"parent_record_offset":16036,"ancestor_record_offset":16036,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":577,"record_offset":16252,"parent_record_offset":16036,"ancestor_record_offset":16036,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":577,"record_offset":16892,"parent_record_offset":16868,"ancestor_record_offset":16036,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":577,"record_offset":16916,"parent_record_offset":16892,"ancestor_record_offset":16036,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":577,"record_offset":16868,"parent_record_offset":16840,"ancestor_record_offset":16036,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":577,"record_offset":17168,"parent_record_offset":17140,"ancestor_record_offset":16036,"inlinee":4962,"source_line":326,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":577,"record_offset":16780,"parent_record_offset":16752,"ancestor_record_offset":16036,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":577,"record_offset":17252,"parent_record_offset":17228,"ancestor_record_offset":16036,"inlinee":4737,"source_line":512,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":577,"record_offset":17228,"parent_record_offset":17196,"ancestor_record_offset":16036,"inlinee":4964,"source_line":622,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":577,"record_offset":17196,"parent_record_offset":17168,"ancestor_record_offset":16036,"inlinee":4963,"source_line":541,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":577,"record_offset":16840,"parent_record_offset":16808,"ancestor_record_offset":16036,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":577,"record_offset":16808,"parent_record_offset":16780,"ancestor_record_offset":16036,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":577,"record_offset":16972,"parent_record_offset":16684,"ancestor_record_offset":16036,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":577,"record_offset":17140,"parent_record_offset":17108,"ancestor_record_offset":16036,"inlinee":4961,"source_line":1271,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":577,"record_offset":16724,"parent_record_offset":16684,"ancestor_record_offset":16036,"inlinee":4632,"source_line":482,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":577,"record_offset":16752,"parent_record_offset":16724,"ancestor_record_offset":16036,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":577,"record_offset":16220,"parent_record_offset":16196,"ancestor_record_offset":16036,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":577,"record_offset":17080,"parent_record_offset":17052,"ancestor_record_offset":16036,"inlinee":4959,"source_line":553,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue_internal
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":577,"record_offset":17108,"parent_record_offset":17080,"ancestor_record_offset":16036,"inlinee":4960,"source_line":612,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::array::impl$60::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\mod.rs
//     evidence={"module":577,"record_offset":16572,"parent_record_offset":16544,"ancestor_record_offset":16036,"inlinee":21247,"source_line":455,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":577,"record_offset":16424,"parent_record_offset":16392,"ancestor_record_offset":16036,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":577,"record_offset":16452,"parent_record_offset":16392,"ancestor_record_offset":16036,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":577,"record_offset":16996,"parent_record_offset":16972,"ancestor_record_offset":16036,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":577,"record_offset":16276,"parent_record_offset":16252,"ancestor_record_offset":16036,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":577,"record_offset":16300,"parent_record_offset":16276,"ancestor_record_offset":16036,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::audio_trigger_library::impl$2::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/audio_trigger_library.rs
//     evidence={"module":577,"record_offset":16600,"parent_record_offset":16572,"ancestor_record_offset":16036,"inlinee":33015,"source_line":15,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":577,"record_offset":16480,"parent_record_offset":16392,"ancestor_record_offset":16036,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":577,"record_offset":16392,"parent_record_offset":16368,"ancestor_record_offset":16036,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":577,"record_offset":16504,"parent_record_offset":16480,"ancestor_record_offset":16036,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] slotmap::basic::SlotMap::with_capacity_and_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"slotmap","version":"1.0.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/slotmap-1.0.6/src/basic.rs
//     evidence={"module":577,"record_offset":16684,"parent_record_offset":16656,"ancestor_record_offset":16036,"inlinee":27732,"source_line":183,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] slotmap::basic::SlotMap::with_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"slotmap","version":"1.0.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/slotmap-1.0.6/src/basic.rs
//     evidence={"module":577,"record_offset":16656,"parent_record_offset":16628,"ancestor_record_offset":16036,"inlinee":27731,"source_line":161,"archive":"_pdb/B/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] slotmap::basic::impl$6::default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"slotmap","version":"1.0.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/slotmap-1.0.6/src/basic.rs
//     evidence={"module":577,"record_offset":16628,"parent_record_offset":16600,"ancestor_record_offset":16036,"inlinee":27730,"source_line":855,"archive":"_pdb/B/modules/00577.jsonl.gz"}
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
// 完整结构及原始类型记录: load_audio.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: load_audio.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: load_audio.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::audio_resources::AudioSamplesLoadedPromise>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::audio_resources::AudioSamplesLoadedPromise>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::audio_resources::AudioSamplesLoadedPromise>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands

// ------------------------------------------------------------------
// 函数: country_core::startup::load_audio::load_audio
// 信息 ID: 5a090cc5df37d2a14e2c1d86
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 4db70a03d7954a41c7ffedde: void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::audio_resources::AudioSamplesLoadedPromise>,bevy_ecs::system::commands::Commands)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::audio_resources::AudioSamplesLoadedPromise>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::audio_resources::AudioSamplesLoadedPromise>,bevy_ecs::system::commands::Commands)> > @ 0xa9d8e6 -> 0xb0e0f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::audio_resources::AudioSamplesLoadedPromise>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::audio_resources::AudioSamplesLoadedPromise>,bevy_ecs::system::commands::Commands)> > > @ 0x14b780c -> 0xb0e0f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===

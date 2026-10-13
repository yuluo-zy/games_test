// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/startup/load_audio.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 165266219630827a89d82400b7006fca4b3c9161c1b857c59ad34ce9d85fc0e6
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: load_audio.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 497
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: []
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"PDB_registry_path_only","version":"0.4.21"}]
// use slotmap; // namespace candidate
//   package identities: [{"name":"slotmap","source":"PDB_registry_path_only","version":"1.0.6"}]
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
// 首条位置: {"module":497,"record_offset":30152,"rva":7123232,"va":5375832352}
// 编译实例 RVAs: 0x6cb120
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: load_audio.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3082886","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375832352,"raw_hex":"e87af5d0ff","size":5,"soft_fail":false,"target_va":5372749696,"va":5375832577,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27144913","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375832352,"raw_hex":"e8d1329e01","size":5,"soft_fail":false,"target_va":5402978144,"va":5375833226,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t27085968","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375832352,"raw_hex":"e8904c9d01","size":5,"soft_fail":false,"target_va":5402919072,"va":5375833099,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::system::commands::Commands::insert_resource
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t8532151","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375832352,"raw_hex":"e8b7308200","size":5,"soft_fail":false,"target_va":5384364800,"va":5375832644,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::world::command_queue::CommandQueue::get_raw
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t1413672","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375832352,"raw_hex":"e828921500","size":5,"soft_fail":false,"target_va":5377246528,"va":5375832851,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::audio_resources::AudioSamplesLoadedPromise::recv
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-1150225","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375832352,"raw_hex":"e8ef72eeff","size":5,"soft_fail":false,"target_va":5374682176,"va":5375832396,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] log::__private_api::log_impl
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1521216","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375832352,"raw_hex":"e840361700","size":5,"soft_fail":false,"target_va":5377353776,"va":5375832555,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::time::Instant::elapsed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1611329","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375832352,"raw_hex":"e841961800","size":5,"soft_fail":false,"target_va":5377443760,"va":5375832426,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::time::Instant::now
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1610569","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375832352,"raw_hex":"e849931800","size":5,"soft_fail":false,"target_va":5377442944,"va":5375832370,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":497,"record_offset":30316,"parent_record_offset":30152,"ancestor_record_offset":30152,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::commands::Commands::insert_resource
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":497,"record_offset":31068,"parent_record_offset":30152,"ancestor_record_offset":30152,"inlinee":12145,"source_line":598,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::commands::Commands::remove_resource
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":497,"record_offset":32880,"parent_record_offset":30152,"ancestor_record_offset":30152,"inlinee":12139,"source_line":622,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":497,"record_offset":30456,"parent_record_offset":30152,"ancestor_record_offset":30152,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":497,"record_offset":30484,"parent_record_offset":30152,"ancestor_record_offset":30152,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::audio_trigger_library::impl$22::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/audio_trigger_library.rs
//     evidence={"module":497,"record_offset":30540,"parent_record_offset":30152,"ancestor_record_offset":30152,"inlinee":27897,"source_line":42,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::audio::game_loading_audio::impl$0::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/audio/game_loading_audio.rs
//     evidence={"module":497,"record_offset":32848,"parent_record_offset":30152,"ancestor_record_offset":30152,"inlinee":27899,"source_line":11,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":497,"record_offset":30512,"parent_record_offset":30152,"ancestor_record_offset":30152,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":497,"record_offset":30372,"parent_record_offset":30152,"ancestor_record_offset":30152,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":497,"record_offset":30852,"parent_record_offset":30828,"ancestor_record_offset":30152,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":497,"record_offset":30876,"parent_record_offset":30852,"ancestor_record_offset":30152,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":497,"record_offset":30828,"parent_record_offset":30800,"ancestor_record_offset":30152,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":497,"record_offset":31296,"parent_record_offset":31268,"ancestor_record_offset":30152,"inlinee":4277,"source_line":270,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::needs_to_grow
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":497,"record_offset":31268,"parent_record_offset":31232,"ancestor_record_offset":30152,"inlinee":4276,"source_line":414,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":497,"record_offset":31400,"parent_record_offset":31376,"ancestor_record_offset":30152,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":497,"record_offset":31232,"parent_record_offset":31200,"ancestor_record_offset":30152,"inlinee":4275,"source_line":318,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":497,"record_offset":30800,"parent_record_offset":30772,"ancestor_record_offset":30152,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":497,"record_offset":30772,"parent_record_offset":30744,"ancestor_record_offset":30152,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":497,"record_offset":30948,"parent_record_offset":30772,"ancestor_record_offset":30152,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":497,"record_offset":31376,"parent_record_offset":31152,"ancestor_record_offset":30152,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":497,"record_offset":31348,"parent_record_offset":31152,"ancestor_record_offset":30152,"inlinee":4292,"source_line":2202,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":497,"record_offset":30988,"parent_record_offset":30680,"ancestor_record_offset":30152,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":497,"record_offset":31200,"parent_record_offset":31152,"ancestor_record_offset":30152,"inlinee":4274,"source_line":944,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":497,"record_offset":31516,"parent_record_offset":31152,"ancestor_record_offset":30152,"inlinee":5993,"source_line":1455,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":497,"record_offset":30716,"parent_record_offset":30680,"ancestor_record_offset":30152,"inlinee":6967,"source_line":480,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":497,"record_offset":30744,"parent_record_offset":30716,"ancestor_record_offset":30152,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":497,"record_offset":30340,"parent_record_offset":30316,"ancestor_record_offset":30152,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::push
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":497,"record_offset":31104,"parent_record_offset":31068,"ancestor_record_offset":30152,"inlinee":12140,"source_line":501,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::command_queue::CommandQueue::push
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/command_queue.rs
//     evidence={"module":497,"record_offset":31548,"parent_record_offset":31104,"ancestor_record_offset":30152,"inlinee":12149,"source_line":78,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::command_queue::RawCommandQueue::push
//     owner=git_dependency_definition; count=6
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/command_queue.rs
//     evidence={"module":497,"record_offset":31152,"parent_record_offset":31104,"ancestor_record_offset":30152,"inlinee":12142,"source_line":157,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::array::impl$60::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\mod.rs
//     evidence={"module":497,"record_offset":30568,"parent_record_offset":30540,"ancestor_record_offset":30152,"inlinee":23309,"source_line":439,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":497,"record_offset":31480,"parent_record_offset":31456,"ancestor_record_offset":30152,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write_unaligned
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":497,"record_offset":31432,"parent_record_offset":31152,"ancestor_record_offset":30152,"inlinee":12143,"source_line":1546,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":497,"record_offset":30904,"parent_record_offset":30876,"ancestor_record_offset":30152,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":497,"record_offset":31012,"parent_record_offset":30988,"ancestor_record_offset":30152,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write_unaligned
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":497,"record_offset":31456,"parent_record_offset":31432,"ancestor_record_offset":30152,"inlinee":12144,"source_line":1579,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":497,"record_offset":30396,"parent_record_offset":30372,"ancestor_record_offset":30152,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":497,"record_offset":30420,"parent_record_offset":30396,"ancestor_record_offset":30152,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::audio_trigger_library::impl$2::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/audio_trigger_library.rs
//     evidence={"module":497,"record_offset":30596,"parent_record_offset":30568,"ancestor_record_offset":30152,"inlinee":23231,"source_line":15,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] slotmap::basic::SlotMap::with_capacity_and_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"slotmap","version":"1.0.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/slotmap-1.0.6/src/basic.rs
//     evidence={"module":497,"record_offset":30680,"parent_record_offset":30652,"ancestor_record_offset":30152,"inlinee":22651,"source_line":183,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] slotmap::basic::SlotMap::with_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"slotmap","version":"1.0.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/slotmap-1.0.6/src/basic.rs
//     evidence={"module":497,"record_offset":30652,"parent_record_offset":30624,"ancestor_record_offset":30152,"inlinee":22650,"source_line":161,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] slotmap::basic::impl$6::default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"slotmap","version":"1.0.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/slotmap-1.0.6/src/basic.rs
//     evidence={"module":497,"record_offset":30624,"parent_record_offset":30596,"ancestor_record_offset":30152,"inlinee":22648,"source_line":855,"archive":"_pdb/A/modules/00497.jsonl.gz"}
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
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::audio_resources::AudioSamplesLoadedPromise>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::audio_resources::AudioSamplesLoadedPromise>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::audio_resources::AudioSamplesLoadedPromise>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands

// ------------------------------------------------------------------
// 函数: country_core::startup::load_audio::load_audio
// 信息 ID: 3f5edd26c6f028a99ece9edd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 4db70a03d7954a41c7ffedde: void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::audio_resources::AudioSamplesLoadedPromise>,bevy_ecs::system::commands::Commands)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::audio_resources::AudioSamplesLoadedPromise>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::audio_resources::AudioSamplesLoadedPromise>,bevy_ecs::system::commands::Commands)> @ 0xdebb94 -> 0x6cb120 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::audio_resources::AudioSamplesLoadedPromise>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::audio_resources::AudioSamplesLoadedPromise>,bevy_ecs::system::commands::Commands)> > @ 0xe601ae -> 0x6cb120 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===

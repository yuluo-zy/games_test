// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/systems/backend/input/input_replay_for_trailers.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 2ef6402b1a664555808469f0dfe40052b5b563d768a8b65c1c4c6bb3236c834d
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: input_replay_for_trailers.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 458
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: []
// use bevy_input; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: []
// use egui; // namespace candidate
//   package identities: [{"name":"egui","source":"PDB_registry_path_only","version":"0.27.2"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"PDB_registry_path_only","version":"0.14.5"}]
// use lock_api; // namespace candidate
//   package identities: [{"name":"lock_api","source":"PDB_registry_path_only","version":"0.4.9"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"PDB_registry_path_only","version":"0.4.21"}]
// use parking_lot; // namespace candidate
//   package identities: [{"name":"parking_lot","source":"PDB_registry_path_only","version":"0.12.1"}]
// use rfd; // namespace candidate
//   package identities: [{"name":"rfd","source":"PDB_registry_path_only","version":"0.10.0"}]
// use serde_json; // namespace candidate
//   package identities: [{"name":"serde_json","source":"PDB_registry_path_only","version":"1.0.85"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::backend::input::input_replay_for_trailers::record_and_replay_input_for_trailers
// 记录形式: compiled_procedure; instances=1
// 源码行: 18
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":458,"record_offset":37576,"rva":6470704,"va":5375179824}
// 编译实例 RVAs: 0x62bc30
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1402dad80
//     owner=unresolved_namespace; count=1
//     target_aliases=7 candidates; complete list: input_replay_for_trailers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3478709","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375179824,"raw_hex":"e84bebcaff","size":5,"soft_fail":false,"target_va":5371702656,"va":5375181360,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14038baa0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: input_replay_for_trailers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2753401","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375179824,"raw_hex":"e887fcd5ff","size":5,"soft_fail":false,"target_va":5372426912,"va":5375180308,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=5
//     target_aliases=1 candidates; complete list: input_replay_for_trailers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2430738","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375179824,"raw_hex":"e8eee8daff","size":5,"soft_fail":false,"target_va":5372749712,"va":5375180445,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14066c360
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: input_replay_for_trailers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t263884","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375179824,"raw_hex":"e8cc060400","size":5,"soft_fail":false,"target_va":5375443808,"va":5375179919,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140852b50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: input_replay_for_trailers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2256088","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375179824,"raw_hex":"e8d86c2200","size":5,"soft_fail":false,"target_va":5377436496,"va":5375180403,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d733d0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: input_replay_for_trailers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7632511","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375179824,"raw_hex":"e87f767400","size":5,"soft_fail":false,"target_va":5382812624,"va":5375180108,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d77ac0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: input_replay_for_trailers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7650517","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375179824,"raw_hex":"e8d5bc7400","size":5,"soft_fail":false,"target_va":5382830784,"va":5375180262,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27797670","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375179824,"raw_hex":"e8a628a801","size":5,"soft_fail":false,"target_va":5402978912,"va":5375181237,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27797619","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375179824,"raw_hex":"e87328a801","size":5,"soft_fail":false,"target_va":5402978944,"va":5375181320,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::camera_rig::MainCamera::load_parameters
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-1130084","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375179824,"raw_hex":"e89cc1eeff","size":5,"soft_fail":false,"target_va":5374051104,"va":5375181183,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::camera_rig::MainCamera::store_parameters
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-1128252","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375179824,"raw_hex":"e8c4c8eeff","size":5,"soft_fail":false,"target_va":5374051888,"va":5375180135,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] log::__private_api::log_impl
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t2173689","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375179824,"raw_hex":"e8f92a2100","size":5,"soft_fail":false,"target_va":5377353776,"va":5375180082,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] country_core::systems::camera_rig::MainCamera::immediately_finish_interpolating
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tjmp\t-1128626","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5375179824,"raw_hex":"e94ec7eeff","size":5,"soft_fail":false,"target_va":5374052592,"va":5375181213,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=9
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":458,"record_offset":38356,"parent_record_offset":37576,"ancestor_record_offset":37576,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_direct] bevy_input::button_input::ButtonInput::just_pressed
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_input/src/button_input.rs
//     evidence={"module":458,"record_offset":37808,"parent_record_offset":37576,"ancestor_record_offset":37576,"inlinee":23184,"source_line":220,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":458,"record_offset":37948,"parent_record_offset":37576,"ancestor_record_offset":37576,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":458,"record_offset":37976,"parent_record_offset":37576,"ancestor_record_offset":37576,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":458,"record_offset":38524,"parent_record_offset":37576,"ancestor_record_offset":37576,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_direct] country_core::input::input_stream::InputRecordReplayState::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/input/input_stream.rs
//     evidence={"module":458,"record_offset":38068,"parent_record_offset":37576,"ancestor_record_offset":37576,"inlinee":26271,"source_line":218,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_direct] country_core::input::input_stream::InputRecordReplayState::seek_to_sync_point
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/input/input_stream.rs
//     evidence={"module":458,"record_offset":40100,"parent_record_offset":37576,"ancestor_record_offset":37576,"inlinee":25291,"source_line":236,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_direct] country_core::input::input_stream::InputStream::sync_point_count
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/input/input_stream.rs
//     evidence={"module":458,"record_offset":40016,"parent_record_offset":37576,"ancestor_record_offset":37576,"inlinee":25289,"source_line":114,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::toolbar::impl$11::clone
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/toolbar.rs
//     evidence={"module":458,"record_offset":38032,"parent_record_offset":37576,"ancestor_record_offset":37576,"inlinee":25288,"source_line":91,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":458,"record_offset":38004,"parent_record_offset":37576,"ancestor_record_offset":37576,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":458,"record_offset":37864,"parent_record_offset":37576,"ancestor_record_offset":37576,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$36::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":458,"record_offset":38412,"parent_record_offset":37576,"ancestor_record_offset":37576,"inlinee":5860,"source_line":1700,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":458,"record_offset":38788,"parent_record_offset":38764,"ancestor_record_offset":37576,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":458,"record_offset":38764,"parent_record_offset":38708,"ancestor_record_offset":37576,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":458,"record_offset":40208,"parent_record_offset":40176,"ancestor_record_offset":37576,"inlinee":7687,"source_line":667,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":458,"record_offset":40352,"parent_record_offset":40208,"ancestor_record_offset":37576,"inlinee":7712,"source_line":159,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::to_physical_idx
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":458,"record_offset":40244,"parent_record_offset":40208,"ancestor_record_offset":37576,"inlinee":7688,"source_line":200,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::wrap_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":458,"record_offset":40272,"parent_record_offset":40244,"ancestor_record_offset":37576,"inlinee":7689,"source_line":195,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::impl$12::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":458,"record_offset":40176,"parent_record_offset":40144,"ancestor_record_offset":37576,"inlinee":21861,"source_line":2771,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::impl$14::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":458,"record_offset":38140,"parent_record_offset":38116,"ancestor_record_offset":37576,"inlinee":23770,"source_line":2786,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::impl$21::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":458,"record_offset":38248,"parent_record_offset":38216,"ancestor_record_offset":37576,"inlinee":23774,"source_line":2875,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::spec_from_iter::impl$0::spec_from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\spec_from_iter.rs
//     evidence={"module":458,"record_offset":38192,"parent_record_offset":38140,"ancestor_record_offset":37576,"inlinee":23772,"source_line":12,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":458,"record_offset":38736,"parent_record_offset":38708,"ancestor_record_offset":37576,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":458,"record_offset":40388,"parent_record_offset":40352,"ancestor_record_offset":37576,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":458,"record_offset":38708,"parent_record_offset":38680,"ancestor_record_offset":37576,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::into_raw_parts_with_alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":458,"record_offset":38272,"parent_record_offset":38248,"ancestor_record_offset":37576,"inlinee":23722,"source_line":900,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":458,"record_offset":38216,"parent_record_offset":38192,"ancestor_record_offset":37576,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$24::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":458,"record_offset":40728,"parent_record_offset":40700,"ancestor_record_offset":37576,"inlinee":4700,"source_line":3200,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=9
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":458,"record_offset":38380,"parent_record_offset":38356,"ancestor_record_offset":37576,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":458,"record_offset":40488,"parent_record_offset":40100,"ancestor_record_offset":37576,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\collect.rs
//     evidence={"module":458,"record_offset":38164,"parent_record_offset":38140,"ancestor_record_offset":37576,"inlinee":7553,"source_line":321,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":458,"record_offset":38116,"parent_record_offset":38092,"ancestor_record_offset":37576,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\manually_drop.rs
//     evidence={"module":458,"record_offset":38296,"parent_record_offset":38272,"ancestor_record_offset":37576,"inlinee":4174,"source_line":70,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":458,"record_offset":40936,"parent_record_offset":40900,"ancestor_record_offset":37576,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":458,"record_offset":40300,"parent_record_offset":40272,"ancestor_record_offset":37576,"inlinee":7690,"source_line":1639,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":458,"record_offset":40460,"parent_record_offset":40100,"ancestor_record_offset":37576,"inlinee":4907,"source_line":1680,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":458,"record_offset":40424,"parent_record_offset":40176,"ancestor_record_offset":37576,"inlinee":5324,"source_line":893,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":458,"record_offset":38548,"parent_record_offset":38524,"ancestor_record_offset":37576,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":458,"record_offset":37888,"parent_record_offset":37864,"ancestor_record_offset":37576,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":458,"record_offset":37912,"parent_record_offset":37888,"ancestor_record_offset":37576,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] country_core::input::input_stream::InputStream::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/input/input_stream.rs
//     evidence={"module":458,"record_offset":38092,"parent_record_offset":38068,"ancestor_record_offset":37576,"inlinee":25297,"source_line":103,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] country_core::input::input_stream::InputStream::sync_point
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/input/input_stream.rs
//     evidence={"module":458,"record_offset":40144,"parent_record_offset":40100,"ancestor_record_offset":37576,"inlinee":25292,"source_line":118,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::contains
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/set.rs
//     evidence={"module":458,"record_offset":37832,"parent_record_offset":37808,"ancestor_record_offset":37576,"inlinee":8703,"source_line":861,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::OsStr::to_os_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":458,"record_offset":38436,"parent_record_offset":38412,"ancestor_record_offset":37576,"inlinee":5424,"source_line":884,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::backend::input::input_replay_for_trailers::do_input_replay_ui
// 记录形式: compiled_procedure; instances=1
// 源码行: 67
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":458,"record_offset":41388,"rva":6472400,"va":5375181520}
// 编译实例 RVAs: 0x62c2d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$egui..widgets..button..Checkbox$u20$as$u20$egui..widgets..Widget$GT$::ui
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t1378693","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375181520,"raw_hex":"e885091500","size":5,"soft_fail":false,"target_va":5376560464,"va":5375181766,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$egui..widgets..label..Label$u20$as$u20$egui..widgets..Widget$GT$::ui
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t1491856","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375181520,"raw_hex":"e890c31600","size":5,"soft_fail":false,"target_va":5376674064,"va":5375182203,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400e16c0
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: input_replay_for_trailers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5549628","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375181520,"raw_hex":"e8c451abff","size":5,"soft_fail":false,"target_va":5369632448,"va":5375182071,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=6
//     target_aliases=1 candidates; complete list: input_replay_for_trailers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2431905","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375181520,"raw_hex":"e85fe4daff","size":5,"soft_fail":false,"target_va":5372749696,"va":5375181596,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: input_replay_for_trailers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2432335","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375181520,"raw_hex":"e8b1e2daff","size":5,"soft_fail":false,"target_va":5372749712,"va":5375182042,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406ce0b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: input_replay_for_trailers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t661588","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375181520,"raw_hex":"e854180a00","size":5,"soft_fail":false,"target_va":5375844528,"va":5375182935,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ea0c90
//     owner=unresolved_namespace; count=6
//     target_aliases=2 candidates; complete list: input_replay_for_trailers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8865970","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375181520,"raw_hex":"e8b2488700","size":5,"soft_fail":false,"target_va":5384047760,"va":5375181785,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t27795149","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375181520,"raw_hex":"e8cd1ea801","size":5,"soft_fail":false,"target_va":5402978144,"va":5375182990,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t2347988","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375181520,"raw_hex":"e8d4d32300","size":5,"soft_fail":false,"target_va":5377529888,"va":5375181895,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":458,"record_offset":42644,"parent_record_offset":41388,"ancestor_record_offset":41388,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_direct] alloc::string::impl$44::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":458,"record_offset":42728,"parent_record_offset":41388,"ancestor_record_offset":41388,"inlinee":4938,"source_line":2705,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":458,"record_offset":41584,"parent_record_offset":41388,"ancestor_record_offset":41388,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":458,"record_offset":42364,"parent_record_offset":41388,"ancestor_record_offset":41388,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_direct] country_core::input::input_stream::InputStream::frame_count
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/input/input_stream.rs
//     evidence={"module":458,"record_offset":42616,"parent_record_offset":41388,"ancestor_record_offset":41388,"inlinee":26272,"source_line":122,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_direct] egui::ui::Ui::checkbox
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"egui","version":"0.27.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/egui-0.27.2/src/ui.rs
//     evidence={"module":458,"record_offset":41640,"parent_record_offset":41388,"ancestor_record_offset":41388,"inlinee":6453,"source_line":1468,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_direct] egui::ui::Ui::horizontal
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/egui-0.27.2/src/ui.rs
//     evidence={"module":458,"record_offset":47088,"parent_record_offset":41388,"ancestor_record_offset":41388,"inlinee":5840,"source_line":1908,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_direct] egui::ui::Ui::label
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"egui","version":"0.27.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/egui-0.27.2/src/ui.rs
//     evidence={"module":458,"record_offset":43560,"parent_record_offset":41388,"ancestor_record_offset":41388,"inlinee":5804,"source_line":1298,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":458,"record_offset":42128,"parent_record_offset":42104,"ancestor_record_offset":41388,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":458,"record_offset":42152,"parent_record_offset":42128,"ancestor_record_offset":41388,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":458,"record_offset":43512,"parent_record_offset":43488,"ancestor_record_offset":41388,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":458,"record_offset":47144,"parent_record_offset":47116,"ancestor_record_offset":41388,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":458,"record_offset":42104,"parent_record_offset":42076,"ancestor_record_offset":41388,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":458,"record_offset":43488,"parent_record_offset":43432,"ancestor_record_offset":41388,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":458,"record_offset":47116,"parent_record_offset":47088,"ancestor_record_offset":41388,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":458,"record_offset":42692,"parent_record_offset":42668,"ancestor_record_offset":41388,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":458,"record_offset":43460,"parent_record_offset":43432,"ancestor_record_offset":41388,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":458,"record_offset":42076,"parent_record_offset":42048,"ancestor_record_offset":41388,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":458,"record_offset":42048,"parent_record_offset":42020,"ancestor_record_offset":41388,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":458,"record_offset":42220,"parent_record_offset":42048,"ancestor_record_offset":41388,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":458,"record_offset":43432,"parent_record_offset":43404,"ancestor_record_offset":41388,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":458,"record_offset":41984,"parent_record_offset":41956,"ancestor_record_offset":41388,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":458,"record_offset":41956,"parent_record_offset":41928,"ancestor_record_offset":41388,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":458,"record_offset":41900,"parent_record_offset":41872,"ancestor_record_offset":41388,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":458,"record_offset":41928,"parent_record_offset":41900,"ancestor_record_offset":41388,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":458,"record_offset":41872,"parent_record_offset":41844,"ancestor_record_offset":41388,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":458,"record_offset":41844,"parent_record_offset":41816,"ancestor_record_offset":41388,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$44::from
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":458,"record_offset":41816,"parent_record_offset":41788,"ancestor_record_offset":41388,"inlinee":4938,"source_line":2705,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$46::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":458,"record_offset":43712,"parent_record_offset":43688,"ancestor_record_offset":41388,"inlinee":6525,"source_line":2729,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":458,"record_offset":42480,"parent_record_offset":42444,"ancestor_record_offset":41388,"inlinee":4296,"source_line":1790,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":458,"record_offset":42444,"parent_record_offset":42416,"ancestor_record_offset":41388,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":458,"record_offset":42020,"parent_record_offset":41984,"ancestor_record_offset":41388,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":458,"record_offset":41608,"parent_record_offset":41584,"ancestor_record_offset":41388,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":458,"record_offset":41704,"parent_record_offset":41668,"ancestor_record_offset":41388,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":458,"record_offset":42280,"parent_record_offset":42256,"ancestor_record_offset":41388,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":458,"record_offset":42668,"parent_record_offset":42644,"ancestor_record_offset":41388,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":458,"record_offset":42256,"parent_record_offset":41984,"ancestor_record_offset":41388,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=21
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":458,"record_offset":42388,"parent_record_offset":42364,"ancestor_record_offset":41388,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":458,"record_offset":42512,"parent_record_offset":42480,"ancestor_record_offset":41388,"inlinee":4298,"source_line":398,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":458,"record_offset":42176,"parent_record_offset":42152,"ancestor_record_offset":41388,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":458,"record_offset":42540,"parent_record_offset":42480,"ancestor_record_offset":41388,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":458,"record_offset":42572,"parent_record_offset":42444,"ancestor_record_offset":41388,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] egui::widget_text::RichText::new
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"egui","version":"0.27.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/egui-0.27.2/src/widget_text.rs
//     evidence={"module":458,"record_offset":41760,"parent_record_offset":41732,"ancestor_record_offset":41388,"inlinee":5819,"source_line":79,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] egui::widget_text::impl$8::from
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"egui","version":"0.27.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/egui-0.27.2/src/widget_text.rs
//     evidence={"module":458,"record_offset":41732,"parent_record_offset":41704,"ancestor_record_offset":41388,"inlinee":5817,"source_line":693,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] egui::widget_text::impl$9::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/egui-0.27.2/src/widget_text.rs
//     evidence={"module":458,"record_offset":43636,"parent_record_offset":43612,"ancestor_record_offset":41388,"inlinee":6523,"source_line":700,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] egui::widgets::button::Checkbox::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"egui","version":"0.27.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/egui-0.27.2/src/widgets/button.rs
//     evidence={"module":458,"record_offset":41668,"parent_record_offset":41640,"ancestor_record_offset":41388,"inlinee":6455,"source_line":363,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] egui::widgets::label::Label::new
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"egui","version":"0.27.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/egui-0.27.2/src/widgets/label.rs
//     evidence={"module":458,"record_offset":43584,"parent_record_offset":43560,"ancestor_record_offset":41388,"inlinee":5806,"source_line":32,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::backend::input::input_replay_for_trailers::do_input_replay_ui::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 94
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x669E
// 首条位置: {"module":458,"inlinee":26270}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":458,"record_offset":26788,"parent_record_offset":21676,"ancestor_record_offset":21676,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":458,"record_offset":26816,"parent_record_offset":21676,"ancestor_record_offset":21676,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=21
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":458,"record_offset":22804,"parent_record_offset":21676,"ancestor_record_offset":21676,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::is_ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":458,"record_offset":26648,"parent_record_offset":21676,"ancestor_record_offset":21676,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":458,"record_offset":25168,"parent_record_offset":21676,"ancestor_record_offset":21676,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_direct] egui::response::Response::clicked
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"egui","version":"0.27.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/egui-0.27.2/src/response.rs
//     evidence={"module":458,"record_offset":21940,"parent_record_offset":21676,"ancestor_record_offset":21676,"inlinee":5782,"source_line":137,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_direct] egui::ui::Ui::button
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"egui","version":"0.27.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/egui-0.27.2/src/ui.rs
//     evidence={"module":458,"record_offset":21912,"parent_record_offset":21676,"ancestor_record_offset":21676,"inlinee":6450,"source_line":1450,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":458,"record_offset":26844,"parent_record_offset":21676,"ancestor_record_offset":21676,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":458,"record_offset":26704,"parent_record_offset":21676,"ancestor_record_offset":21676,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_direct] rfd::file_dialog::FileDialog::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"rfd","version":"0.10.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/rfd-0.10.0/src/file_dialog.rs
//     evidence={"module":458,"record_offset":24916,"parent_record_offset":21676,"ancestor_record_offset":21676,"inlinee":13180,"source_line":33,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_direct] serde_json::ser::to_string_pretty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":458,"record_offset":25212,"parent_record_offset":21676,"ancestor_record_offset":21676,"inlinee":13183,"source_line":2236,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_direct] std::fs::create_dir_all
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":458,"record_offset":24024,"parent_record_offset":21676,"ancestor_record_offset":21676,"inlinee":4768,"source_line":2306,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_direct] std::fs::read_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":458,"record_offset":28980,"parent_record_offset":21676,"ancestor_record_offset":21676,"inlinee":11086,"source_line":302,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_direct] std::fs::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":458,"record_offset":26016,"parent_record_offset":21676,"ancestor_record_offset":21676,"inlinee":10620,"source_line":339,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::canonicalize
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":458,"record_offset":25112,"parent_record_offset":21676,"ancestor_record_offset":21676,"inlinee":4742,"source_line":2788,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$30::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":458,"record_offset":23108,"parent_record_offset":21676,"ancestor_record_offset":21676,"inlinee":4732,"source_line":1627,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$36::from
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":458,"record_offset":23044,"parent_record_offset":21676,"ancestor_record_offset":21676,"inlinee":5860,"source_line":1700,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$44::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":458,"record_offset":24972,"parent_record_offset":21676,"ancestor_record_offset":21676,"inlinee":4490,"source_line":1780,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":458,"record_offset":23520,"parent_record_offset":23496,"ancestor_record_offset":21676,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":458,"record_offset":23544,"parent_record_offset":23520,"ancestor_record_offset":21676,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=21
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":458,"record_offset":24524,"parent_record_offset":24500,"ancestor_record_offset":21676,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":458,"record_offset":23496,"parent_record_offset":23460,"ancestor_record_offset":21676,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=21
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":458,"record_offset":24500,"parent_record_offset":24444,"ancestor_record_offset":21676,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":458,"record_offset":30244,"parent_record_offset":30216,"ancestor_record_offset":21676,"inlinee":4113,"source_line":1240,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":458,"record_offset":24472,"parent_record_offset":24444,"ancestor_record_offset":21676,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":458,"record_offset":23460,"parent_record_offset":23432,"ancestor_record_offset":21676,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":458,"record_offset":23432,"parent_record_offset":23404,"ancestor_record_offset":21676,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":458,"record_offset":23616,"parent_record_offset":23432,"ancestor_record_offset":21676,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":458,"record_offset":24444,"parent_record_offset":24416,"ancestor_record_offset":21676,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":458,"record_offset":23364,"parent_record_offset":23328,"ancestor_record_offset":21676,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":458,"record_offset":23328,"parent_record_offset":23292,"ancestor_record_offset":21676,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":458,"record_offset":23292,"parent_record_offset":23256,"ancestor_record_offset":21676,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":458,"record_offset":22092,"parent_record_offset":22060,"ancestor_record_offset":21676,"inlinee":4296,"source_line":1790,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":458,"record_offset":22060,"parent_record_offset":22028,"ancestor_record_offset":21676,"inlinee":4586,"source_line":2105,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":458,"record_offset":22872,"parent_record_offset":22848,"ancestor_record_offset":21676,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":458,"record_offset":25296,"parent_record_offset":25248,"ancestor_record_offset":21676,"inlinee":6967,"source_line":480,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":458,"record_offset":23404,"parent_record_offset":23364,"ancestor_record_offset":21676,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$10::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":458,"record_offset":23256,"parent_record_offset":23216,"ancestor_record_offset":21676,"inlinee":4740,"source_line":2780,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":458,"record_offset":22344,"parent_record_offset":22320,"ancestor_record_offset":21676,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::clone::impls::impl$19::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\clone.rs
//     evidence={"module":458,"record_offset":23728,"parent_record_offset":23216,"ancestor_record_offset":21676,"inlinee":4516,"source_line":219,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":458,"record_offset":26048,"parent_record_offset":26016,"ancestor_record_offset":21676,"inlinee":4750,"source_line":701,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":458,"record_offset":23680,"parent_record_offset":23656,"ancestor_record_offset":21676,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":458,"record_offset":23656,"parent_record_offset":23364,"ancestor_record_offset":21676,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=100
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":458,"record_offset":22436,"parent_record_offset":22028,"ancestor_record_offset":21676,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":458,"record_offset":22116,"parent_record_offset":22092,"ancestor_record_offset":21676,"inlinee":4298,"source_line":398,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":458,"record_offset":23572,"parent_record_offset":23544,"ancestor_record_offset":21676,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_err
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":458,"record_offset":22256,"parent_record_offset":22172,"ancestor_record_offset":21676,"inlinee":5288,"source_line":581,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":458,"record_offset":22280,"parent_record_offset":22256,"ancestor_record_offset":21676,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":458,"record_offset":22516,"parent_record_offset":22484,"ancestor_record_offset":21676,"inlinee":5796,"source_line":2580,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":458,"record_offset":22200,"parent_record_offset":22172,"ancestor_record_offset":21676,"inlinee":5726,"source_line":2629,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":458,"record_offset":26728,"parent_record_offset":26704,"ancestor_record_offset":21676,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":458,"record_offset":22540,"parent_record_offset":22516,"ancestor_record_offset":21676,"inlinee":5298,"source_line":3369,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":458,"record_offset":22224,"parent_record_offset":22200,"ancestor_record_offset":21676,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":458,"record_offset":26752,"parent_record_offset":26728,"ancestor_record_offset":21676,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":458,"record_offset":22968,"parent_record_offset":22908,"ancestor_record_offset":21676,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":458,"record_offset":23000,"parent_record_offset":22872,"ancestor_record_offset":21676,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] egui::context::Context::input
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"egui","version":"0.27.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/egui-0.27.2/src/context.rs
//     evidence={"module":458,"record_offset":22000,"parent_record_offset":21972,"ancestor_record_offset":21676,"inlinee":5798,"source_line":795,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] egui::context::Context::write
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"egui","version":"0.27.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/egui-0.27.2/src/context.rs
//     evidence={"module":458,"record_offset":22028,"parent_record_offset":22000,"ancestor_record_offset":21676,"inlinee":5785,"source_line":717,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] egui::context::impl$13::input::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"egui","version":"0.27.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/egui-0.27.2/src/context.rs
//     evidence={"module":458,"record_offset":22380,"parent_record_offset":22028,"ancestor_record_offset":21676,"inlinee":5800,"source_line":796,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] egui::response::Response::clicked_by
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"egui","version":"0.27.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/egui-0.27.2/src/response.rs
//     evidence={"module":458,"record_offset":21972,"parent_record_offset":21940,"ancestor_record_offset":21676,"inlinee":5797,"source_line":150,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] egui::response::impl$0::clicked_by::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"egui","version":"0.27.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/egui-0.27.2/src/response.rs
//     evidence={"module":458,"record_offset":22404,"parent_record_offset":22380,"ancestor_record_offset":21676,"inlinee":5802,"source_line":151,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::RwLock::write
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":458,"record_offset":22148,"parent_record_offset":22060,"ancestor_record_offset":21676,"inlinee":5786,"source_line":479,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::RwLockWriteGuard::map
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":458,"record_offset":22320,"parent_record_offset":22060,"ancestor_record_offset":21676,"inlinee":5789,"source_line":1494,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::impl$52::drop
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":458,"record_offset":22460,"parent_record_offset":22436,"ancestor_record_offset":21676,"inlinee":5794,"source_line":2576,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::lock_exclusive
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":458,"record_offset":22172,"parent_record_offset":22148,"ancestor_record_offset":21676,"inlinee":5787,"source_line":67,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::unlock_exclusive
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":458,"record_offset":22484,"parent_record_offset":22460,"ancestor_record_offset":21676,"inlinee":5795,"source_line":94,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] rfd::file_dialog::impl$7::default
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"rfd","version":"0.10.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/rfd-0.10.0/src/file_dialog.rs
//     evidence={"module":458,"record_offset":24940,"parent_record_offset":24916,"ancestor_record_offset":21676,"inlinee":13182,"source_line":18,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Serializer::pretty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":458,"record_offset":25596,"parent_record_offset":25568,"ancestor_record_offset":21676,"inlinee":13186,"source_line":36,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Serializer::with_formatter
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":458,"record_offset":25620,"parent_record_offset":25596,"ancestor_record_offset":21676,"inlinee":10188,"source_line":49,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::to_vec_pretty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":458,"record_offset":25248,"parent_record_offset":25212,"ancestor_record_offset":21676,"inlinee":13184,"source_line":2201,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::to_writer_pretty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":458,"record_offset":25568,"parent_record_offset":25248,"ancestor_record_offset":21676,"inlinee":13185,"source_line":2168,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::OsStr::to_os_string
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":458,"record_offset":23068,"parent_record_offset":23044,"ancestor_record_offset":21676,"inlinee":5424,"source_line":884,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$10::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":458,"record_offset":23144,"parent_record_offset":23108,"ancestor_record_offset":21676,"inlinee":4734,"source_line":605,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$55::as_ref
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":458,"record_offset":24156,"parent_record_offset":24132,"ancestor_record_offset":21676,"inlinee":4494,"source_line":1560,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":458,"record_offset":24204,"parent_record_offset":24180,"ancestor_record_offset":21676,"inlinee":4498,"source_line":562,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":458,"record_offset":24180,"parent_record_offset":24156,"ancestor_record_offset":21676,"inlinee":4496,"source_line":580,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::DirBuilder::create
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":458,"record_offset":24252,"parent_record_offset":24024,"ancestor_record_offset":21676,"inlinee":4771,"source_line":2561,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::DirBuilder::recursive
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":458,"record_offset":24056,"parent_record_offset":24024,"ancestor_record_offset":21676,"inlinee":4770,"source_line":2537,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::canonicalize
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":458,"record_offset":25136,"parent_record_offset":25112,"ancestor_record_offset":21676,"inlinee":4744,"source_line":2220,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":458,"record_offset":24132,"parent_record_offset":24108,"ancestor_record_offset":21676,"inlinee":4492,"source_line":2041,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$44::deref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":458,"record_offset":24108,"parent_record_offset":24084,"ancestor_record_offset":21676,"inlinee":4490,"source_line":1780,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$79::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":458,"record_offset":24084,"parent_record_offset":24024,"ancestor_record_offset":21676,"inlinee":4752,"source_line":3182,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::impl$9::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":458,"record_offset":23180,"parent_record_offset":23144,"ancestor_record_offset":21676,"inlinee":4736,"source_line":12,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] std::sys_common::wtf8::impl$34::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys_common\wtf8.rs
//     evidence={"module":458,"record_offset":23216,"parent_record_offset":23180,"ancestor_record_offset":21676,"inlinee":4738,"source_line":135,"archive":"_pdb/A/modules/00458.jsonl.gz"}
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
// 完整结构及原始类型记录: input_replay_for_trailers.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: input_replay_for_trailers.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: input_replay_for_trailers.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 2; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::ivy::ivy_leaf_storage::IvyLeafStorage>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::startup::load_glade_settings::GladeSettings>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::ivy::ivy_leaf_storage::IvyLeafStorage>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::Res<country_core::startup::load_glade_settings::GladeSettings>", "bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::ivy::ivy_leaf_storage::IvyLeafStorage>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::startup::load_glade_settings::GladeSettings>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::input::input_stream::InputRecordReplayState>,bevy_ecs::change_detection::Res<bevy_input::button_input::ButtonInput<enum2$<bevy_input::keyboard::KeyCode> > >,bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::Toolbar>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::input::input_stream::InputRecordReplayState>", "bevy_ecs::change_detection::Res<bevy_input::button_input::ButtonInput<enum2$<bevy_input::keyboard::KeyCode> > >", "bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>", "bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>", "bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::Toolbar>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::input::input_stream::InputRecordReplayState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_input::button_input::ButtonInput<enum2$<bevy_input::keyboard::KeyCode> > >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::Toolbar>

// ------------------------------------------------------------------
// 函数: country_core::systems::backend::input::input_replay_for_trailers::do_input_replay_ui
// 信息 ID: 73a88e01bba95e9a22a69ff0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::ops::function::FnOnce::call_once<tiny_glade::gui::gui_debug::do_gui_debug::closure$0::closure_env$13,tuple$<ref_mut$<egui::ui::Ui> > > @ 0x3c8bb -> 0x62c2d0 (direct_jump_outside_procedure); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::backend::input::input_replay_for_trailers::do_input_replay_ui::closure$0
// 信息 ID: 11408296661ce09c5238833a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::backend::input::input_replay_for_trailers::do_input_replay_ui", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::backend::input::input_replay_for_trailers::do_input_replay_ui::closure_env$0 <- core::ops::function::FnOnce::call_once<country_core::systems::backend::input::input_replay_for_trailers::do_input_replay_ui::closure_env$0,tuple$<ref_mut$<egui::ui::Ui> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::backend::input::input_replay_for_trailers::do_input_replay_ui::closure_env$0 <- core::ptr::drop_in_place<country_core::systems::backend::input::input_replay_for_trailers::do_input_replay_ui::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<country_core::systems::backend::input::input_replay_for_trailers::do_input_replay_ui::closure_env$0,tuple$<ref_mut$<egui::ui::Ui> > >; module=458, inline_record=21676; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::backend::input::input_replay_for_trailers::record_and_replay_input_for_trailers
// 信息 ID: 89853462c32a2cdbe7df2596
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 55cdcc6aa7c329956db2e394: void (*)(bevy_ecs::change_detection::ResMut<country_core::input::input_stream::InputRecordReplayState>,bevy_ecs::change_detection::Res<bevy_input::button_input::ButtonInput<enum2$<bevy_input::keyboard::KeyCode> > >,bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::Toolbar>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::input::input_stream::InputRecordReplayState>,bevy_ecs::change_detection::Res<bevy_input::button_input::ButtonInput<enum2$<bevy_input::keyboard::KeyCode> > >,bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::Toolbar>),void (*)(bevy_ecs::change_detection::ResMut<country_core::input::input_stream::InputRecordReplayState>,bevy_ecs::change_detection::Res<bevy_input::button_input::ButtonInput<enum2$<bevy_input::keyboard::KeyCode> > >,bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::Toolbar>)> > @ 0xe79fee -> 0x62bc30 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::ResMut<country_core::input::input_stream::InputRecordReplayState>,bevy_ecs::change_detection::Res<bevy_input::button_input::ButtonInput<enum2$<bevy_input::keyboard::KeyCode> > >,bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::Toolbar>),void (*)(bevy_ecs::change_detection::ResMut<country_core::input::input_stream::InputRecordReplayState>,bevy_ecs::change_detection::Res<bevy_input::button_input::ButtonInput<enum2$<bevy_input::keyboard::KeyCode> > >,bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::Toolbar>)> @ 0x1087e1c -> 0x62bc30 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===

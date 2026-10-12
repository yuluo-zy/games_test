// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/systems/backend/input/input_replay_for_trailers.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 7e31a9f4b1a5fa97dc3117d07d2f91f3d310d5a29af98f2b99fb47ef6d399255
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: input_replay_for_trailers.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 476
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use bevy_input; // namespace candidate
//   package identities: [{"name":"bevy_input","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use bevy_platform; // namespace candidate
//   package identities: [{"name":"bevy_platform","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use egui; // namespace candidate
//   package identities: [{"name":"egui","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.27.2"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.27"}]
// use rfd; // namespace candidate
//   package identities: [{"name":"rfd","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.10.0"}]
// use serde_json; // namespace candidate
//   package identities: [{"name":"serde_json","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.85"}]
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
// 首条位置: {"module":476,"record_offset":46248,"rva":8635936,"va":5377345056}
// 编译实例 RVAs: 0x83c620
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t5299127","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377345056,"raw_hex":"e8b7db5000","size":5,"soft_fail":false,"target_va":5382644528,"va":5377345396,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14014b1c0
//     owner=unresolved_namespace; count=1
//     target_aliases=12 candidates; complete list: input_replay_for_trailers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7281525","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377345056,"raw_hex":"e88be490ff","size":5,"soft_fail":false,"target_va":5370065344,"va":5377346864,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140602160
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: input_replay_for_trailers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2336494","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377345056,"raw_hex":"e81259dcff","size":5,"soft_fail":false,"target_va":5375009120,"va":5377345609,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=5
//     target_aliases=1 candidates; complete list: input_replay_for_trailers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2264273","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377345056,"raw_hex":"e82f73ddff","size":5,"soft_fail":false,"target_va":5375081504,"va":5377345772,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067dd80
//     owner=unresolved_namespace; count=3
//     target_aliases=69 candidates; complete list: input_replay_for_trailers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1829220","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377345056,"raw_hex":"e89c16e4ff","size":5,"soft_fail":false,"target_va":5375516032,"va":5377345247,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14089aef0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: input_replay_for_trailers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t387180","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377345056,"raw_hex":"e86ce80500","size":5,"soft_fail":false,"target_va":5377732336,"va":5377345151,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d61b00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: input_replay_for_trailers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5395024","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377345056,"raw_hex":"e850525200","size":5,"soft_fail":false,"target_va":5382740736,"va":5377345707,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1412f7fa0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: input_replay_for_trailers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11253767","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377345056,"raw_hex":"e807b8ab00","size":5,"soft_fail":false,"target_va":5388599200,"va":5377345428,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141301640
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: input_replay_for_trailers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11292193","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377345056,"raw_hex":"e8214eac00","size":5,"soft_fail":false,"target_va":5388637760,"va":5377345562,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t34194988","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377345056,"raw_hex":"e82cc60902","size":5,"soft_fail":false,"target_va":5411541808,"va":5377346815,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t34195108","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377345056,"raw_hex":"e8a4c60902","size":5,"soft_fail":false,"target_va":5411541904,"va":5377346791,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::camera_rig::MainCamera::immediately_finish_interpolating
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t872131","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377345056,"raw_hex":"e8c34e0d00","size":5,"soft_fail":false,"target_va":5378218832,"va":5377346696,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::camera_rig::MainCamera::load_parameters
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t870558","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377345056,"raw_hex":"e89e480d00","size":5,"soft_fail":false,"target_va":5378217248,"va":5377346685,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::camera_rig::MainCamera::store_parameters
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t872291","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377345056,"raw_hex":"e8634f0d00","size":5,"soft_fail":false,"target_va":5378217744,"va":5377345448,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":476,"record_offset":47232,"parent_record_offset":46248,"ancestor_record_offset":46248,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_direct] bevy_input::button_input::ButtonInput::just_pressed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_input","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_input-0.16.0/src/button_input.rs
//     evidence={"module":476,"record_offset":46476,"parent_record_offset":46248,"ancestor_record_offset":46248,"inlinee":26137,"source_line":188,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":476,"record_offset":46700,"parent_record_offset":46248,"ancestor_record_offset":46248,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":476,"record_offset":47400,"parent_record_offset":46248,"ancestor_record_offset":46248,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_direct] country_core::input::input_stream::InputRecordReplayState::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/input/input_stream.rs
//     evidence={"module":476,"record_offset":46944,"parent_record_offset":46248,"ancestor_record_offset":46248,"inlinee":26142,"source_line":224,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_direct] country_core::input::input_stream::InputRecordReplayState::seek_to_sync_point
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/input/input_stream.rs
//     evidence={"module":476,"record_offset":49344,"parent_record_offset":46248,"ancestor_record_offset":46248,"inlinee":26153,"source_line":242,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_direct] country_core::input::input_stream::InputStream::sync_point_count
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/input/input_stream.rs
//     evidence={"module":476,"record_offset":49260,"parent_record_offset":46248,"ancestor_record_offset":46248,"inlinee":26152,"source_line":119,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::toolbar::impl$12::clone
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/toolbar.rs
//     evidence={"module":476,"record_offset":46908,"parent_record_offset":46248,"ancestor_record_offset":46248,"inlinee":26140,"source_line":87,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":476,"record_offset":46732,"parent_record_offset":46248,"ancestor_record_offset":46248,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":476,"record_offset":46616,"parent_record_offset":46248,"ancestor_record_offset":46248,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$37::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":476,"record_offset":47288,"parent_record_offset":46248,"ancestor_record_offset":46248,"inlinee":5169,"source_line":1825,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":476,"record_offset":47672,"parent_record_offset":47648,"ancestor_record_offset":46248,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":476,"record_offset":47648,"parent_record_offset":47592,"ancestor_record_offset":46248,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":476,"record_offset":49544,"parent_record_offset":49516,"ancestor_record_offset":46248,"inlinee":4735,"source_line":769,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":476,"record_offset":49452,"parent_record_offset":49420,"ancestor_record_offset":46248,"inlinee":4910,"source_line":685,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":476,"record_offset":49656,"parent_record_offset":49452,"ancestor_record_offset":46248,"inlinee":4911,"source_line":161,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::to_physical_idx
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":476,"record_offset":49488,"parent_record_offset":49452,"ancestor_record_offset":46248,"inlinee":4738,"source_line":216,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::wrap_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":476,"record_offset":49516,"parent_record_offset":49488,"ancestor_record_offset":46248,"inlinee":4739,"source_line":211,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::impl$12::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":476,"record_offset":49420,"parent_record_offset":49388,"ancestor_record_offset":46248,"inlinee":11302,"source_line":2924,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::impl$14::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":476,"record_offset":47016,"parent_record_offset":46992,"ancestor_record_offset":46248,"inlinee":26146,"source_line":2940,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::impl$21::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":476,"record_offset":47124,"parent_record_offset":47092,"ancestor_record_offset":46248,"inlinee":26150,"source_line":3051,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::spec_from_iter::impl$0::spec_from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\spec_from_iter.rs
//     evidence={"module":476,"record_offset":47068,"parent_record_offset":47016,"ancestor_record_offset":46248,"inlinee":26148,"source_line":13,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":476,"record_offset":49588,"parent_record_offset":49544,"ancestor_record_offset":46248,"inlinee":4736,"source_line":294,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":476,"record_offset":49692,"parent_record_offset":49656,"ancestor_record_offset":46248,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":476,"record_offset":49612,"parent_record_offset":49588,"ancestor_record_offset":46248,"inlinee":4737,"source_line":512,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":476,"record_offset":47620,"parent_record_offset":47592,"ancestor_record_offset":46248,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":476,"record_offset":47592,"parent_record_offset":47568,"ancestor_record_offset":46248,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":476,"record_offset":49740,"parent_record_offset":49716,"ancestor_record_offset":46248,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":476,"record_offset":49716,"parent_record_offset":49692,"ancestor_record_offset":46248,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":476,"record_offset":47568,"parent_record_offset":47544,"ancestor_record_offset":46248,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::into_raw_parts_with_alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":476,"record_offset":47148,"parent_record_offset":47124,"ancestor_record_offset":46248,"inlinee":26151,"source_line":1176,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":476,"record_offset":47092,"parent_record_offset":47068,"ancestor_record_offset":46248,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$25::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":476,"record_offset":50084,"parent_record_offset":50060,"ancestor_record_offset":46248,"inlinee":4970,"source_line":3781,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":476,"record_offset":47256,"parent_record_offset":47232,"ancestor_record_offset":46248,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] bevy_platform::collections::hash_set::HashSet::contains
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_platform","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_platform-0.16.0/src/collections/hash_set.rs
//     evidence={"module":476,"record_offset":46500,"parent_record_offset":46476,"ancestor_record_offset":46248,"inlinee":26138,"source_line":730,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":476,"record_offset":49848,"parent_record_offset":49344,"ancestor_record_offset":46248,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\collect.rs
//     evidence={"module":476,"record_offset":47040,"parent_record_offset":47016,"ancestor_record_offset":46248,"inlinee":4756,"source_line":322,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":476,"record_offset":46992,"parent_record_offset":46968,"ancestor_record_offset":46248,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\manually_drop.rs
//     evidence={"module":476,"record_offset":47172,"parent_record_offset":47148,"ancestor_record_offset":46248,"inlinee":5075,"source_line":180,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":476,"record_offset":50312,"parent_record_offset":50276,"ancestor_record_offset":46248,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":476,"record_offset":49820,"parent_record_offset":49344,"ancestor_record_offset":46248,"inlinee":5005,"source_line":2021,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":476,"record_offset":49784,"parent_record_offset":49420,"ancestor_record_offset":46248,"inlinee":5587,"source_line":930,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":476,"record_offset":46572,"parent_record_offset":46548,"ancestor_record_offset":46248,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":476,"record_offset":46788,"parent_record_offset":46756,"ancestor_record_offset":46248,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":476,"record_offset":46816,"parent_record_offset":46756,"ancestor_record_offset":46248,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":476,"record_offset":47424,"parent_record_offset":47400,"ancestor_record_offset":46248,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":476,"record_offset":46640,"parent_record_offset":46616,"ancestor_record_offset":46248,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":476,"record_offset":46664,"parent_record_offset":46640,"ancestor_record_offset":46248,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] country_core::input::input_stream::InputStream::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/input/input_stream.rs
//     evidence={"module":476,"record_offset":46968,"parent_record_offset":46944,"ancestor_record_offset":46248,"inlinee":26144,"source_line":108,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] country_core::input::input_stream::InputStream::sync_point
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/input/input_stream.rs
//     evidence={"module":476,"record_offset":49388,"parent_record_offset":49344,"ancestor_record_offset":46248,"inlinee":26154,"source_line":123,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::contains_key
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/map.rs
//     evidence={"module":476,"record_offset":46548,"parent_record_offset":46524,"ancestor_record_offset":46248,"inlinee":4474,"source_line":1414,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::contains
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/set.rs
//     evidence={"module":476,"record_offset":46524,"parent_record_offset":46500,"ancestor_record_offset":46248,"inlinee":4472,"source_line":862,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":476,"record_offset":46844,"parent_record_offset":46756,"ancestor_record_offset":46248,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":476,"record_offset":46756,"parent_record_offset":46732,"ancestor_record_offset":46248,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":476,"record_offset":46868,"parent_record_offset":46844,"ancestor_record_offset":46248,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::OsStr::to_os_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":476,"record_offset":47312,"parent_record_offset":47288,"ancestor_record_offset":46248,"inlinee":5171,"source_line":926,"archive":"_pdb/B/modules/00476.jsonl.gz"}
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
// 首条位置: {"module":476,"record_offset":50788,"rva":8637904,"va":5377347024}
// 编译实例 RVAs: 0x83cdd0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$egui..widgets..button..Checkbox$u20$as$u20$egui..widgets..Widget$GT$::ui
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t4242027","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377347024,"raw_hex":"e86bba4000","size":5,"soft_fail":false,"target_va":5381589296,"va":5377347264,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$egui..widgets..label..Label$u20$as$u20$egui..widgets..Widget$GT$::ui
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t4111593","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377347024,"raw_hex":"e8e9bc3e00","size":5,"soft_fail":false,"target_va":5381459264,"va":5377347666,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=6
//     target_aliases=1 candidates; complete list: input_replay_for_trailers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2265617","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377347024,"raw_hex":"e8ef6dddff","size":5,"soft_fail":false,"target_va":5375081488,"va":5377347100,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: input_replay_for_trailers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2266041","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377347024,"raw_hex":"e8476cddff","size":5,"soft_fail":false,"target_va":5375081504,"va":5377347540,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140addc60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: input_replay_for_trailers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2754899","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377347024,"raw_hex":"e853092a00","size":5,"soft_fail":false,"target_va":5380103264,"va":5377348360,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t34192506","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377347024,"raw_hex":"e87abc0902","size":5,"soft_fail":false,"target_va":5411540992,"va":5377348481,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t5492810","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377347024,"raw_hex":"e84ad05300","size":5,"soft_fail":false,"target_va":5382840208,"va":5377347393,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t34192536","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377347024,"raw_hex":"e898bc0902","size":5,"soft_fail":false,"target_va":5411540963,"va":5377348422,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t4211352","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377347024,"raw_hex":"e898424000","size":5,"soft_fail":false,"target_va":5381558640,"va":5377347283,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":476,"record_offset":52032,"parent_record_offset":50788,"ancestor_record_offset":50788,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_direct] alloc::string::impl$47::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":476,"record_offset":52116,"parent_record_offset":50788,"ancestor_record_offset":50788,"inlinee":5924,"source_line":2956,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":476,"record_offset":50984,"parent_record_offset":50788,"ancestor_record_offset":50788,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":476,"record_offset":51752,"parent_record_offset":50788,"ancestor_record_offset":50788,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_direct] country_core::input::input_stream::InputStream::frame_count
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/input/input_stream.rs
//     evidence={"module":476,"record_offset":52004,"parent_record_offset":50788,"ancestor_record_offset":50788,"inlinee":26155,"source_line":127,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_direct] egui::ui::Ui::checkbox
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/ui.rs
//     evidence={"module":476,"record_offset":51040,"parent_record_offset":50788,"ancestor_record_offset":50788,"inlinee":7601,"source_line":1468,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_direct] egui::ui::Ui::horizontal
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/ui.rs
//     evidence={"module":476,"record_offset":56196,"parent_record_offset":50788,"ancestor_record_offset":50788,"inlinee":7637,"source_line":1908,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_direct] egui::ui::Ui::label
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/ui.rs
//     evidence={"module":476,"record_offset":52948,"parent_record_offset":50788,"ancestor_record_offset":50788,"inlinee":7571,"source_line":1298,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":476,"record_offset":51568,"parent_record_offset":51544,"ancestor_record_offset":50788,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":476,"record_offset":51592,"parent_record_offset":51568,"ancestor_record_offset":50788,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":476,"record_offset":52896,"parent_record_offset":52872,"ancestor_record_offset":50788,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":476,"record_offset":56252,"parent_record_offset":56224,"ancestor_record_offset":50788,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":476,"record_offset":51544,"parent_record_offset":51516,"ancestor_record_offset":50788,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":476,"record_offset":52872,"parent_record_offset":52816,"ancestor_record_offset":50788,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":476,"record_offset":56224,"parent_record_offset":56196,"ancestor_record_offset":50788,"inlinee":4280,"source_line":273,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":476,"record_offset":52080,"parent_record_offset":52056,"ancestor_record_offset":50788,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":476,"record_offset":51456,"parent_record_offset":51428,"ancestor_record_offset":50788,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":476,"record_offset":52844,"parent_record_offset":52816,"ancestor_record_offset":50788,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":476,"record_offset":52816,"parent_record_offset":52792,"ancestor_record_offset":50788,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":476,"record_offset":51516,"parent_record_offset":51484,"ancestor_record_offset":50788,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":476,"record_offset":51484,"parent_record_offset":51456,"ancestor_record_offset":50788,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":476,"record_offset":52792,"parent_record_offset":52768,"ancestor_record_offset":50788,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":476,"record_offset":51392,"parent_record_offset":51364,"ancestor_record_offset":50788,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":476,"record_offset":51364,"parent_record_offset":51336,"ancestor_record_offset":50788,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":476,"record_offset":51308,"parent_record_offset":51280,"ancestor_record_offset":50788,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":476,"record_offset":51336,"parent_record_offset":51308,"ancestor_record_offset":50788,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":476,"record_offset":51280,"parent_record_offset":51252,"ancestor_record_offset":50788,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":476,"record_offset":51252,"parent_record_offset":51224,"ancestor_record_offset":50788,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$47::from
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":476,"record_offset":51224,"parent_record_offset":51196,"ancestor_record_offset":50788,"inlinee":5924,"source_line":2956,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":476,"record_offset":51868,"parent_record_offset":51832,"ancestor_record_offset":50788,"inlinee":4776,"source_line":1888,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$37::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":476,"record_offset":51832,"parent_record_offset":51804,"ancestor_record_offset":50788,"inlinee":4774,"source_line":2550,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":476,"record_offset":51428,"parent_record_offset":51392,"ancestor_record_offset":50788,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":476,"record_offset":51008,"parent_record_offset":50984,"ancestor_record_offset":50788,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":476,"record_offset":51104,"parent_record_offset":51068,"ancestor_record_offset":50788,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":476,"record_offset":51668,"parent_record_offset":51644,"ancestor_record_offset":50788,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":476,"record_offset":52056,"parent_record_offset":52032,"ancestor_record_offset":50788,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":476,"record_offset":51644,"parent_record_offset":51392,"ancestor_record_offset":50788,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":476,"record_offset":51776,"parent_record_offset":51752,"ancestor_record_offset":50788,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":476,"record_offset":51900,"parent_record_offset":51868,"ancestor_record_offset":50788,"inlinee":4777,"source_line":425,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":476,"record_offset":56360,"parent_record_offset":56332,"ancestor_record_offset":50788,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":476,"record_offset":51928,"parent_record_offset":51868,"ancestor_record_offset":50788,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":476,"record_offset":51960,"parent_record_offset":51832,"ancestor_record_offset":50788,"inlinee":4779,"source_line":4075,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] egui::widget_text::RichText::new
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/widget_text.rs
//     evidence={"module":476,"record_offset":51160,"parent_record_offset":51132,"ancestor_record_offset":50788,"inlinee":6308,"source_line":79,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] egui::widget_text::impl$10::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/widget_text.rs
//     evidence={"module":476,"record_offset":53024,"parent_record_offset":53000,"ancestor_record_offset":50788,"inlinee":6407,"source_line":707,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] egui::widget_text::impl$8::from
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/widget_text.rs
//     evidence={"module":476,"record_offset":51132,"parent_record_offset":51104,"ancestor_record_offset":50788,"inlinee":6306,"source_line":693,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] egui::widgets::button::Checkbox::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/widgets/button.rs
//     evidence={"module":476,"record_offset":51068,"parent_record_offset":51040,"ancestor_record_offset":50788,"inlinee":7603,"source_line":363,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] egui::widgets::label::Label::new
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/widgets/label.rs
//     evidence={"module":476,"record_offset":52972,"parent_record_offset":52948,"ancestor_record_offset":50788,"inlinee":7573,"source_line":32,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::backend::input::input_replay_for_trailers::do_input_replay_ui::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 94
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6608
// 首条位置: {"module":476,"inlinee":26120}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":476,"record_offset":25452,"parent_record_offset":19300,"ancestor_record_offset":19300,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":476,"record_offset":19656,"parent_record_offset":19300,"ancestor_record_offset":19300,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::is_ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":476,"record_offset":24508,"parent_record_offset":19300,"ancestor_record_offset":19300,"inlinee":6884,"source_line":564,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":476,"record_offset":22924,"parent_record_offset":19300,"ancestor_record_offset":19300,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_direct] egui::response::Response::clicked
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/response.rs
//     evidence={"module":476,"record_offset":19568,"parent_record_offset":19300,"ancestor_record_offset":19300,"inlinee":7597,"source_line":137,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_direct] egui::ui::Ui::button
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/ui.rs
//     evidence={"module":476,"record_offset":19540,"parent_record_offset":19300,"ancestor_record_offset":19300,"inlinee":7595,"source_line":1450,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":476,"record_offset":25484,"parent_record_offset":19300,"ancestor_record_offset":19300,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":476,"record_offset":25368,"parent_record_offset":19300,"ancestor_record_offset":19300,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_direct] rfd::file_dialog::FileDialog::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"rfd","version":"0.10.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rfd-0.10.0/src/file_dialog.rs
//     evidence={"module":476,"record_offset":22672,"parent_record_offset":19300,"ancestor_record_offset":19300,"inlinee":9319,"source_line":33,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_direct] serde_json::ser::to_string_pretty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":476,"record_offset":22968,"parent_record_offset":19300,"ancestor_record_offset":19300,"inlinee":9300,"source_line":2236,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_direct] std::fs::create_dir_all
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":476,"record_offset":20956,"parent_record_offset":19300,"ancestor_record_offset":19300,"inlinee":8207,"source_line":2766,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_direct] std::fs::read_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":476,"record_offset":27004,"parent_record_offset":19300,"ancestor_record_offset":19300,"inlinee":9324,"source_line":310,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_direct] std::fs::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":476,"record_offset":23820,"parent_record_offset":19300,"ancestor_record_offset":19300,"inlinee":9318,"source_line":347,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::canonicalize
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":476,"record_offset":22868,"parent_record_offset":19300,"ancestor_record_offset":19300,"inlinee":9322,"source_line":2951,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$30::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":476,"record_offset":19952,"parent_record_offset":19300,"ancestor_record_offset":19300,"inlinee":8124,"source_line":1738,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$37::from
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":476,"record_offset":19896,"parent_record_offset":19300,"ancestor_record_offset":19300,"inlinee":5169,"source_line":1825,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$45::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":476,"record_offset":22728,"parent_record_offset":19300,"ancestor_record_offset":19300,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":476,"record_offset":20540,"parent_record_offset":20516,"ancestor_record_offset":19300,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":476,"record_offset":20564,"parent_record_offset":20540,"ancestor_record_offset":19300,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=32
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":476,"record_offset":21464,"parent_record_offset":21440,"ancestor_record_offset":19300,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":476,"record_offset":20516,"parent_record_offset":20340,"ancestor_record_offset":19300,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=32
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":476,"record_offset":21440,"parent_record_offset":21384,"ancestor_record_offset":19300,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":476,"record_offset":22180,"parent_record_offset":22152,"ancestor_record_offset":19300,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":476,"record_offset":20280,"parent_record_offset":20252,"ancestor_record_offset":19300,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":476,"record_offset":21412,"parent_record_offset":21384,"ancestor_record_offset":19300,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":476,"record_offset":21384,"parent_record_offset":21360,"ancestor_record_offset":19300,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":476,"record_offset":20340,"parent_record_offset":20308,"ancestor_record_offset":19300,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":476,"record_offset":20308,"parent_record_offset":20280,"ancestor_record_offset":19300,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":476,"record_offset":21360,"parent_record_offset":21336,"ancestor_record_offset":19300,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":476,"record_offset":20376,"parent_record_offset":20340,"ancestor_record_offset":19300,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":476,"record_offset":20212,"parent_record_offset":20180,"ancestor_record_offset":19300,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":476,"record_offset":20180,"parent_record_offset":20112,"ancestor_record_offset":19300,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":476,"record_offset":19760,"parent_record_offset":19724,"ancestor_record_offset":19300,"inlinee":4776,"source_line":1888,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$37::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":476,"record_offset":19724,"parent_record_offset":19700,"ancestor_record_offset":19300,"inlinee":4774,"source_line":2550,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":476,"record_offset":20152,"parent_record_offset":20112,"ancestor_record_offset":19300,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":476,"record_offset":23076,"parent_record_offset":23016,"ancestor_record_offset":19300,"inlinee":4632,"source_line":482,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":476,"record_offset":20252,"parent_record_offset":20212,"ancestor_record_offset":19300,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$11::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":476,"record_offset":20080,"parent_record_offset":20048,"ancestor_record_offset":19300,"inlinee":4623,"source_line":3286,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":476,"record_offset":20112,"parent_record_offset":20080,"ancestor_record_offset":19300,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":476,"record_offset":22212,"parent_record_offset":22180,"ancestor_record_offset":19300,"inlinee":5033,"source_line":219,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":476,"record_offset":20400,"parent_record_offset":20376,"ancestor_record_offset":19300,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":476,"record_offset":20424,"parent_record_offset":20400,"ancestor_record_offset":19300,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":476,"record_offset":23852,"parent_record_offset":23820,"ancestor_record_offset":19300,"inlinee":5126,"source_line":703,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":476,"record_offset":20640,"parent_record_offset":20616,"ancestor_record_offset":19300,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":476,"record_offset":20472,"parent_record_offset":20448,"ancestor_record_offset":19300,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":476,"record_offset":22236,"parent_record_offset":22212,"ancestor_record_offset":19300,"inlinee":5034,"source_line":393,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":476,"record_offset":20448,"parent_record_offset":20424,"ancestor_record_offset":19300,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":476,"record_offset":25540,"parent_record_offset":25508,"ancestor_record_offset":19300,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":476,"record_offset":25568,"parent_record_offset":25508,"ancestor_record_offset":19300,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":476,"record_offset":20616,"parent_record_offset":20212,"ancestor_record_offset":19300,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=109
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":476,"record_offset":19676,"parent_record_offset":19656,"ancestor_record_offset":19300,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_byte_sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":476,"record_offset":21980,"parent_record_offset":21956,"ancestor_record_offset":19300,"inlinee":5966,"source_line":1340,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":476,"record_offset":22028,"parent_record_offset":22004,"ancestor_record_offset":19300,"inlinee":5968,"source_line":528,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":476,"record_offset":22004,"parent_record_offset":21980,"ancestor_record_offset":19300,"inlinee":5967,"source_line":1320,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":476,"record_offset":19792,"parent_record_offset":19760,"ancestor_record_offset":19300,"inlinee":4777,"source_line":425,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":476,"record_offset":25392,"parent_record_offset":25368,"ancestor_record_offset":19300,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":476,"record_offset":25416,"parent_record_offset":25392,"ancestor_record_offset":19300,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":476,"record_offset":19820,"parent_record_offset":19760,"ancestor_record_offset":19300,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":476,"record_offset":19852,"parent_record_offset":19724,"ancestor_record_offset":19300,"inlinee":4779,"source_line":4075,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] egui::context::Context::input
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/context.rs
//     evidence={"module":476,"record_offset":19620,"parent_record_offset":19596,"ancestor_record_offset":19300,"inlinee":6385,"source_line":795,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] egui::response::Response::clicked_by
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/response.rs
//     evidence={"module":476,"record_offset":19596,"parent_record_offset":19568,"ancestor_record_offset":19300,"inlinee":7598,"source_line":150,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":476,"record_offset":25596,"parent_record_offset":25508,"ancestor_record_offset":19300,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":476,"record_offset":25508,"parent_record_offset":25484,"ancestor_record_offset":19300,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":476,"record_offset":25620,"parent_record_offset":25596,"ancestor_record_offset":19300,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] rfd::file_dialog::impl$7::default
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"rfd","version":"0.10.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rfd-0.10.0/src/file_dialog.rs
//     evidence={"module":476,"record_offset":22696,"parent_record_offset":22672,"ancestor_record_offset":19300,"inlinee":9321,"source_line":18,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Serializer::pretty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":476,"record_offset":23352,"parent_record_offset":23324,"ancestor_record_offset":19300,"inlinee":9303,"source_line":36,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Serializer::with_formatter
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":476,"record_offset":23376,"parent_record_offset":23352,"ancestor_record_offset":19300,"inlinee":8161,"source_line":49,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::to_vec_pretty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":476,"record_offset":23016,"parent_record_offset":22968,"ancestor_record_offset":19300,"inlinee":9301,"source_line":2201,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::to_writer_pretty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":476,"record_offset":23324,"parent_record_offset":23016,"ancestor_record_offset":19300,"inlinee":9302,"source_line":2168,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::OsStr::to_os_string
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":476,"record_offset":19920,"parent_record_offset":19896,"ancestor_record_offset":19300,"inlinee":5171,"source_line":926,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$10::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":476,"record_offset":19984,"parent_record_offset":19952,"ancestor_record_offset":19300,"inlinee":8126,"source_line":642,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$59::as_ref
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":476,"record_offset":21088,"parent_record_offset":21064,"ancestor_record_offset":19300,"inlinee":5134,"source_line":1639,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":476,"record_offset":21136,"parent_record_offset":21112,"ancestor_record_offset":19300,"inlinee":5138,"source_line":599,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":476,"record_offset":21112,"parent_record_offset":21088,"ancestor_record_offset":19300,"inlinee":5136,"source_line":617,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::DirBuilder::create
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":476,"record_offset":21184,"parent_record_offset":20956,"ancestor_record_offset":19300,"inlinee":8210,"source_line":3032,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::DirBuilder::recursive
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":476,"record_offset":20988,"parent_record_offset":20956,"ancestor_record_offset":19300,"inlinee":8209,"source_line":3008,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::canonicalize
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":476,"record_offset":22892,"parent_record_offset":22868,"ancestor_record_offset":19300,"inlinee":9323,"source_line":2678,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::decode_repr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":476,"record_offset":21956,"parent_record_offset":21928,"ancestor_record_offset":19300,"inlinee":5965,"source_line":253,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::impl$3::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":476,"record_offset":21928,"parent_record_offset":21900,"ancestor_record_offset":19300,"inlinee":5963,"source_line":239,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":476,"record_offset":21064,"parent_record_offset":21040,"ancestor_record_offset":19300,"inlinee":5132,"source_line":2182,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$45::deref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":476,"record_offset":21040,"parent_record_offset":21016,"ancestor_record_offset":19300,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$83::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":476,"record_offset":21016,"parent_record_offset":20956,"ancestor_record_offset":19300,"inlinee":5128,"source_line":3362,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::impl$10::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":476,"record_offset":20016,"parent_record_offset":19984,"ancestor_record_offset":19300,"inlinee":8128,"source_line":13,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] std::sys_common::wtf8::impl$35::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys_common\wtf8.rs
//     evidence={"module":476,"record_offset":20048,"parent_record_offset":20016,"ancestor_record_offset":19300,"inlinee":8130,"source_line":132,"archive":"_pdb/B/modules/00476.jsonl.gz"}
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
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 9; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::startup::init_photomode_state::PhotomodeState>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::startup::init_photomode_state::PhotomodeState>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::startup::init_photomode_state::PhotomodeState>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TodSequence>,bevy_ecs::change_detection::ResMut<country_core::resources::weather::Weather>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TodSequence>", "bevy_ecs::change_detection::ResMut<country_core::resources::weather::Weather>", "bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>", "bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TodSequence>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::weather::Weather>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>", "bevy_ecs::system::commands::Commands", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>", "bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::placement_data_library::AssetPlacementDataLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::compute_passes::leaf_particles::LoadedParticleEmitterCount>,bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::InteractableForestPlacements>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>", "bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>", "bevy_ecs::system::commands::Commands", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::placement_data_library::AssetPlacementDataLibrary>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::compute_passes::leaf_particles::LoadedParticleEmitterCount>", "bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::InteractableForestPlacements>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::placement_data_library::AssetPlacementDataLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::compute_passes::leaf_particles::LoadedParticleEmitterCount>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::InteractableForestPlacements>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::placement_data_library::AssetPlacementDataLibrary>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>", "bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>", "bevy_ecs::system::commands::Commands", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::placement_data_library::AssetPlacementDataLibrary>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::placement_data_library::AssetPlacementDataLibrary>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::color_library::ColorLibrarySsbo>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>", "bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::color_library::ColorLibrarySsbo>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::color_library::ColorLibrarySsbo>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::systems::backend::input::input_replay_for_trailers::do_input_replay_ui
// 信息 ID: aaf286f0a6b5ea951566e335
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::ops::function::FnOnce::call_once<tiny_glade::gui::gui_debug::do_gui_debug::closure$0::closure$0::closure$12::closure_env$0,tuple$<ref_mut$<egui::ui::Ui> > > @ 0xc025b -> 0x83cdd0 (direct_jump_outside_procedure); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::backend::input::input_replay_for_trailers::do_input_replay_ui::closure$0
// 信息 ID: 57d9056acc2582c955a2c419
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::backend::input::input_replay_for_trailers::do_input_replay_ui", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::backend::input::input_replay_for_trailers::do_input_replay_ui::closure_env$0 <- core::ops::function::FnOnce::call_once<country_core::systems::backend::input::input_replay_for_trailers::do_input_replay_ui::closure_env$0,tuple$<ref_mut$<egui::ui::Ui> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<country_core::systems::backend::input::input_replay_for_trailers::do_input_replay_ui::closure_env$0,tuple$<ref_mut$<egui::ui::Ui> > >; module=476, inline_record=19300; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::backend::input::input_replay_for_trailers::record_and_replay_input_for_trailers
// 信息 ID: e93a64d8e3078308f1128dda
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 55cdcc6aa7c329956db2e394: void (*)(bevy_ecs::change_detection::ResMut<country_core::input::input_stream::InputRecordReplayState>,bevy_ecs::change_detection::Res<bevy_input::button_input::ButtonInput<enum2$<bevy_input::keyboard::KeyCode> > >,bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::Toolbar>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::input::input_stream::InputRecordReplayState>,bevy_ecs::change_detection::Res<bevy_input::button_input::ButtonInput<enum2$<bevy_input::keyboard::KeyCode> > >,bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::Toolbar>),void (*)(bevy_ecs::change_detection::ResMut<country_core::input::input_stream::InputRecordReplayState>,bevy_ecs::change_detection::Res<bevy_input::button_input::ButtonInput<enum2$<bevy_input::keyboard::KeyCode> > >,bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::Toolbar>)> > > @ 0x14dbd7d -> 0x83c620 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::input::input_stream::InputRecordReplayState>,bevy_ecs::change_detection::Res<bevy_input::button_input::ButtonInput<enum2$<bevy_input::keyboard::KeyCode> > >,bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::Toolbar>),void (*)(bevy_ecs::change_detection::ResMut<country_core::input::input_stream::InputRecordReplayState>,bevy_ecs::change_detection::Res<bevy_input::button_input::ButtonInput<enum2$<bevy_input::keyboard::KeyCode> > >,bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::Toolbar>)> > @ 0x168a14b -> 0x83c620 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===

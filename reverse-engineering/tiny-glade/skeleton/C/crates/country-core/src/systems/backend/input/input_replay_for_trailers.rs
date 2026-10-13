// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/country-core/src/systems/backend/input/input_replay_for_trailers.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 7e31a9f4b1a5fa97dc3117d07d2f91f3d310d5a29af98f2b99fb47ef6d399255
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: input_replay_for_trailers.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 590
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
// use bevy_input; // namespace candidate
//   package identities: [{"name":"bevy_input","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
// use bevy_platform; // namespace candidate
//   package identities: [{"name":"bevy_platform","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use egui; // namespace candidate
//   package identities: [{"name":"egui","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.27.2"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.1"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.27"}]
// use rfd; // namespace candidate
//   package identities: [{"name":"rfd","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.10.0"}]
// use serde_json; // namespace candidate
//   package identities: [{"name":"serde_json","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c","version":"1.0.151"}]
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
// 首条位置: {"module":590,"record_offset":65184,"rva":12644208,"va":5381353328}
// 编译实例 RVAs: 0xc0ef70
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <core::wtf8::Wtf8>::to_owned
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3439198","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381353328,"raw_hex":"e85e7a3400","size":5,"soft_fail":false,"target_va":5384793264,"va":5381354061,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t3255743","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381353328,"raw_hex":"e8bfad3100","size":5,"soft_fail":false,"target_va":5384609408,"va":5381353660,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=7
//     evidence={"assembly":"\tcallq\t-5675269","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381353328,"raw_hex":"e8fb66a9ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5381353968,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140084d10
//     owner=unresolved_namespace; count=2
//     target_aliases=4 candidates; complete list: input_replay_for_trailers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12100800","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381353328,"raw_hex":"e8405b47ff","size":5,"soft_fail":false,"target_va":5369253136,"va":5381353931,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140087590
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: input_replay_for_trailers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12090415","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381353328,"raw_hex":"e8d18347ff","size":5,"soft_fail":false,"target_va":5369263504,"va":5381353914,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400ace80
//     owner=unresolved_namespace; count=1
//     target_aliases=14 candidates; complete list: input_replay_for_trailers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11937861","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381353328,"raw_hex":"e8bbd749ff","size":5,"soft_fail":false,"target_va":5369417344,"va":5381355200,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1415f8ac0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: input_replay_for_trailers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10394073","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381353328,"raw_hex":"e8d9999e00","size":5,"soft_fail":false,"target_va":5391747776,"va":5381353698,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1416076f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: input_replay_for_trailers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10454369","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381353328,"raw_hex":"e861859f00","size":5,"soft_fail":false,"target_va":5391808240,"va":5381353866,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1417a68f0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: input_replay_for_trailers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12155164","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381353328,"raw_hex":"e81c79b900","size":5,"soft_fail":false,"target_va":5393508592,"va":5381353423,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t35266578","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381353328,"raw_hex":"e812201a02","size":5,"soft_fail":false,"target_va":5416621536,"va":5381354953,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t35267240","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381353328,"raw_hex":"e8a8221a02","size":5,"soft_fail":false,"target_va":5416622240,"va":5381354995,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::camera_rig::MainCamera::immediately_finish_interpolating
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-2034737","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381353328,"raw_hex":"e8cff3e0ff","size":5,"soft_fail":false,"target_va":5379320176,"va":5381354908,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::camera_rig::MainCamera::load_parameters
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-2037398","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381353328,"raw_hex":"e86ae9e0ff","size":5,"soft_fail":false,"target_va":5379317504,"va":5381354897,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::camera_rig::MainCamera::store_parameters
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-2035163","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381353328,"raw_hex":"e825f2e0ff","size":5,"soft_fail":false,"target_va":5379318560,"va":5381353718,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":590,"record_offset":66100,"parent_record_offset":65184,"ancestor_record_offset":65184,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_direct] bevy_input::button_input::ButtonInput::just_pressed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_input","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_input-0.17.3/src/button_input.rs
//     evidence={"module":590,"record_offset":65412,"parent_record_offset":65184,"ancestor_record_offset":65184,"inlinee":34554,"source_line":188,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":590,"record_offset":65580,"parent_record_offset":65184,"ancestor_record_offset":65184,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":590,"record_offset":66156,"parent_record_offset":65184,"ancestor_record_offset":65184,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_direct] country_core::input::input_stream::InputRecordReplayState::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/input/input_stream.rs
//     evidence={"module":590,"record_offset":65784,"parent_record_offset":65184,"ancestor_record_offset":65184,"inlinee":29504,"source_line":224,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_direct] country_core::input::input_stream::InputRecordReplayState::seek_to_sync_point
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/input/input_stream.rs
//     evidence={"module":590,"record_offset":69104,"parent_record_offset":65184,"ancestor_record_offset":65184,"inlinee":33052,"source_line":242,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_direct] country_core::input::input_stream::InputStream::sync_point_count
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/input/input_stream.rs
//     evidence={"module":590,"record_offset":69020,"parent_record_offset":65184,"ancestor_record_offset":65184,"inlinee":33047,"source_line":119,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::toolbar::impl$19::clone
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/toolbar.rs
//     evidence={"module":590,"record_offset":65748,"parent_record_offset":65184,"ancestor_record_offset":65184,"inlinee":37683,"source_line":106,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":590,"record_offset":65636,"parent_record_offset":65184,"ancestor_record_offset":65184,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":590,"record_offset":65496,"parent_record_offset":65184,"ancestor_record_offset":65184,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$37::from
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":590,"record_offset":66988,"parent_record_offset":65184,"ancestor_record_offset":65184,"inlinee":7154,"source_line":1959,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":590,"record_offset":66492,"parent_record_offset":66468,"ancestor_record_offset":65184,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":590,"record_offset":66516,"parent_record_offset":66492,"ancestor_record_offset":65184,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":590,"record_offset":66540,"parent_record_offset":66516,"ancestor_record_offset":65184,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":590,"record_offset":66468,"parent_record_offset":66412,"ancestor_record_offset":65184,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":590,"record_offset":69212,"parent_record_offset":69180,"ancestor_record_offset":65184,"inlinee":12113,"source_line":907,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::to_physical_idx
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":590,"record_offset":69256,"parent_record_offset":69212,"ancestor_record_offset":65184,"inlinee":4784,"source_line":248,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::wrap_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":590,"record_offset":69280,"parent_record_offset":69256,"ancestor_record_offset":65184,"inlinee":4785,"source_line":243,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::impl$14::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":590,"record_offset":69180,"parent_record_offset":69148,"ancestor_record_offset":65184,"inlinee":13895,"source_line":3626,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::impl$16::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":590,"record_offset":65856,"parent_record_offset":65832,"ancestor_record_offset":65184,"inlinee":12130,"source_line":3641,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::impl$23::from
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":590,"record_offset":65964,"parent_record_offset":65932,"ancestor_record_offset":65184,"inlinee":12134,"source_line":3746,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::spec_from_iter::impl$0::spec_from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\spec_from_iter.rs
//     evidence={"module":590,"record_offset":65908,"parent_record_offset":65856,"ancestor_record_offset":65184,"inlinee":12132,"source_line":12,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":590,"record_offset":66440,"parent_record_offset":66412,"ancestor_record_offset":65184,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":590,"record_offset":66412,"parent_record_offset":66388,"ancestor_record_offset":65184,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":590,"record_offset":66388,"parent_record_offset":66364,"ancestor_record_offset":65184,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::into_raw_parts_with_alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":590,"record_offset":65988,"parent_record_offset":65964,"ancestor_record_offset":65184,"inlinee":12135,"source_line":1339,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":590,"record_offset":65932,"parent_record_offset":65908,"ancestor_record_offset":65184,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":590,"record_offset":69624,"parent_record_offset":69600,"ancestor_record_offset":65184,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":590,"record_offset":66124,"parent_record_offset":66100,"ancestor_record_offset":65184,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] bevy_platform::collections::hash_set::HashSet::contains
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_platform","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_platform-0.17.3/src/collections/hash_set.rs
//     evidence={"module":590,"record_offset":65436,"parent_record_offset":65412,"ancestor_record_offset":65184,"inlinee":34555,"source_line":735,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":590,"record_offset":65880,"parent_record_offset":65856,"ancestor_record_offset":65184,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":590,"record_offset":65832,"parent_record_offset":65808,"ancestor_record_offset":65184,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\manually_drop.rs
//     evidence={"module":590,"record_offset":66012,"parent_record_offset":65988,"ancestor_record_offset":65184,"inlinee":5072,"source_line":184,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_dangling::MaybeDangling::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_dangling.rs
//     evidence={"module":590,"record_offset":66036,"parent_record_offset":66012,"ancestor_record_offset":65184,"inlinee":5074,"source_line":80,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":590,"record_offset":69852,"parent_record_offset":69816,"ancestor_record_offset":65184,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":590,"record_offset":69304,"parent_record_offset":69280,"ancestor_record_offset":65184,"inlinee":4834,"source_line":2456,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":590,"record_offset":69388,"parent_record_offset":69104,"ancestor_record_offset":65184,"inlinee":4931,"source_line":2493,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":590,"record_offset":65604,"parent_record_offset":65580,"ancestor_record_offset":65184,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=39
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":590,"record_offset":66188,"parent_record_offset":66156,"ancestor_record_offset":65184,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":590,"record_offset":69348,"parent_record_offset":69212,"ancestor_record_offset":65184,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":590,"record_offset":65520,"parent_record_offset":65496,"ancestor_record_offset":65184,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":590,"record_offset":65544,"parent_record_offset":65520,"ancestor_record_offset":65184,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] country_core::input::input_stream::InputStream::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/input/input_stream.rs
//     evidence={"module":590,"record_offset":65808,"parent_record_offset":65784,"ancestor_record_offset":65184,"inlinee":29502,"source_line":108,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] country_core::input::input_stream::InputStream::sync_point
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/input/input_stream.rs
//     evidence={"module":590,"record_offset":69148,"parent_record_offset":69104,"ancestor_record_offset":65184,"inlinee":32967,"source_line":123,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::contains
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/set.rs
//     evidence={"module":590,"record_offset":65460,"parent_record_offset":65436,"ancestor_record_offset":65184,"inlinee":5589,"source_line":862,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":590,"record_offset":65684,"parent_record_offset":65660,"ancestor_record_offset":65184,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":590,"record_offset":65660,"parent_record_offset":65636,"ancestor_record_offset":65184,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":590,"record_offset":65708,"parent_record_offset":65684,"ancestor_record_offset":65184,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::OsStr::to_os_string
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":590,"record_offset":67012,"parent_record_offset":66988,"ancestor_record_offset":65184,"inlinee":5109,"source_line":986,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::Slice::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":590,"record_offset":67036,"parent_record_offset":67012,"ancestor_record_offset":65184,"inlinee":5111,"source_line":265,"archive":"_pdb/C/modules/00590.jsonl.gz"}
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
// 首条位置: {"module":590,"record_offset":61488,"rva":12643328,"va":5381352448}
// 编译实例 RVAs: 0xc0ec00
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$egui..widgets..button..Checkbox$u20$as$u20$egui..widgets..Widget$GT$::ui
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t1019368","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381352448,"raw_hex":"e8e88d0f00","size":5,"soft_fail":false,"target_va":5382371936,"va":5381352563,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$egui..widgets..label..Label$u20$as$u20$egui..widgets..Widget$GT$::ui
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t1459227","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381352448,"raw_hex":"e81b441600","size":5,"soft_fail":false,"target_va":5382812128,"va":5381352896,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-5674001","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381352448,"raw_hex":"e8ef6ba9ff","size":5,"soft_fail":false,"target_va":5375678688,"va":5381352684,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-5674086","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381352448,"raw_hex":"e89a6ba9ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5381352785,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=2
//     target_aliases=1699 candidates; complete list: input_replay_for_trailers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12630482","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381352448,"raw_hex":"e82e463fff","size":5,"soft_fail":false,"target_va":5368722192,"va":5381352669,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14054ca00
//     owner=unresolved_namespace; count=2
//     target_aliases=4 candidates; complete list: input_replay_for_trailers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7086683","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381352448,"raw_hex":"e8a5dd93ff","size":5,"soft_fail":false,"target_va":5374265856,"va":5381352534,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409c3e10
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: input_replay_for_trailers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2404317","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381352448,"raw_hex":"e82350dbff","size":5,"soft_fail":false,"target_va":5378948624,"va":5381352936,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b0bfd0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: input_replay_for_trailers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1060616","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381352448,"raw_hex":"e8f8d0efff","size":5,"soft_fail":false,"target_va":5380292560,"va":5381353171,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t35267538","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381352448,"raw_hex":"e8d2231a02","size":5,"soft_fail":false,"target_va":5416620769,"va":5381353226,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t3442197","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381352448,"raw_hex":"e815863400","size":5,"soft_fail":false,"target_va":5384794864,"va":5381352662,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t35267548","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381352448,"raw_hex":"e8dc231a02","size":5,"soft_fail":false,"target_va":5416620794,"va":5381353241,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t1419317","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381352448,"raw_hex":"e835a81500","size":5,"soft_fail":false,"target_va":5382771904,"va":5381352582,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":590,"record_offset":62072,"parent_record_offset":61488,"ancestor_record_offset":61488,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_direct] alloc::string::impl$48::from
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":590,"record_offset":62156,"parent_record_offset":61488,"ancestor_record_offset":61488,"inlinee":4520,"source_line":3113,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":590,"record_offset":61688,"parent_record_offset":61488,"ancestor_record_offset":61488,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hint.rs
//     evidence={"module":590,"record_offset":63040,"parent_record_offset":61488,"ancestor_record_offset":61488,"inlinee":7244,"source_line":613,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":590,"record_offset":61804,"parent_record_offset":61488,"ancestor_record_offset":61488,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_direct] country_core::input::input_stream::InputStream::frame_count
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/input/input_stream.rs
//     evidence={"module":590,"record_offset":62044,"parent_record_offset":61488,"ancestor_record_offset":61488,"inlinee":33045,"source_line":127,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_direct] egui::ui::Ui::checkbox
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/ui.rs
//     evidence={"module":590,"record_offset":61744,"parent_record_offset":61488,"ancestor_record_offset":61488,"inlinee":17893,"source_line":1468,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_direct] egui::ui::Ui::horizontal
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/ui.rs
//     evidence={"module":590,"record_offset":64392,"parent_record_offset":61488,"ancestor_record_offset":61488,"inlinee":7290,"source_line":1908,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_direct] egui::ui::Ui::label
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/ui.rs
//     evidence={"module":590,"record_offset":63068,"parent_record_offset":61488,"ancestor_record_offset":61488,"inlinee":7289,"source_line":1298,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":590,"record_offset":62488,"parent_record_offset":62464,"ancestor_record_offset":61488,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":590,"record_offset":62512,"parent_record_offset":62488,"ancestor_record_offset":61488,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":590,"record_offset":62932,"parent_record_offset":62908,"ancestor_record_offset":61488,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":590,"record_offset":62956,"parent_record_offset":62932,"ancestor_record_offset":61488,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":590,"record_offset":62536,"parent_record_offset":62512,"ancestor_record_offset":61488,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":590,"record_offset":62980,"parent_record_offset":62956,"ancestor_record_offset":61488,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":590,"record_offset":62464,"parent_record_offset":62436,"ancestor_record_offset":61488,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":590,"record_offset":62908,"parent_record_offset":62852,"ancestor_record_offset":61488,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":590,"record_offset":64456,"parent_record_offset":64420,"ancestor_record_offset":61488,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":590,"record_offset":64420,"parent_record_offset":64392,"ancestor_record_offset":61488,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":590,"record_offset":62120,"parent_record_offset":62096,"ancestor_record_offset":61488,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":590,"record_offset":62376,"parent_record_offset":62348,"ancestor_record_offset":61488,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":590,"record_offset":62880,"parent_record_offset":62852,"ancestor_record_offset":61488,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":590,"record_offset":62852,"parent_record_offset":62828,"ancestor_record_offset":61488,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":590,"record_offset":62436,"parent_record_offset":62404,"ancestor_record_offset":61488,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":590,"record_offset":62404,"parent_record_offset":62376,"ancestor_record_offset":61488,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":590,"record_offset":62828,"parent_record_offset":62804,"ancestor_record_offset":61488,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":590,"record_offset":62256,"parent_record_offset":62212,"ancestor_record_offset":61488,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":590,"record_offset":62284,"parent_record_offset":62256,"ancestor_record_offset":61488,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":590,"record_offset":62312,"parent_record_offset":62284,"ancestor_record_offset":61488,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":590,"record_offset":62212,"parent_record_offset":62184,"ancestor_record_offset":61488,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":590,"record_offset":62184,"parent_record_offset":62156,"ancestor_record_offset":61488,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":590,"record_offset":61908,"parent_record_offset":61872,"ancestor_record_offset":61488,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":590,"record_offset":61872,"parent_record_offset":61848,"ancestor_record_offset":61488,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":590,"record_offset":62348,"parent_record_offset":62312,"ancestor_record_offset":61488,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":590,"record_offset":61712,"parent_record_offset":61688,"ancestor_record_offset":61488,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":590,"record_offset":63120,"parent_record_offset":63092,"ancestor_record_offset":61488,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":590,"record_offset":62096,"parent_record_offset":62072,"ancestor_record_offset":61488,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":590,"record_offset":62592,"parent_record_offset":62312,"ancestor_record_offset":61488,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":590,"record_offset":62616,"parent_record_offset":62592,"ancestor_record_offset":61488,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=17
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":590,"record_offset":61824,"parent_record_offset":61804,"ancestor_record_offset":61488,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":590,"record_offset":61940,"parent_record_offset":61908,"ancestor_record_offset":61488,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":590,"record_offset":61968,"parent_record_offset":61908,"ancestor_record_offset":61488,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":590,"record_offset":62000,"parent_record_offset":61872,"ancestor_record_offset":61488,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] egui::widget_text::RichText::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/widget_text.rs
//     evidence={"module":590,"record_offset":63168,"parent_record_offset":63144,"ancestor_record_offset":61488,"inlinee":6021,"source_line":79,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] egui::widget_text::impl$10::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/widget_text.rs
//     evidence={"module":590,"record_offset":63144,"parent_record_offset":63120,"ancestor_record_offset":61488,"inlinee":6019,"source_line":707,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] egui::widgets::button::Checkbox::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/widgets/button.rs
//     evidence={"module":590,"record_offset":61768,"parent_record_offset":61744,"ancestor_record_offset":61488,"inlinee":16470,"source_line":363,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] egui::widgets::label::Label::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/widgets/label.rs
//     evidence={"module":590,"record_offset":63092,"parent_record_offset":63068,"ancestor_record_offset":61488,"inlinee":6015,"source_line":32,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::backend::input::input_replay_for_trailers::do_input_replay_ui::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 94
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x933A
// 首条位置: {"module":590,"inlinee":37690}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":590,"record_offset":92068,"parent_record_offset":87348,"ancestor_record_offset":87348,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":590,"record_offset":87808,"parent_record_offset":87348,"ancestor_record_offset":87348,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::is_ok
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":590,"record_offset":91928,"parent_record_offset":87348,"ancestor_record_offset":87348,"inlinee":5676,"source_line":593,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":590,"record_offset":90260,"parent_record_offset":87348,"ancestor_record_offset":87348,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_direct] egui::response::Response::clicked
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/response.rs
//     evidence={"module":590,"record_offset":87720,"parent_record_offset":87348,"ancestor_record_offset":87348,"inlinee":6107,"source_line":137,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_direct] egui::ui::Ui::button
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/ui.rs
//     evidence={"module":590,"record_offset":87624,"parent_record_offset":87348,"ancestor_record_offset":87348,"inlinee":7333,"source_line":1450,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":590,"record_offset":92124,"parent_record_offset":87348,"ancestor_record_offset":87348,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":590,"record_offset":91984,"parent_record_offset":87348,"ancestor_record_offset":87348,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_direct] rfd::file_dialog::FileDialog::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"rfd","version":"0.10.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rfd-0.10.0/src/file_dialog.rs
//     evidence={"module":590,"record_offset":90148,"parent_record_offset":87348,"ancestor_record_offset":87348,"inlinee":8111,"source_line":33,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_direct] serde_json::ser::to_string_pretty
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":590,"record_offset":90304,"parent_record_offset":87348,"ancestor_record_offset":87348,"inlinee":8087,"source_line":2264,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_direct] std::fs::create_dir_all
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":590,"record_offset":89292,"parent_record_offset":87348,"ancestor_record_offset":87348,"inlinee":7539,"source_line":3123,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_direct] std::fs::read_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":590,"record_offset":93656,"parent_record_offset":87348,"ancestor_record_offset":87348,"inlinee":8116,"source_line":383,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_direct] std::fs::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":590,"record_offset":91296,"parent_record_offset":87348,"ancestor_record_offset":87348,"inlinee":8109,"source_line":420,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::canonicalize
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":590,"record_offset":90204,"parent_record_offset":87348,"ancestor_record_offset":87348,"inlinee":8114,"source_line":3307,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$30::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":590,"record_offset":88144,"parent_record_offset":87348,"ancestor_record_offset":87348,"inlinee":6706,"source_line":1874,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$37::from
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":590,"record_offset":88060,"parent_record_offset":87348,"ancestor_record_offset":87348,"inlinee":7154,"source_line":1959,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":590,"record_offset":88852,"parent_record_offset":88828,"ancestor_record_offset":87348,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":590,"record_offset":88876,"parent_record_offset":88852,"ancestor_record_offset":87348,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":590,"record_offset":89628,"parent_record_offset":89604,"ancestor_record_offset":87348,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":590,"record_offset":89652,"parent_record_offset":89628,"ancestor_record_offset":87348,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":590,"record_offset":88900,"parent_record_offset":88876,"ancestor_record_offset":87348,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":590,"record_offset":89676,"parent_record_offset":89652,"ancestor_record_offset":87348,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":590,"record_offset":88828,"parent_record_offset":88668,"ancestor_record_offset":87348,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":590,"record_offset":89604,"parent_record_offset":89548,"ancestor_record_offset":87348,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":590,"record_offset":94888,"parent_record_offset":94860,"ancestor_record_offset":87348,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":590,"record_offset":88416,"parent_record_offset":88392,"ancestor_record_offset":87348,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":590,"record_offset":88588,"parent_record_offset":88552,"ancestor_record_offset":87348,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=17
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":590,"record_offset":89576,"parent_record_offset":89548,"ancestor_record_offset":87348,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=17
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":590,"record_offset":89548,"parent_record_offset":89524,"ancestor_record_offset":87348,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":590,"record_offset":88464,"parent_record_offset":88440,"ancestor_record_offset":87348,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":590,"record_offset":88440,"parent_record_offset":88416,"ancestor_record_offset":87348,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":590,"record_offset":88668,"parent_record_offset":88624,"ancestor_record_offset":87348,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":590,"record_offset":88624,"parent_record_offset":88588,"ancestor_record_offset":87348,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=17
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":590,"record_offset":89524,"parent_record_offset":89500,"ancestor_record_offset":87348,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":590,"record_offset":88716,"parent_record_offset":88668,"ancestor_record_offset":87348,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":590,"record_offset":88508,"parent_record_offset":88324,"ancestor_record_offset":87348,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":590,"record_offset":87924,"parent_record_offset":87888,"ancestor_record_offset":87348,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":590,"record_offset":87888,"parent_record_offset":87860,"ancestor_record_offset":87348,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":590,"record_offset":88392,"parent_record_offset":88368,"ancestor_record_offset":87348,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":590,"record_offset":88368,"parent_record_offset":88324,"ancestor_record_offset":87348,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":590,"record_offset":90412,"parent_record_offset":90352,"ancestor_record_offset":87348,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":590,"record_offset":88552,"parent_record_offset":88508,"ancestor_record_offset":87348,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":590,"record_offset":88288,"parent_record_offset":88252,"ancestor_record_offset":87348,"inlinee":5763,"source_line":3747,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":590,"record_offset":88324,"parent_record_offset":88288,"ancestor_record_offset":87348,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] alloc::wtf8::impl$14::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\wtf8\mod.rs
//     evidence={"module":590,"record_offset":88252,"parent_record_offset":88216,"ancestor_record_offset":87348,"inlinee":6712,"source_line":38,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":590,"record_offset":88740,"parent_record_offset":88716,"ancestor_record_offset":87348,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":590,"record_offset":91328,"parent_record_offset":91296,"ancestor_record_offset":87348,"inlinee":5107,"source_line":717,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":590,"record_offset":88788,"parent_record_offset":88764,"ancestor_record_offset":87348,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":590,"record_offset":88764,"parent_record_offset":88740,"ancestor_record_offset":87348,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":590,"record_offset":92092,"parent_record_offset":92068,"ancestor_record_offset":87348,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":590,"record_offset":88956,"parent_record_offset":88508,"ancestor_record_offset":87348,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":590,"record_offset":88980,"parent_record_offset":88956,"ancestor_record_offset":87348,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=93
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":590,"record_offset":87684,"parent_record_offset":87652,"ancestor_record_offset":87348,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":590,"record_offset":87956,"parent_record_offset":87924,"ancestor_record_offset":87348,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":590,"record_offset":92008,"parent_record_offset":91984,"ancestor_record_offset":87348,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":590,"record_offset":92032,"parent_record_offset":92008,"ancestor_record_offset":87348,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":590,"record_offset":87984,"parent_record_offset":87924,"ancestor_record_offset":87348,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":590,"record_offset":88016,"parent_record_offset":87888,"ancestor_record_offset":87348,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] egui::context::Context::input
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/context.rs
//     evidence={"module":590,"record_offset":87772,"parent_record_offset":87748,"ancestor_record_offset":87348,"inlinee":6085,"source_line":795,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] egui::response::Response::clicked_by
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/response.rs
//     evidence={"module":590,"record_offset":87748,"parent_record_offset":87720,"ancestor_record_offset":87348,"inlinee":6108,"source_line":150,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] egui::widgets::button::Button::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/widgets/button.rs
//     evidence={"module":590,"record_offset":87652,"parent_record_offset":87624,"ancestor_record_offset":87348,"inlinee":7335,"source_line":40,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":590,"record_offset":92172,"parent_record_offset":92148,"ancestor_record_offset":87348,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":590,"record_offset":92148,"parent_record_offset":92124,"ancestor_record_offset":87348,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":590,"record_offset":92196,"parent_record_offset":92172,"ancestor_record_offset":87348,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] rfd::file_dialog::impl$7::default
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"rfd","version":"0.10.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rfd-0.10.0/src/file_dialog.rs
//     evidence={"module":590,"record_offset":90172,"parent_record_offset":90148,"ancestor_record_offset":87348,"inlinee":8113,"source_line":18,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Serializer::pretty
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":590,"record_offset":90716,"parent_record_offset":90688,"ancestor_record_offset":87348,"inlinee":8091,"source_line":39,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Serializer::with_formatter
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":590,"record_offset":90740,"parent_record_offset":90716,"ancestor_record_offset":87348,"inlinee":8092,"source_line":52,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::to_vec_pretty
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":590,"record_offset":90352,"parent_record_offset":90304,"ancestor_record_offset":87348,"inlinee":8088,"source_line":2229,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::to_writer_pretty
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":590,"record_offset":90688,"parent_record_offset":90352,"ancestor_record_offset":87348,"inlinee":8089,"source_line":2197,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::OsStr::to_os_string
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":590,"record_offset":88084,"parent_record_offset":88060,"ancestor_record_offset":87348,"inlinee":5109,"source_line":986,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$10::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":590,"record_offset":88180,"parent_record_offset":88144,"ancestor_record_offset":87348,"inlinee":6708,"source_line":699,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::DirBuilder::create
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":590,"record_offset":89352,"parent_record_offset":89292,"ancestor_record_offset":87348,"inlinee":7542,"source_line":3441,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::DirBuilder::recursive
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":590,"record_offset":89324,"parent_record_offset":89292,"ancestor_record_offset":87348,"inlinee":7541,"source_line":3417,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::canonicalize
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":590,"record_offset":90228,"parent_record_offset":90204,"ancestor_record_offset":87348,"inlinee":8115,"source_line":3035,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::Slice::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":590,"record_offset":88108,"parent_record_offset":88084,"ancestor_record_offset":87348,"inlinee":5111,"source_line":265,"archive":"_pdb/C/modules/00590.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::impl$7::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":590,"record_offset":88216,"parent_record_offset":88180,"ancestor_record_offset":87348,"inlinee":6710,"source_line":70,"archive":"_pdb/C/modules/00590.jsonl.gz"}
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
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

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
// 信息 ID: 08fbab52fec7e013c83bd0f2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::ops::function::FnOnce::call_once<tiny_glade::gui::gui_debug::do_gui_debug::closure$0::closure$0::closure$16::closure_env$0,tuple$<ref_mut$<egui::ui::Ui> > > @ 0x6887cb -> 0xc0ec00 (direct_jump_outside_procedure); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::backend::input::input_replay_for_trailers::do_input_replay_ui::closure$0
// 信息 ID: 0156e17ffcddbc9df581b4af
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::backend::input::input_replay_for_trailers::do_input_replay_ui", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::backend::input::input_replay_for_trailers::do_input_replay_ui::closure_env$0 <- core::ops::function::FnOnce::call_once<country_core::systems::backend::input::input_replay_for_trailers::do_input_replay_ui::closure_env$0,tuple$<ref_mut$<egui::ui::Ui> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<country_core::systems::backend::input::input_replay_for_trailers::do_input_replay_ui::closure_env$0,tuple$<ref_mut$<egui::ui::Ui> > >; module=590, inline_record=87348; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::backend::input::input_replay_for_trailers::record_and_replay_input_for_trailers
// 信息 ID: 8daa4da6365fb10df37fce64
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 55cdcc6aa7c329956db2e394: void (*)(bevy_ecs::change_detection::ResMut<country_core::input::input_stream::InputRecordReplayState>,bevy_ecs::change_detection::Res<bevy_input::button_input::ButtonInput<enum2$<bevy_input::keyboard::KeyCode> > >,bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::Toolbar>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::change_detection::ResMut<country_core::input::input_stream::InputRecordReplayState>,bevy_ecs::change_detection::Res<bevy_input::button_input::ButtonInput<enum2$<bevy_input::keyboard::KeyCode> > >,bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::Toolbar>),tuple$<>,void (*)(bevy_ecs::change_detection::ResMut<country_core::input::input_stream::InputRecordReplayState>,bevy_ecs::change_detection::Res<bevy_input::button_input::ButtonInput<enum2$<bevy_input::keyboard::KeyCode> > >,bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::Toolbar>)> @ 0xa1b06a -> 0xc0ef70 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::input::input_stream::InputRecordReplayState>,bevy_ecs::change_detection::Res<bevy_input::button_input::ButtonInput<enum2$<bevy_input::keyboard::KeyCode> > >,bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::Toolbar>),tuple$<>,void (*)(bevy_ecs::change_detection::ResMut<country_core::input::input_stream::InputRecordReplayState>,bevy_ecs::change_detection::Res<bevy_input::button_input::ButtonInput<enum2$<bevy_input::keyboard::KeyCode> > >,bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::Toolbar>)> > @ 0x193f34c -> 0xc0ef70 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===

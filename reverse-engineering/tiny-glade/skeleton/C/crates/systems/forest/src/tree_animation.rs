// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/systems/forest/src/tree_animation.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 2d9eff173eb5acba8669274811eec68a8dad28e4c3aadae5267e1ec98109fae2
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: tree_animation.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 3935
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
// use bevy_ptr; // namespace candidate
//   package identities: [{"name":"bevy_ptr","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use forest; // namespace candidate
//   package identities: [{"name":"forest","source":"local_or_path","version":"0.1.0"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.30.10"}]
// use nonmax; // namespace candidate
//   package identities: [{"name":"nonmax","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.5.5"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.1"}]
// use rhapsody_core; // namespace candidate
//   package identities: [{"name":"rhapsody-core","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876","version":"0.1.0"}]
// use slotmap; // namespace candidate
//   package identities: [{"name":"slotmap","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.1.1"}]
// use std; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: forest::tree_animation::handplaced_tree_animation
// 记录形式: compiled_procedure; instances=1
// 源码行: 39
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3935,"record_offset":9056,"rva":42395136,"va":5411104256}
// 编译实例 RVAs: 0x286e600
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1408fb460
//     owner=unresolved_namespace; count=1
//     target_aliases=20 candidates; complete list: tree_animation.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-32978329","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411104256,"raw_hex":"e867ca08fe","size":5,"soft_fail":false,"target_va":5378126944,"va":5411105268,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5514678","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411104256,"raw_hex":"e8b6255400","size":5,"soft_fail":false,"target_va":5416621408,"va":5411106725,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-3353","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411104256,"raw_hex":"e8e7f2ffff","size":5,"soft_fail":false,"target_va":5411101056,"va":5411104404,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t5514559","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411104256,"raw_hex":"e83f255400","size":5,"soft_fail":false,"target_va":5416621376,"va":5411106812,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::audio_trigger_library::AudioTriggerCollection::boxes
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-31679213","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411104256,"raw_hex":"e8139d1cfe","size":5,"soft_fail":false,"target_va":5379425872,"va":5411105080,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary::get_triggers
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-31679249","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411104256,"raw_hex":"e8ef9c1cfe","size":5,"soft_fail":false,"target_va":5379425824,"va":5411105068,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-27141007","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411104256,"raw_hex":"e871dc61fe","size":5,"soft_fail":false,"target_va":5383963792,"va":5411104794,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::__RUST_STD_INTERNAL_VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-27132500","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411104256,"raw_hex":"e8acfd61fe","size":5,"soft_fail":false,"target_va":5383972224,"va":5411104719,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-27141721","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411104256,"raw_hex":"e8a7d961fe","size":5,"soft_fail":false,"target_va":5383964912,"va":5411106628,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-27141560","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411104256,"raw_hex":"e848da61fe","size":5,"soft_fail":false,"target_va":5383962816,"va":5411104371,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody_core::camera::CameraMatrices::eye_direction
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody-core","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t-27692431","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411104256,"raw_hex":"e8717259fe","size":5,"soft_fail":false,"target_va":5383412624,"va":5411105050,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody_core::camera::CameraMatrices::eye_position
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody-core","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t-27692498","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411104256,"raw_hex":"e82e7259fe","size":5,"soft_fail":false,"target_va":5383412544,"va":5411105037,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::native::lazy::Storage$LT$T$C$D$GT$::get_or_init_slow
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t5919244","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411104256,"raw_hex":"e80c525a00","size":5,"soft_fail":false,"target_va":5417025984,"va":5411106735,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::LocalKey$LT$T$GT$::with
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-6980","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411104256,"raw_hex":"e8bce4ffff","size":5,"soft_fail":false,"target_va":5411100192,"va":5411107167,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5512578","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411104256,"raw_hex":"e8821d5400","size":5,"soft_fail":false,"target_va":5416619360,"va":5411106777,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb3::intersect_segment
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-27802879","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411104256,"raw_hex":"e801c357fe","size":5,"soft_fail":false,"target_va":5383302496,"va":5411105370,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::pack_2x16f_uint
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-27831626","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411104256,"raw_hex":"e8b65257fe","size":5,"soft_fail":false,"target_va":5383274928,"va":5411106549,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":3935,"record_offset":11500,"parent_record_offset":9056,"ancestor_record_offset":9056,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$58::deref_mut
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":3935,"record_offset":17640,"parent_record_offset":9056,"ancestor_record_offset":9056,"inlinee":9404,"source_line":434,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::query::iter::impl$1::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":3935,"record_offset":12824,"parent_record_offset":9056,"ancestor_record_offset":9056,"inlinee":10317,"source_line":909,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::iter_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":3935,"record_offset":12388,"parent_record_offset":9056,"ancestor_record_offset":9056,"inlinee":18903,"source_line":704,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":3935,"record_offset":11620,"parent_record_offset":9056,"ancestor_record_offset":9056,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":3935,"record_offset":17728,"parent_record_offset":9056,"ancestor_record_offset":9056,"inlinee":5478,"source_line":1018,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::any
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":3935,"record_offset":11736,"parent_record_offset":9056,"ancestor_record_offset":9056,"inlinee":12047,"source_line":2863,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3935,"record_offset":17956,"parent_record_offset":9056,"ancestor_record_offset":9056,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_direct] forest::tree_animation::should_remove_tree_canopy_from_view
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"forest","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/forest/src/tree_animation.rs
//     evidence={"module":3935,"record_offset":10648,"parent_record_offset":9056,"ancestor_record_offset":9056,"inlinee":134486,"source_line":152,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3935,"record_offset":10692,"parent_record_offset":9056,"ancestor_record_offset":9056,"inlinee":8276,"source_line":589,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3935,"record_offset":9244,"parent_record_offset":9056,"ancestor_record_offset":9056,"inlinee":9267,"source_line":626,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3935,"record_offset":9636,"parent_record_offset":9056,"ancestor_record_offset":9056,"inlinee":8242,"source_line":659,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::powi
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":3935,"record_offset":17844,"parent_record_offset":9056,"ancestor_record_offset":9056,"inlinee":17052,"source_line":323,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_direct] utils::smoothstep
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":3935,"record_offset":11652,"parent_record_offset":9056,"ancestor_record_offset":9056,"inlinee":11673,"source_line":233,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3935,"record_offset":12656,"parent_record_offset":12632,"ancestor_record_offset":9056,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3935,"record_offset":12704,"parent_record_offset":12680,"ancestor_record_offset":9056,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3935,"record_offset":12680,"parent_record_offset":12656,"ancestor_record_offset":9056,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3935,"record_offset":12632,"parent_record_offset":12608,"ancestor_record_offset":9056,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3935,"record_offset":12608,"parent_record_offset":12576,"ancestor_record_offset":9056,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3935,"record_offset":13168,"parent_record_offset":13144,"ancestor_record_offset":9056,"inlinee":5735,"source_line":3021,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3935,"record_offset":13192,"parent_record_offset":13168,"ancestor_record_offset":9056,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3935,"record_offset":12576,"parent_record_offset":12548,"ancestor_record_offset":9056,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":3935,"record_offset":15836,"parent_record_offset":15812,"ancestor_record_offset":9056,"inlinee":7082,"source_line":661,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::entities
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":3935,"record_offset":16064,"parent_record_offset":12852,"ancestor_record_offset":9056,"inlinee":10334,"source_line":484,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":3935,"record_offset":14612,"parent_record_offset":12852,"ancestor_record_offset":9056,"inlinee":10331,"source_line":655,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::table_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":3935,"record_offset":14696,"parent_record_offset":12852,"ancestor_record_offset":9056,"inlinee":8568,"source_line":478,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::ArchetypeEntity::table_row
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":3935,"record_offset":16288,"parent_record_offset":12852,"ancestor_record_offset":9056,"inlinee":10337,"source_line":346,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetypes::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":3935,"record_offset":14528,"parent_record_offset":12852,"ancestor_record_offset":9056,"inlinee":10330,"source_line":864,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":3935,"record_offset":11528,"parent_record_offset":11500,"ancestor_record_offset":9056,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$57::set_changed
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":3935,"record_offset":17664,"parent_record_offset":17640,"ancestor_record_offset":9056,"inlinee":9399,"source_line":397,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::StorageSwitch::extract
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":3935,"record_offset":16396,"parent_record_offset":16356,"ancestor_record_offset":9056,"inlinee":9590,"source_line":2964,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::matches_component_set
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":3935,"record_offset":14164,"parent_record_offset":14136,"ancestor_record_offset":9056,"inlinee":7078,"source_line":1577,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$48::set_archetype
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":3935,"record_offset":14812,"parent_record_offset":14788,"ancestor_record_offset":9056,"inlinee":11800,"source_line":1912,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$48::set_table
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":3935,"record_offset":13252,"parent_record_offset":13228,"ancestor_record_offset":9056,"inlinee":11801,"source_line":1927,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$49::fetch
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":3935,"record_offset":16356,"parent_record_offset":16324,"ancestor_record_offset":9056,"inlinee":11822,"source_line":1987,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$49::fetch::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":3935,"record_offset":16436,"parent_record_offset":16396,"ancestor_record_offset":9056,"inlinee":9577,"source_line":1994,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$55::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":3935,"record_offset":15756,"parent_record_offset":14788,"ancestor_record_offset":9056,"inlinee":18914,"source_line":2203,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$55::set_archetype::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":3935,"record_offset":15812,"parent_record_offset":15784,"ancestor_record_offset":9056,"inlinee":18916,"source_line":2209,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$55::set_table
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":3935,"record_offset":14136,"parent_record_offset":13228,"ancestor_record_offset":9056,"inlinee":18910,"source_line":2219,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$55::set_table::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":3935,"record_offset":14192,"parent_record_offset":14164,"ancestor_record_offset":9056,"inlinee":18912,"source_line":2224,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$56::fetch
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":3935,"record_offset":16972,"parent_record_offset":16324,"ancestor_record_offset":9056,"inlinee":18918,"source_line":2277,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$92::fetch
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":3935,"record_offset":16324,"parent_record_offset":12852,"ancestor_record_offset":9056,"inlinee":11825,"source_line":2520,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":3935,"record_offset":12524,"parent_record_offset":12412,"ancestor_record_offset":9056,"inlinee":10316,"source_line":40,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::init
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":3935,"record_offset":12548,"parent_record_offset":12524,"ancestor_record_offset":9056,"inlinee":9124,"source_line":2408,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":3935,"record_offset":12852,"parent_record_offset":12824,"ancestor_record_offset":9056,"inlinee":10318,"source_line":2511,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$3::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/world_query.rs
//     evidence={"module":3935,"record_offset":14788,"parent_record_offset":12852,"ancestor_record_offset":9056,"inlinee":10710,"source_line":182,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$3::set_table
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/world_query.rs
//     evidence={"module":3935,"record_offset":13228,"parent_record_offset":12852,"ancestor_record_offset":9056,"inlinee":10708,"source_line":195,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_array::BlobArray::get_sub_slice
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_array.rs
//     evidence={"module":3935,"record_offset":13608,"parent_record_offset":13584,"ancestor_record_offset":9056,"inlinee":10327,"source_line":143,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::contains
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":3935,"record_offset":14264,"parent_record_offset":14240,"ancestor_record_offset":9056,"inlinee":7086,"source_line":47,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":3935,"record_offset":13344,"parent_record_offset":13316,"ancestor_record_offset":9056,"inlinee":8615,"source_line":56,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::contains
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":3935,"record_offset":14240,"parent_record_offset":14216,"ancestor_record_offset":9056,"inlinee":7084,"source_line":398,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":3935,"record_offset":13316,"parent_record_offset":13288,"ancestor_record_offset":9056,"inlinee":8626,"source_line":405,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":3935,"record_offset":13480,"parent_record_offset":13456,"ancestor_record_offset":9056,"inlinee":8628,"source_line":406,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":3935,"record_offset":13288,"parent_record_offset":13252,"ancestor_record_offset":9056,"inlinee":8625,"source_line":489,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::has_column
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":3935,"record_offset":14216,"parent_record_offset":14192,"ancestor_record_offset":9056,"inlinee":18913,"source_line":510,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":3935,"record_offset":13144,"parent_record_offset":12852,"ancestor_record_offset":9056,"inlinee":10320,"source_line":646,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":3935,"record_offset":16488,"parent_record_offset":16436,"ancestor_record_offset":9056,"inlinee":8632,"source_line":114,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":3935,"record_offset":13060,"parent_record_offset":12852,"ancestor_record_offset":9056,"inlinee":10319,"source_line":752,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_changed_ticks_slice
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/column.rs
//     evidence={"module":3935,"record_offset":13640,"parent_record_offset":13252,"ancestor_record_offset":9056,"inlinee":11819,"source_line":313,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_data_slice
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/column.rs
//     evidence={"module":3935,"record_offset":13584,"parent_record_offset":13252,"ancestor_record_offset":9056,"inlinee":10326,"source_line":297,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::thin_array_ptr::ThinArrayPtr::as_slice
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/thin_array_ptr.rs
//     evidence={"module":3935,"record_offset":13664,"parent_record_offset":13640,"ancestor_record_offset":9056,"inlinee":11820,"source_line":300,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::copy_unsafe
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":3935,"record_offset":12464,"parent_record_offset":12440,"ancestor_record_offset":9056,"inlinee":11851,"source_line":639,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":3935,"record_offset":12488,"parent_record_offset":12464,"ancestor_record_offset":9056,"inlinee":11852,"source_line":522,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::reborrow
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":3935,"record_offset":12412,"parent_record_offset":12388,"ancestor_record_offset":9056,"inlinee":18904,"source_line":604,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::reborrow_unsafe
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":3935,"record_offset":12440,"parent_record_offset":12412,"ancestor_record_offset":9056,"inlinee":11850,"source_line":621,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"bevy_ptr","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.17.3/src/lib.rs
//     evidence={"module":3935,"record_offset":16620,"parent_record_offset":16436,"ancestor_record_offset":9056,"inlinee":9579,"source_line":1073,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\bool.rs
//     evidence={"module":3935,"record_offset":16996,"parent_record_offset":16972,"ancestor_record_offset":9056,"inlinee":4276,"source_line":65,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":3935,"record_offset":11020,"parent_record_offset":10992,"ancestor_record_offset":9056,"inlinee":6507,"source_line":2061,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":3935,"record_offset":11048,"parent_record_offset":11020,"ancestor_record_offset":9056,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":3935,"record_offset":10968,"parent_record_offset":10940,"ancestor_record_offset":9056,"inlinee":6546,"source_line":1221,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":3935,"record_offset":10992,"parent_record_offset":10968,"ancestor_record_offset":9056,"inlinee":6505,"source_line":1255,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":3935,"record_offset":11108,"parent_record_offset":10992,"ancestor_record_offset":9056,"inlinee":5177,"source_line":2443,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$52::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":3935,"record_offset":11220,"parent_record_offset":11196,"ancestor_record_offset":9056,"inlinee":4995,"source_line":2052,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":3935,"record_offset":12036,"parent_record_offset":11820,"ancestor_record_offset":9056,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::powi
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":3935,"record_offset":17868,"parent_record_offset":17844,"ancestor_record_offset":9056,"inlinee":17053,"source_line":2010,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":3935,"record_offset":11864,"parent_record_offset":11820,"ancestor_record_offset":9056,"inlinee":5459,"source_line":79,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::any::check::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":3935,"record_offset":12100,"parent_record_offset":11760,"ancestor_record_offset":9056,"inlinee":12049,"source_line":2870,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":3935,"record_offset":11760,"parent_record_offset":11736,"ancestor_record_offset":9056,"inlinee":5047,"source_line":2461,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":3935,"record_offset":11072,"parent_record_offset":11048,"ancestor_record_offset":9056,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$353::add_assign
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":3935,"record_offset":12248,"parent_record_offset":12220,"ancestor_record_offset":9056,"inlinee":8870,"source_line":791,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$385::sub_assign
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":3935,"record_offset":12188,"parent_record_offset":12164,"ancestor_record_offset":9056,"inlinee":11677,"source_line":862,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":3935,"record_offset":10840,"parent_record_offset":10800,"ancestor_record_offset":9056,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3935,"record_offset":13368,"parent_record_offset":13344,"ancestor_record_offset":9056,"inlinee":5449,"source_line":1541,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3935,"record_offset":13416,"parent_record_offset":13392,"ancestor_record_offset":9056,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3935,"record_offset":12340,"parent_record_offset":12124,"ancestor_record_offset":9056,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3935,"record_offset":12068,"parent_record_offset":11792,"ancestor_record_offset":9056,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":3935,"record_offset":9584,"parent_record_offset":9556,"ancestor_record_offset":9056,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":3935,"record_offset":11456,"parent_record_offset":10800,"ancestor_record_offset":9056,"inlinee":6443,"source_line":266,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3935,"record_offset":11172,"parent_record_offset":10940,"ancestor_record_offset":9056,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3935,"record_offset":12772,"parent_record_offset":12748,"ancestor_record_offset":9056,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":3935,"record_offset":13028,"parent_record_offset":12968,"ancestor_record_offset":9056,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":3935,"record_offset":10056,"parent_record_offset":10032,"ancestor_record_offset":9056,"inlinee":8268,"source_line":575,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":3935,"record_offset":10032,"parent_record_offset":10008,"ancestor_record_offset":9056,"inlinee":8267,"source_line":728,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":3935,"record_offset":10240,"parent_record_offset":10212,"ancestor_record_offset":9056,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$49::slice_contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":3935,"record_offset":10340,"parent_record_offset":10312,"ancestor_record_offset":9056,"inlinee":8260,"source_line":405,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":3935,"record_offset":10312,"parent_record_offset":10284,"ancestor_record_offset":9056,"inlinee":8258,"source_line":2589,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":3935,"record_offset":13084,"parent_record_offset":13060,"ancestor_record_offset":9056,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":3935,"record_offset":17032,"parent_record_offset":12852,"ancestor_record_offset":9056,"inlinee":4887,"source_line":639,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":3935,"record_offset":13108,"parent_record_offset":13084,"ancestor_record_offset":9056,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":3935,"record_offset":13536,"parent_record_offset":13504,"ancestor_record_offset":9056,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":3935,"record_offset":12748,"parent_record_offset":12576,"ancestor_record_offset":9056,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":3935,"record_offset":9984,"parent_record_offset":9960,"ancestor_record_offset":9056,"inlinee":8265,"source_line":81,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":3935,"record_offset":10008,"parent_record_offset":9984,"ancestor_record_offset":9056,"inlinee":8266,"source_line":119,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":3935,"record_offset":10368,"parent_record_offset":10340,"ancestor_record_offset":9056,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":3935,"record_offset":11904,"parent_record_offset":11864,"ancestor_record_offset":9056,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$172::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":3935,"record_offset":9960,"parent_record_offset":9844,"ancestor_record_offset":9056,"inlinee":8264,"source_line":438,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":3935,"record_offset":9372,"parent_record_offset":9348,"ancestor_record_offset":9056,"inlinee":9253,"source_line":377,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":3935,"record_offset":9672,"parent_record_offset":9636,"ancestor_record_offset":9056,"inlinee":8243,"source_line":1527,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":3935,"record_offset":9812,"parent_record_offset":9780,"ancestor_record_offset":9056,"inlinee":8253,"source_line":125,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":3935,"record_offset":9780,"parent_record_offset":9744,"ancestor_record_offset":9056,"inlinee":8251,"source_line":215,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":3935,"record_offset":9720,"parent_record_offset":9696,"ancestor_record_offset":9056,"inlinee":8247,"source_line":315,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":3935,"record_offset":9744,"parent_record_offset":9720,"ancestor_record_offset":9056,"inlinee":8249,"source_line":709,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":3935,"record_offset":9696,"parent_record_offset":9672,"ancestor_record_offset":9056,"inlinee":8245,"source_line":798,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":3935,"record_offset":10284,"parent_record_offset":9744,"ancestor_record_offset":9056,"inlinee":8257,"source_line":659,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":3935,"record_offset":9348,"parent_record_offset":9316,"ancestor_record_offset":9056,"inlinee":9270,"source_line":438,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":3935,"record_offset":9316,"parent_record_offset":9288,"ancestor_record_offset":9056,"inlinee":9269,"source_line":468,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":3935,"record_offset":9476,"parent_record_offset":9444,"ancestor_record_offset":9056,"inlinee":9252,"source_line":507,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":3935,"record_offset":9532,"parent_record_offset":9476,"ancestor_record_offset":9056,"inlinee":9254,"source_line":527,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":3935,"record_offset":9444,"parent_record_offset":9412,"ancestor_record_offset":9056,"inlinee":8274,"source_line":539,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":3935,"record_offset":9288,"parent_record_offset":9244,"ancestor_record_offset":9056,"inlinee":8272,"source_line":62,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":3935,"record_offset":9556,"parent_record_offset":9532,"ancestor_record_offset":9056,"inlinee":8315,"source_line":189,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":3935,"record_offset":9844,"parent_record_offset":9812,"ancestor_record_offset":9056,"inlinee":8255,"source_line":78,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":3935,"record_offset":10128,"parent_record_offset":9844,"ancestor_record_offset":9056,"inlinee":8270,"source_line":16,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":3935,"record_offset":10100,"parent_record_offset":9844,"ancestor_record_offset":9056,"inlinee":8269,"source_line":10,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":3935,"record_offset":9904,"parent_record_offset":9844,"ancestor_record_offset":9056,"inlinee":8262,"source_line":23,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] forest::tree_animation::handplaced_tree_animation::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"forest","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/forest/src/tree_animation.rs
//     evidence={"module":3935,"record_offset":12124,"parent_record_offset":12100,"ancestor_record_offset":9056,"inlinee":134488,"source_line":74,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":3935,"record_offset":12312,"parent_record_offset":12124,"ancestor_record_offset":9056,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$44::add_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":3935,"record_offset":12220,"parent_record_offset":12124,"ancestor_record_offset":9056,"inlinee":36831,"source_line":1676,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$60::sub_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":3935,"record_offset":12164,"parent_record_offset":12124,"ancestor_record_offset":9056,"inlinee":36833,"source_line":1816,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxU32::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":3935,"record_offset":16548,"parent_record_offset":16488,"ancestor_record_offset":9056,"inlinee":8634,"source_line":153,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":3935,"record_offset":13504,"parent_record_offset":13480,"ancestor_record_offset":9056,"inlinee":4193,"source_line":153,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3935,"record_offset":10728,"parent_record_offset":10692,"ancestor_record_offset":9056,"inlinee":8278,"source_line":425,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3935,"record_offset":18020,"parent_record_offset":17988,"ancestor_record_offset":9056,"inlinee":8283,"source_line":601,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3935,"record_offset":18460,"parent_record_offset":18256,"ancestor_record_offset":9056,"inlinee":6607,"source_line":602,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":3935,"record_offset":10868,"parent_record_offset":10840,"ancestor_record_offset":9056,"inlinee":8280,"source_line":92,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3935,"record_offset":10940,"parent_record_offset":10800,"ancestor_record_offset":9056,"inlinee":6545,"source_line":429,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3935,"record_offset":11144,"parent_record_offset":10940,"ancestor_record_offset":9056,"inlinee":6548,"source_line":591,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] slotmap::basic::Slot::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"slotmap","version":"1.1.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/slotmap-1.1.1/src/basic.rs
//     evidence={"module":3935,"record_offset":11980,"parent_record_offset":11820,"ancestor_record_offset":9056,"inlinee":36827,"source_line":48,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] slotmap::basic::Slot::occupied
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"slotmap","version":"1.1.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/slotmap-1.1.1/src/basic.rs
//     evidence={"module":3935,"record_offset":12004,"parent_record_offset":11980,"ancestor_record_offset":9056,"inlinee":4742,"source_line":44,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] slotmap::basic::impl$16::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"slotmap","version":"1.1.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/slotmap-1.1.1/src/basic.rs
//     evidence={"module":3935,"record_offset":11820,"parent_record_offset":11792,"ancestor_record_offset":9056,"inlinee":36826,"source_line":1169,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] slotmap::basic::impl$19::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"slotmap","version":"1.1.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/slotmap-1.1.1/src/basic.rs
//     evidence={"module":3935,"record_offset":11792,"parent_record_offset":11760,"ancestor_record_offset":9056,"inlinee":36824,"source_line":1222,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":3935,"record_offset":10896,"parent_record_offset":10868,"ancestor_record_offset":9056,"inlinee":5707,"source_line":58,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":3935,"record_offset":10800,"parent_record_offset":10764,"ancestor_record_offset":9056,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":3935,"record_offset":10764,"parent_record_offset":10728,"ancestor_record_offset":9056,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: forest::tree_animation::handplaced_tree_animation::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 74
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x20D58
// 首条位置: {"module":3935,"inlinee":134488}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3935,"record_offset":12340,"parent_record_offset":12124,"ancestor_record_offset":12124,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":3935,"record_offset":12312,"parent_record_offset":12124,"ancestor_record_offset":12124,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$44::add_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":3935,"record_offset":12220,"parent_record_offset":12124,"ancestor_record_offset":12124,"inlinee":36831,"source_line":1676,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$60::sub_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":3935,"record_offset":12164,"parent_record_offset":12124,"ancestor_record_offset":12124,"inlinee":36833,"source_line":1816,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$353::add_assign
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":3935,"record_offset":12248,"parent_record_offset":12220,"ancestor_record_offset":12124,"inlinee":8870,"source_line":791,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$385::sub_assign
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":3935,"record_offset":12188,"parent_record_offset":12164,"ancestor_record_offset":12124,"inlinee":11677,"source_line":862,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: forest::tree_animation::should_remove_tree_canopy_from_view
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 152
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x20D56
// 首条位置: {"module":3935,"inlinee":134486}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: forest::tree_animation::forest_tree_animation
// 记录形式: compiled_procedure; instances=1
// 源码行: 181
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3935,"record_offset":2072,"rva":42392992,"va":5411102112}
// 编译实例 RVAs: 0x286dda0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f550
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: tree_animation.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5247498","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411102112,"raw_hex":"e80a125000","size":5,"soft_fail":false,"target_va":5416351056,"va":5411103553,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5517646","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411102112,"raw_hex":"e84e315400","size":5,"soft_fail":false,"target_va":5416621408,"va":5411103757,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5517939","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411102112,"raw_hex":"e873325400","size":5,"soft_fail":false,"target_va":5416621762,"va":5411103818,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-1171","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411102112,"raw_hex":"e86dfbffff","size":5,"soft_fail":false,"target_va":5411101056,"va":5411102222,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t5517515","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411102112,"raw_hex":"e8cb305400","size":5,"soft_fail":false,"target_va":5416621376,"va":5411103856,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-27138786","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411102112,"raw_hex":"e81ee561fe","size":5,"soft_fail":false,"target_va":5383963792,"va":5411102573,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::__RUST_STD_INTERNAL_VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-27130281","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411102112,"raw_hex":"e8570662fe","size":5,"soft_fail":false,"target_va":5383972224,"va":5411102500,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-27138788","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411102112,"raw_hex":"e81ce561fe","size":5,"soft_fail":false,"target_va":5383964912,"va":5411103695,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-27139373","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411102112,"raw_hex":"e8d3e261fe","size":5,"soft_fail":false,"target_va":5383962816,"va":5411102184,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::native::lazy::Storage$LT$T$C$D$GT$::get_or_init_slow
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t5922212","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411102112,"raw_hex":"e8a45d5a00","size":5,"soft_fail":false,"target_va":5417025984,"va":5411103767,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::LocalKey$LT$T$GT$::with
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-3923","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411102112,"raw_hex":"e8adf0ffff","size":5,"soft_fail":false,"target_va":5411100192,"va":5411104110,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5515550","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411102112,"raw_hex":"e81e295400","size":5,"soft_fail":false,"target_va":5416619360,"va":5411103805,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::easing_functions::EasingFunction::apply
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-27806256","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411102112,"raw_hex":"e8d0b557fe","size":5,"soft_fail":false,"target_va":5383297200,"va":5411103451,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::impl$15::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3935,"record_offset":7964,"parent_record_offset":2072,"ancestor_record_offset":2072,"inlinee":8424,"source_line":3811,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":3935,"record_offset":7908,"parent_record_offset":2072,"ancestor_record_offset":2072,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$58::deref_mut
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":3935,"record_offset":7356,"parent_record_offset":2072,"ancestor_record_offset":2072,"inlinee":9404,"source_line":434,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::message_writer::MessageWriter::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_writer.rs
//     evidence={"module":3935,"record_offset":7556,"parent_record_offset":2072,"ancestor_record_offset":2072,"inlinee":11206,"source_line":69,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::query::iter::impl$1::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":3935,"record_offset":4892,"parent_record_offset":2072,"ancestor_record_offset":2072,"inlinee":10317,"source_line":909,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::iter_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":3935,"record_offset":4452,"parent_record_offset":2072,"ancestor_record_offset":2072,"inlinee":18903,"source_line":704,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":3935,"record_offset":7468,"parent_record_offset":2072,"ancestor_record_offset":2072,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3935,"record_offset":8072,"parent_record_offset":2072,"ancestor_record_offset":2072,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3935,"record_offset":3644,"parent_record_offset":2072,"ancestor_record_offset":2072,"inlinee":8276,"source_line":589,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3935,"record_offset":2244,"parent_record_offset":2072,"ancestor_record_offset":2072,"inlinee":9267,"source_line":626,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3935,"record_offset":2632,"parent_record_offset":2072,"ancestor_record_offset":2072,"inlinee":8242,"source_line":659,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3935,"record_offset":4724,"parent_record_offset":4700,"ancestor_record_offset":2072,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3935,"record_offset":4772,"parent_record_offset":4748,"ancestor_record_offset":2072,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3935,"record_offset":4748,"parent_record_offset":4724,"ancestor_record_offset":2072,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3935,"record_offset":7740,"parent_record_offset":7704,"ancestor_record_offset":2072,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3935,"record_offset":4700,"parent_record_offset":4676,"ancestor_record_offset":2072,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3935,"record_offset":4676,"parent_record_offset":4644,"ancestor_record_offset":2072,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3935,"record_offset":5196,"parent_record_offset":5172,"ancestor_record_offset":2072,"inlinee":5735,"source_line":3021,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3935,"record_offset":5220,"parent_record_offset":5196,"ancestor_record_offset":2072,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3935,"record_offset":7676,"parent_record_offset":7640,"ancestor_record_offset":2072,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3935,"record_offset":7704,"parent_record_offset":7676,"ancestor_record_offset":2072,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3935,"record_offset":4644,"parent_record_offset":4616,"ancestor_record_offset":2072,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":3935,"record_offset":6272,"parent_record_offset":6248,"ancestor_record_offset":2072,"inlinee":7082,"source_line":661,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::entities
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":3935,"record_offset":6500,"parent_record_offset":4916,"ancestor_record_offset":2072,"inlinee":10334,"source_line":484,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":3935,"record_offset":6088,"parent_record_offset":4916,"ancestor_record_offset":2072,"inlinee":10331,"source_line":655,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::ArchetypeEntity::table_row
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":3935,"record_offset":6752,"parent_record_offset":4916,"ancestor_record_offset":2072,"inlinee":10337,"source_line":346,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetypes::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":3935,"record_offset":6004,"parent_record_offset":4916,"ancestor_record_offset":2072,"inlinee":10330,"source_line":864,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":3935,"record_offset":7612,"parent_record_offset":7580,"ancestor_record_offset":2072,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":3935,"record_offset":7580,"parent_record_offset":7556,"ancestor_record_offset":2072,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$57::set_changed
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":3935,"record_offset":7380,"parent_record_offset":7356,"ancestor_record_offset":2072,"inlinee":9399,"source_line":397,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::messages::Messages::write_with_caller
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs
//     evidence={"module":3935,"record_offset":7640,"parent_record_offset":7580,"ancestor_record_offset":2072,"inlinee":8681,"source_line":129,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::StorageSwitch::extract
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":3935,"record_offset":6828,"parent_record_offset":6804,"ancestor_record_offset":2072,"inlinee":9590,"source_line":2964,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::matches_component_set
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":3935,"record_offset":5388,"parent_record_offset":5364,"ancestor_record_offset":2072,"inlinee":7078,"source_line":1577,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$48::set_table
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":3935,"record_offset":5280,"parent_record_offset":5256,"ancestor_record_offset":2072,"inlinee":11801,"source_line":1927,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$49::fetch
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":3935,"record_offset":6804,"parent_record_offset":6780,"ancestor_record_offset":2072,"inlinee":11822,"source_line":1987,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$49::fetch::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":3935,"record_offset":6852,"parent_record_offset":6828,"ancestor_record_offset":2072,"inlinee":9577,"source_line":1994,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$55::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":3935,"record_offset":6196,"parent_record_offset":6172,"ancestor_record_offset":2072,"inlinee":18914,"source_line":2203,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$55::set_archetype::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":3935,"record_offset":6248,"parent_record_offset":6220,"ancestor_record_offset":2072,"inlinee":18916,"source_line":2209,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$55::set_table
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":3935,"record_offset":5364,"parent_record_offset":5256,"ancestor_record_offset":2072,"inlinee":18910,"source_line":2219,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$55::set_table::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":3935,"record_offset":5416,"parent_record_offset":5388,"ancestor_record_offset":2072,"inlinee":18912,"source_line":2224,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$95::fetch
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":3935,"record_offset":6780,"parent_record_offset":4916,"ancestor_record_offset":2072,"inlinee":18908,"source_line":2520,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":3935,"record_offset":4592,"parent_record_offset":4476,"ancestor_record_offset":2072,"inlinee":10316,"source_line":40,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::init
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":3935,"record_offset":4616,"parent_record_offset":4592,"ancestor_record_offset":2072,"inlinee":9124,"source_line":2408,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":3935,"record_offset":4916,"parent_record_offset":4892,"ancestor_record_offset":2072,"inlinee":10318,"source_line":2511,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$4::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/world_query.rs
//     evidence={"module":3935,"record_offset":6172,"parent_record_offset":4916,"ancestor_record_offset":2072,"inlinee":18906,"source_line":182,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$4::set_table
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/world_query.rs
//     evidence={"module":3935,"record_offset":5256,"parent_record_offset":4916,"ancestor_record_offset":2072,"inlinee":18905,"source_line":195,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::contains
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":3935,"record_offset":5488,"parent_record_offset":5464,"ancestor_record_offset":2072,"inlinee":7086,"source_line":47,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::contains
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":3935,"record_offset":5464,"parent_record_offset":5440,"ancestor_record_offset":2072,"inlinee":7084,"source_line":398,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":3935,"record_offset":5328,"parent_record_offset":5304,"ancestor_record_offset":2072,"inlinee":8626,"source_line":405,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::entities
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":3935,"record_offset":5660,"parent_record_offset":4916,"ancestor_record_offset":2072,"inlinee":10328,"source_line":208,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":3935,"record_offset":5304,"parent_record_offset":5280,"ancestor_record_offset":2072,"inlinee":8625,"source_line":489,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::has_column
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":3935,"record_offset":5440,"parent_record_offset":5416,"ancestor_record_offset":2072,"inlinee":18913,"source_line":510,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":3935,"record_offset":5172,"parent_record_offset":4916,"ancestor_record_offset":2072,"inlinee":10320,"source_line":646,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":3935,"record_offset":6876,"parent_record_offset":6852,"ancestor_record_offset":2072,"inlinee":8632,"source_line":114,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":3935,"record_offset":5088,"parent_record_offset":4916,"ancestor_record_offset":2072,"inlinee":10319,"source_line":752,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::copy_unsafe
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":3935,"record_offset":4532,"parent_record_offset":4508,"ancestor_record_offset":2072,"inlinee":11851,"source_line":639,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":3935,"record_offset":4556,"parent_record_offset":4532,"ancestor_record_offset":2072,"inlinee":11852,"source_line":522,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::reborrow
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":3935,"record_offset":4476,"parent_record_offset":4452,"ancestor_record_offset":2072,"inlinee":18904,"source_line":604,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::reborrow_unsafe
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":3935,"record_offset":4508,"parent_record_offset":4476,"ancestor_record_offset":2072,"inlinee":11850,"source_line":621,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ptr","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.17.3/src/lib.rs
//     evidence={"module":3935,"record_offset":7048,"parent_record_offset":7020,"ancestor_record_offset":2072,"inlinee":9579,"source_line":1073,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":3935,"record_offset":3972,"parent_record_offset":3944,"ancestor_record_offset":2072,"inlinee":6507,"source_line":2061,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":3935,"record_offset":4000,"parent_record_offset":3972,"ancestor_record_offset":2072,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":3935,"record_offset":3920,"parent_record_offset":3892,"ancestor_record_offset":2072,"inlinee":6546,"source_line":1221,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":3935,"record_offset":3944,"parent_record_offset":3920,"ancestor_record_offset":2072,"inlinee":6505,"source_line":1255,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":3935,"record_offset":4060,"parent_record_offset":3944,"ancestor_record_offset":2072,"inlinee":5177,"source_line":2443,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$52::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":3935,"record_offset":4172,"parent_record_offset":4148,"ancestor_record_offset":2072,"inlinee":4995,"source_line":2052,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":3935,"record_offset":4024,"parent_record_offset":4000,"ancestor_record_offset":2072,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":3935,"record_offset":3792,"parent_record_offset":3752,"ancestor_record_offset":2072,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3935,"record_offset":5596,"parent_record_offset":5572,"ancestor_record_offset":2072,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":3935,"record_offset":2580,"parent_record_offset":2552,"ancestor_record_offset":2072,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":3935,"record_offset":4408,"parent_record_offset":3752,"ancestor_record_offset":2072,"inlinee":6443,"source_line":266,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3935,"record_offset":4124,"parent_record_offset":3892,"ancestor_record_offset":2072,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3935,"record_offset":4840,"parent_record_offset":4816,"ancestor_record_offset":2072,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":3935,"record_offset":5056,"parent_record_offset":4996,"ancestor_record_offset":2072,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":3935,"record_offset":3052,"parent_record_offset":3028,"ancestor_record_offset":2072,"inlinee":8268,"source_line":575,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":3935,"record_offset":3028,"parent_record_offset":3004,"ancestor_record_offset":2072,"inlinee":8267,"source_line":728,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":3935,"record_offset":3236,"parent_record_offset":3208,"ancestor_record_offset":2072,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3935,"record_offset":7860,"parent_record_offset":7704,"ancestor_record_offset":2072,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$49::slice_contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":3935,"record_offset":3336,"parent_record_offset":3308,"ancestor_record_offset":2072,"inlinee":8260,"source_line":405,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":3935,"record_offset":3308,"parent_record_offset":3280,"ancestor_record_offset":2072,"inlinee":8258,"source_line":2589,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":3935,"record_offset":5112,"parent_record_offset":5088,"ancestor_record_offset":2072,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":3935,"record_offset":5856,"parent_record_offset":4916,"ancestor_record_offset":2072,"inlinee":4887,"source_line":639,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":3935,"record_offset":7996,"parent_record_offset":7964,"ancestor_record_offset":2072,"inlinee":8426,"source_line":30,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":3935,"record_offset":5136,"parent_record_offset":5112,"ancestor_record_offset":2072,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":3935,"record_offset":5880,"parent_record_offset":5856,"ancestor_record_offset":2072,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":3935,"record_offset":8028,"parent_record_offset":7996,"ancestor_record_offset":2072,"inlinee":8559,"source_line":276,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":3935,"record_offset":4816,"parent_record_offset":4644,"ancestor_record_offset":2072,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":3935,"record_offset":2980,"parent_record_offset":2956,"ancestor_record_offset":2072,"inlinee":8265,"source_line":81,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":3935,"record_offset":3004,"parent_record_offset":2980,"ancestor_record_offset":2072,"inlinee":8266,"source_line":119,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":3935,"record_offset":3364,"parent_record_offset":3336,"ancestor_record_offset":2072,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":3935,"record_offset":4996,"parent_record_offset":4916,"ancestor_record_offset":2072,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$172::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":3935,"record_offset":2956,"parent_record_offset":2840,"ancestor_record_offset":2072,"inlinee":8264,"source_line":438,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":3935,"record_offset":2372,"parent_record_offset":2348,"ancestor_record_offset":2072,"inlinee":9253,"source_line":377,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":3935,"record_offset":2668,"parent_record_offset":2632,"ancestor_record_offset":2072,"inlinee":8243,"source_line":1527,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":3935,"record_offset":2808,"parent_record_offset":2776,"ancestor_record_offset":2072,"inlinee":8253,"source_line":125,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":3935,"record_offset":2776,"parent_record_offset":2740,"ancestor_record_offset":2072,"inlinee":8251,"source_line":215,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":3935,"record_offset":2716,"parent_record_offset":2692,"ancestor_record_offset":2072,"inlinee":8247,"source_line":315,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":3935,"record_offset":2740,"parent_record_offset":2716,"ancestor_record_offset":2072,"inlinee":8249,"source_line":709,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":3935,"record_offset":2692,"parent_record_offset":2668,"ancestor_record_offset":2072,"inlinee":8245,"source_line":798,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":3935,"record_offset":3280,"parent_record_offset":2740,"ancestor_record_offset":2072,"inlinee":8257,"source_line":659,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":3935,"record_offset":2348,"parent_record_offset":2316,"ancestor_record_offset":2072,"inlinee":9270,"source_line":438,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":3935,"record_offset":2316,"parent_record_offset":2288,"ancestor_record_offset":2072,"inlinee":9269,"source_line":468,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":3935,"record_offset":2472,"parent_record_offset":2440,"ancestor_record_offset":2072,"inlinee":9252,"source_line":507,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":3935,"record_offset":2528,"parent_record_offset":2472,"ancestor_record_offset":2072,"inlinee":9254,"source_line":527,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":3935,"record_offset":2440,"parent_record_offset":2412,"ancestor_record_offset":2072,"inlinee":8274,"source_line":539,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":3935,"record_offset":2288,"parent_record_offset":2244,"ancestor_record_offset":2072,"inlinee":8272,"source_line":62,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":3935,"record_offset":2552,"parent_record_offset":2528,"ancestor_record_offset":2072,"inlinee":8315,"source_line":189,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":3935,"record_offset":2840,"parent_record_offset":2808,"ancestor_record_offset":2072,"inlinee":8255,"source_line":78,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":3935,"record_offset":3124,"parent_record_offset":2840,"ancestor_record_offset":2072,"inlinee":8270,"source_line":16,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":3935,"record_offset":3096,"parent_record_offset":2840,"ancestor_record_offset":2072,"inlinee":8269,"source_line":10,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":3935,"record_offset":2900,"parent_record_offset":2840,"ancestor_record_offset":2072,"inlinee":8262,"source_line":23,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxU32::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":3935,"record_offset":6900,"parent_record_offset":6876,"ancestor_record_offset":2072,"inlinee":8634,"source_line":153,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3935,"record_offset":3680,"parent_record_offset":3644,"ancestor_record_offset":2072,"inlinee":8278,"source_line":425,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3935,"record_offset":8136,"parent_record_offset":8104,"ancestor_record_offset":2072,"inlinee":8283,"source_line":601,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3935,"record_offset":8576,"parent_record_offset":8372,"ancestor_record_offset":2072,"inlinee":6607,"source_line":602,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":3935,"record_offset":3820,"parent_record_offset":3792,"ancestor_record_offset":2072,"inlinee":8280,"source_line":92,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3935,"record_offset":3892,"parent_record_offset":3752,"ancestor_record_offset":2072,"inlinee":6545,"source_line":429,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3935,"record_offset":4096,"parent_record_offset":3892,"ancestor_record_offset":2072,"inlinee":6548,"source_line":591,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":3935,"record_offset":3848,"parent_record_offset":3820,"ancestor_record_offset":2072,"inlinee":5707,"source_line":58,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":3935,"record_offset":3752,"parent_record_offset":3716,"ancestor_record_offset":2072,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/03935.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":3935,"record_offset":3716,"parent_record_offset":3680,"ancestor_record_offset":2072,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/03935.jsonl.gz"}
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
// 完整结构及原始类型记录: tree_animation.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: tree_animation.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: tree_animation.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 2; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::resources::tree_storage::InteractableForestTree>,ref_mut$<country_core::systems::prefab::PrefabInstanceData>,enum2$<core::option::Option<ref$<country_core::components::TreeMarkedForDeletion> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::prefab::DespawnPrefabCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::InteractableForestPlacements>)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::resources::tree_storage::InteractableForestTree>,ref_mut$<country_core::systems::prefab::PrefabInstanceData>,enum2$<core::option::Option<ref$<country_core::components::TreeMarkedForDeletion> > > >,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::message::message_writer::MessageWriter<country_core::systems::prefab::DespawnPrefabCmd>", "bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::InteractableForestPlacements>"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::resources::tree_storage::InteractableForestTree>,ref_mut$<country_core::systems::prefab::PrefabInstanceData>,enum2$<core::option::Option<ref$<country_core::components::TreeMarkedForDeletion> > > >,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::prefab::DespawnPrefabCmd>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::InteractableForestPlacements>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::TreeAnimation>,ref_mut$<country_core::systems::prefab::PrefabInstanceData>,enum2$<core::option::Option<ref$<country_core::components::TreeMarkedForDeletion> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::history::tree_edit::TreeCanopyMaxCullDistance>)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::TreeAnimation>,ref_mut$<country_core::systems::prefab::PrefabInstanceData>,enum2$<core::option::Option<ref$<country_core::components::TreeMarkedForDeletion> > > >,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::change_detection::ResMut<country_core::resources::history::tree_edit::TreeCanopyMaxCullDistance>"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::TreeAnimation>,ref_mut$<country_core::systems::prefab::PrefabInstanceData>,enum2$<core::option::Option<ref$<country_core::components::TreeMarkedForDeletion> > > >,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::tree_edit::TreeCanopyMaxCullDistance>

// ------------------------------------------------------------------
// 函数: forest::tree_animation::forest_tree_animation
// 信息 ID: 9e6dbd59e7ef4050b0413975
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 4f3db311ded877d925979f8d: void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::resources::tree_storage::InteractableForestTree>,ref_mut$<country_core::systems::prefab::PrefabInstanceData>,enum2$<core::option::Option<ref$<country_core::components::TreeMarkedForDeletion> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::prefab::DespawnPrefabCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::InteractableForestPlacements>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::resources::tree_storage::InteractableForestTree>,ref_mut$<country_core::systems::prefab::PrefabInstanceData>,enum2$<core::option::Option<ref$<country_core::components::TreeMarkedForDeletion> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::prefab::DespawnPrefabCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::InteractableForestPlacements>),tuple$<>,void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::resources::tree_storage::InteractableForestTree>,ref_mut$<country_core::systems::prefab::PrefabInstanceData>,enum2$<core::option::Option<ref$<country_core::components::TreeMarkedForDeletion> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::prefab::DespawnPrefabCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::InteractableForestPlacements>)> @ 0x20b5aaf -> 0x286dda0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::resources::tree_storage::InteractableForestTree>,ref_mut$<country_core::systems::prefab::PrefabInstanceData>,enum2$<core::option::Option<ref$<country_core::components::TreeMarkedForDeletion> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::prefab::DespawnPrefabCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::InteractableForestPlacements>),tuple$<>,void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::resources::tree_storage::InteractableForestTree>,ref_mut$<country_core::systems::prefab::PrefabInstanceData>,enum2$<core::option::Option<ref$<country_core::components::TreeMarkedForDeletion> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::prefab::DespawnPrefabCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::InteractableForestPlacements>)> > @ 0x20b99fd -> 0x286dda0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: forest::tree_animation::handplaced_tree_animation
// 信息 ID: 987a0045f463bac3fa2af72d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 154a1c8c75146f343d5796ba: void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::TreeAnimation>,ref_mut$<country_core::systems::prefab::PrefabInstanceData>,enum2$<core::option::Option<ref$<country_core::components::TreeMarkedForDeletion> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::history::tree_edit::TreeCanopyMaxCullDistance>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::TreeAnimation>,ref_mut$<country_core::systems::prefab::PrefabInstanceData>,enum2$<core::option::Option<ref$<country_core::components::TreeMarkedForDeletion> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::history::tree_edit::TreeCanopyMaxCullDistance>),tuple$<>,void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::TreeAnimation>,ref_mut$<country_core::systems::prefab::PrefabInstanceData>,enum2$<core::option::Option<ref$<country_core::components::TreeMarkedForDeletion> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::history::tree_edit::TreeCanopyMaxCullDistance>)> @ 0x20b5eec -> 0x286e600 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::TreeAnimation>,ref_mut$<country_core::systems::prefab::PrefabInstanceData>,enum2$<core::option::Option<ref$<country_core::components::TreeMarkedForDeletion> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::history::tree_edit::TreeCanopyMaxCullDistance>),tuple$<>,void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::TreeAnimation>,ref_mut$<country_core::systems::prefab::PrefabInstanceData>,enum2$<core::option::Option<ref$<country_core::components::TreeMarkedForDeletion> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::history::tree_edit::TreeCanopyMaxCullDistance>)> > @ 0x20b8a3e -> 0x286e600 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: forest::tree_animation::handplaced_tree_animation::closure$0
// 信息 ID: f0e521108a3ad20b9cd6be8a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "forest::tree_animation::handplaced_tree_animation", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] forest::tree_animation::handplaced_tree_animation; module=3935, inline_record=12124; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: forest::tree_animation::should_remove_tree_canopy_from_view
// 信息 ID: a3bc9e1b73d2af60f0fe0134
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] forest::tree_animation::handplaced_tree_animation; module=3935, inline_record=10648; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===

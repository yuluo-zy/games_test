// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/systems/critters/src/butterfly.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: c0611c9f440879d5f2d919cc065d1c685354ba5e6f0b507c09d7a66e593abd5a
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: butterfly.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 2836, 2858
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
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.30.10"}]
// use nonmax; // namespace candidate
//   package identities: [{"name":"nonmax","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.5.5"}]
// use perchance; // namespace candidate
//   package identities: [{"name":"perchance","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.0"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.1"}]
// use std; // namespace candidate
//   package identities: []
// use system_critters; // namespace candidate
//   package identities: [{"name":"system-critters","source":"local_or_path","version":"0.1.0"}]
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_critters::butterfly::update_butterfly_states
// 记录形式: compiled_procedure; instances=1
// 源码行: 43
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2836,"record_offset":14816,"rva":36250192,"va":5404959312}
// 编译实例 RVAs: 0x2292250
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-29284552","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404959312,"raw_hex":"e8382741fe","size":5,"soft_fail":false,"target_va":5375678704,"va":5404963251,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406ca440
//     owner=unresolved_namespace; count=1
//     target_aliases=77 candidates; complete list: butterfly.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-29130963","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404959312,"raw_hex":"e82d7f43fe","size":5,"soft_fail":false,"target_va":5375829056,"va":5404960014,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c714c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: butterfly.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23206699","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404959312,"raw_hex":"e8d5e49dfe","size":5,"soft_fail":false,"target_va":5381756096,"va":5404962790,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c8c720
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: butterfly.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23095541","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404959312,"raw_hex":"e80b979ffe","size":5,"soft_fail":false,"target_va":5381867296,"va":5404962832,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1422bdf80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: butterfly.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t178582","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404959312,"raw_hex":"e896b90200","size":5,"soft_fail":false,"target_va":5405138816,"va":5404960229,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142927b50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: butterfly.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6903441","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404959312,"raw_hex":"e891566900","size":5,"soft_fail":false,"target_va":5411863376,"va":5404959930,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1be30
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: butterfly.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12097314","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404959312,"raw_hex":"e82297b800","size":5,"soft_fail":false,"target_va":5417057840,"va":5404960521,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::query::iter::QueryIterationCursor$LT$D$C$F$GT$::init
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t6907499","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404959312,"raw_hex":"e86b666900","size":5,"soft_fail":false,"target_va":5411867776,"va":5404960272,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t11657966","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404959312,"raw_hex":"e8eee2b100","size":5,"soft_fail":false,"target_va":5416621408,"va":5404963437,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t11658087","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404959312,"raw_hex":"e867e3b100","size":5,"soft_fail":false,"target_va":5416621600,"va":5404963508,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t6948","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404959312,"raw_hex":"e8241b0000","size":5,"soft_fail":false,"target_va":5404966416,"va":5404959463,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t11657825","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404959312,"raw_hex":"e861e2b100","size":5,"soft_fail":false,"target_va":5416621376,"va":5404963546,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast::position
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-23986039","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404959312,"raw_hex":"e8890092fe","size":5,"soft_fail":false,"target_va":5380974048,"va":5404960082,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::camera_rig::MainCamera::eye_position
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-25644062","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404959312,"raw_hex":"e8e2b378fe","size":5,"soft_fail":false,"target_va":5379317488,"va":5404961545,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::camera_rig::MainCamera::mode
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-25635561","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404959312,"raw_hex":"e817d578fe","size":5,"soft_fail":false,"target_va":5379325968,"va":5404961524,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-20996038","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404959312,"raw_hex":"e83aa0bffe","size":5,"soft_fail":false,"target_va":5383963792,"va":5404959825,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::__RUST_STD_INTERNAL_VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-20987531","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404959312,"raw_hex":"e875c1bffe","size":5,"soft_fail":false,"target_va":5383972224,"va":5404959750,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-20998409","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404959312,"raw_hex":"e8f796bffe","size":5,"soft_fail":false,"target_va":5383964912,"va":5404963316,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-20996629","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404959312,"raw_hex":"e8eb9dbffe","size":5,"soft_fail":false,"target_va":5383962816,"va":5404959440,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::native::lazy::Storage$LT$T$C$D$GT$::get_or_init_slow
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t12073812","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404959312,"raw_hex":"e8543bb800","size":5,"soft_fail":false,"target_va":5417037264,"va":5404963447,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::LocalKey$LT$T$GT$::with
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t6903423","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404959312,"raw_hex":"e87f566900","size":5,"soft_fail":false,"target_va":5411867888,"va":5404964460,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t11655866","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404959312,"raw_hex":"e8badab100","size":5,"soft_fail":false,"target_va":5416619360,"va":5404963489,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*%rax","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5404959312,"raw_hex":"ffd0","size":2,"soft_fail":false,"va":5404959963,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":2836,"record_offset":17264,"parent_record_offset":14816,"ancestor_record_offset":14816,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$58::deref_mut
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":2836,"record_offset":18352,"parent_record_offset":14816,"ancestor_record_offset":14816,"inlinee":9404,"source_line":434,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::query::iter::impl$1::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":2836,"record_offset":18464,"parent_record_offset":14816,"ancestor_record_offset":14816,"inlinee":10317,"source_line":909,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::iter
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":2836,"record_offset":17476,"parent_record_offset":14816,"ancestor_record_offset":14816,"inlinee":10312,"source_line":673,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::iter_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":2836,"record_offset":18080,"parent_record_offset":14816,"ancestor_record_offset":14816,"inlinee":18903,"source_line":704,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::abs
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":2836,"record_offset":24376,"parent_record_offset":14816,"ancestor_record_offset":14816,"inlinee":11731,"source_line":1569,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":2836,"record_offset":24236,"parent_record_offset":14816,"ancestor_record_offset":14816,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":2836,"record_offset":25292,"parent_record_offset":14816,"ancestor_record_offset":14816,"inlinee":5478,"source_line":1018,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":2836,"record_offset":18256,"parent_record_offset":14816,"ancestor_record_offset":14816,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":2836,"record_offset":18024,"parent_record_offset":14816,"ancestor_record_offset":14816,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":2836,"record_offset":17960,"parent_record_offset":14816,"ancestor_record_offset":14816,"inlinee":7311,"source_line":789,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_direct] core::ops::range::Range::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":2836,"record_offset":24404,"parent_record_offset":14816,"ancestor_record_offset":14816,"inlinee":11437,"source_line":123,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2836,"record_offset":25320,"parent_record_offset":14816,"ancestor_record_offset":14816,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2836,"record_offset":17384,"parent_record_offset":14816,"ancestor_record_offset":14816,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2836,"record_offset":25932,"parent_record_offset":14816,"ancestor_record_offset":14816,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":2836,"record_offset":23964,"parent_record_offset":14816,"ancestor_record_offset":14816,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":2836,"record_offset":17324,"parent_record_offset":14816,"ancestor_record_offset":14816,"inlinee":8075,"source_line":281,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::ssbo::impl$6::mapped_data
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":2836,"record_offset":17420,"parent_record_offset":14816,"ancestor_record_offset":14816,"inlinee":30004,"source_line":310,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::world_raster::WorldRaster::as_float_grid
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/world_raster.rs
//     evidence={"module":2836,"record_offset":17448,"parent_record_offset":14816,"ancestor_record_offset":14816,"inlinee":27618,"source_line":188,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":2836,"record_offset":23716,"parent_record_offset":14816,"ancestor_record_offset":14816,"inlinee":11325,"source_line":487,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::length_squared
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":2836,"record_offset":23908,"parent_record_offset":14816,"ancestor_record_offset":14816,"inlinee":26151,"source_line":471,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":2836,"record_offset":18288,"parent_record_offset":14816,"ancestor_record_offset":14816,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_direct] perchance::PerchanceContext::uniform_range_f32
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"perchance","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/perchance-0.4.0/src/lib.rs
//     evidence={"module":2836,"record_offset":24572,"parent_record_offset":14816,"ancestor_record_offset":14816,"inlinee":31957,"source_line":270,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":2836,"record_offset":16456,"parent_record_offset":14816,"ancestor_record_offset":14816,"inlinee":8276,"source_line":589,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":2836,"record_offset":15052,"parent_record_offset":14816,"ancestor_record_offset":14816,"inlinee":9267,"source_line":626,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":2836,"record_offset":15444,"parent_record_offset":14816,"ancestor_record_offset":14816,"inlinee":8242,"source_line":659,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2836,"record_offset":26120,"parent_record_offset":26096,"ancestor_record_offset":14816,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2836,"record_offset":26144,"parent_record_offset":26120,"ancestor_record_offset":14816,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2836,"record_offset":26168,"parent_record_offset":26144,"ancestor_record_offset":14816,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2836,"record_offset":26096,"parent_record_offset":26004,"ancestor_record_offset":14816,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2836,"record_offset":17764,"parent_record_offset":17740,"ancestor_record_offset":14816,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2836,"record_offset":26032,"parent_record_offset":26004,"ancestor_record_offset":14816,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2836,"record_offset":26004,"parent_record_offset":25980,"ancestor_record_offset":14816,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2836,"record_offset":17812,"parent_record_offset":17788,"ancestor_record_offset":14816,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2836,"record_offset":17788,"parent_record_offset":17764,"ancestor_record_offset":14816,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2836,"record_offset":25980,"parent_record_offset":25956,"ancestor_record_offset":14816,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2836,"record_offset":17740,"parent_record_offset":17716,"ancestor_record_offset":14816,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2836,"record_offset":17716,"parent_record_offset":17684,"ancestor_record_offset":14816,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2836,"record_offset":18768,"parent_record_offset":18744,"ancestor_record_offset":14816,"inlinee":5735,"source_line":3021,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2836,"record_offset":18792,"parent_record_offset":18768,"ancestor_record_offset":14816,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2836,"record_offset":18048,"parent_record_offset":18024,"ancestor_record_offset":14816,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2836,"record_offset":17684,"parent_record_offset":17652,"ancestor_record_offset":14816,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::entities
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":2836,"record_offset":22652,"parent_record_offset":18488,"ancestor_record_offset":14816,"inlinee":10334,"source_line":484,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":2836,"record_offset":18976,"parent_record_offset":18488,"ancestor_record_offset":14816,"inlinee":10331,"source_line":655,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::table_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":2836,"record_offset":20748,"parent_record_offset":18488,"ancestor_record_offset":14816,"inlinee":8568,"source_line":478,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::ArchetypeEntity::table_row
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":2836,"record_offset":22904,"parent_record_offset":18488,"ancestor_record_offset":14816,"inlinee":10337,"source_line":346,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetypes::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":2836,"record_offset":18892,"parent_record_offset":18488,"ancestor_record_offset":14816,"inlinee":10330,"source_line":864,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":2836,"record_offset":17292,"parent_record_offset":17264,"ancestor_record_offset":14816,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$57::set_changed
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":2836,"record_offset":18376,"parent_record_offset":18352,"ancestor_record_offset":14816,"inlinee":9399,"source_line":397,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::StorageSwitch::extract
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":2836,"record_offset":22980,"parent_record_offset":22956,"ancestor_record_offset":14816,"inlinee":9590,"source_line":2964,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_archetype
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":2836,"record_offset":20872,"parent_record_offset":20840,"ancestor_record_offset":14816,"inlinee":10333,"source_line":1530,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_table
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":2836,"record_offset":19092,"parent_record_offset":19060,"ancestor_record_offset":14816,"inlinee":10322,"source_line":1545,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":2836,"record_offset":22956,"parent_record_offset":22932,"ancestor_record_offset":14816,"inlinee":10341,"source_line":1598,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":2836,"record_offset":23004,"parent_record_offset":22980,"ancestor_record_offset":14816,"inlinee":9575,"source_line":1605,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$48::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":2836,"record_offset":21964,"parent_record_offset":20840,"ancestor_record_offset":14816,"inlinee":11800,"source_line":1912,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$48::set_table
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":2836,"record_offset":20124,"parent_record_offset":19060,"ancestor_record_offset":14816,"inlinee":11801,"source_line":1927,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$49::fetch::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":2836,"record_offset":23436,"parent_record_offset":23408,"ancestor_record_offset":14816,"inlinee":9577,"source_line":1994,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$98::fetch
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":2836,"record_offset":22932,"parent_record_offset":18488,"ancestor_record_offset":14816,"inlinee":28005,"source_line":2520,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":2836,"record_offset":17620,"parent_record_offset":17504,"ancestor_record_offset":14816,"inlinee":10316,"source_line":40,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::init
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":2836,"record_offset":17652,"parent_record_offset":17620,"ancestor_record_offset":14816,"inlinee":9124,"source_line":2408,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":2836,"record_offset":18488,"parent_record_offset":18464,"ancestor_record_offset":14816,"inlinee":10318,"source_line":2511,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$5::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/world_query.rs
//     evidence={"module":2836,"record_offset":20840,"parent_record_offset":18488,"ancestor_record_offset":14816,"inlinee":28008,"source_line":182,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$5::set_table
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/world_query.rs
//     evidence={"module":2836,"record_offset":19060,"parent_record_offset":18488,"ancestor_record_offset":14816,"inlinee":28007,"source_line":195,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_array::BlobArray::get_sub_slice
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_array.rs
//     evidence={"module":2836,"record_offset":19548,"parent_record_offset":19524,"ancestor_record_offset":14816,"inlinee":10327,"source_line":143,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":2836,"record_offset":19192,"parent_record_offset":19168,"ancestor_record_offset":14816,"inlinee":8615,"source_line":56,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":2836,"record_offset":19168,"parent_record_offset":19144,"ancestor_record_offset":14816,"inlinee":8626,"source_line":405,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":2836,"record_offset":19372,"parent_record_offset":19348,"ancestor_record_offset":14816,"inlinee":8628,"source_line":406,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":2836,"record_offset":19144,"parent_record_offset":19116,"ancestor_record_offset":14816,"inlinee":8625,"source_line":489,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_data_slice_for
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":2836,"record_offset":19116,"parent_record_offset":19092,"ancestor_record_offset":14816,"inlinee":10323,"source_line":377,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":2836,"record_offset":18744,"parent_record_offset":18488,"ancestor_record_offset":14816,"inlinee":10320,"source_line":646,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":2836,"record_offset":23028,"parent_record_offset":23004,"ancestor_record_offset":14816,"inlinee":8632,"source_line":114,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":2836,"record_offset":18660,"parent_record_offset":18488,"ancestor_record_offset":14816,"inlinee":10319,"source_line":752,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_data_slice
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/column.rs
//     evidence={"module":2836,"record_offset":19524,"parent_record_offset":19500,"ancestor_record_offset":14816,"inlinee":10326,"source_line":297,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::impl$4::get_data_slice_for::closure$0
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":2836,"record_offset":19500,"parent_record_offset":19476,"ancestor_record_offset":14816,"inlinee":10325,"source_line":382,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::as_readonly
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":2836,"record_offset":17504,"parent_record_offset":17476,"ancestor_record_offset":14816,"inlinee":11849,"source_line":545,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::copy_unsafe
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":2836,"record_offset":17560,"parent_record_offset":17536,"ancestor_record_offset":14816,"inlinee":11851,"source_line":639,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":2836,"record_offset":17584,"parent_record_offset":17560,"ancestor_record_offset":14816,"inlinee":11852,"source_line":522,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::reborrow
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":2836,"record_offset":18104,"parent_record_offset":18080,"ancestor_record_offset":14816,"inlinee":18904,"source_line":604,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::reborrow_unsafe
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":2836,"record_offset":17536,"parent_record_offset":17504,"ancestor_record_offset":14816,"inlinee":11850,"source_line":621,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::archetypes
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":2836,"record_offset":17920,"parent_record_offset":17620,"ancestor_record_offset":14816,"inlinee":11874,"source_line":264,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ptr","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.17.3/src/lib.rs
//     evidence={"module":2836,"record_offset":23224,"parent_record_offset":23196,"ancestor_record_offset":14816,"inlinee":9579,"source_line":1073,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":2836,"record_offset":16784,"parent_record_offset":16756,"ancestor_record_offset":14816,"inlinee":6507,"source_line":2061,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":2836,"record_offset":16812,"parent_record_offset":16784,"ancestor_record_offset":14816,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":2836,"record_offset":16732,"parent_record_offset":16704,"ancestor_record_offset":14816,"inlinee":6546,"source_line":1221,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":2836,"record_offset":16756,"parent_record_offset":16732,"ancestor_record_offset":14816,"inlinee":6505,"source_line":1255,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":2836,"record_offset":16872,"parent_record_offset":16756,"ancestor_record_offset":14816,"inlinee":5177,"source_line":2443,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$52::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":2836,"record_offset":16984,"parent_record_offset":16960,"ancestor_record_offset":14816,"inlinee":4995,"source_line":2052,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":2836,"record_offset":24452,"parent_record_offset":24428,"ancestor_record_offset":14816,"inlinee":6833,"source_line":2145,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":2836,"record_offset":24476,"parent_record_offset":24452,"ancestor_record_offset":14816,"inlinee":8914,"source_line":1918,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":2836,"record_offset":23864,"parent_record_offset":23840,"ancestor_record_offset":14816,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::rotate_right
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":2836,"record_offset":24792,"parent_record_offset":24768,"ancestor_record_offset":14816,"inlinee":12398,"source_line":2047,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":2836,"record_offset":17988,"parent_record_offset":17960,"ancestor_record_offset":14816,"inlinee":7313,"source_line":68,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":2836,"record_offset":16836,"parent_record_offset":16812,"ancestor_record_offset":14816,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$10::wrapping_mul
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":2836,"record_offset":24740,"parent_record_offset":24704,"ancestor_record_offset":14816,"inlinee":6064,"source_line":2532,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":2836,"record_offset":26064,"parent_record_offset":26032,"ancestor_record_offset":14816,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::rotate_right
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":2836,"record_offset":24768,"parent_record_offset":24704,"ancestor_record_offset":14816,"inlinee":12397,"source_line":414,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":2836,"record_offset":18316,"parent_record_offset":18288,"ancestor_record_offset":14816,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":2836,"record_offset":16604,"parent_record_offset":16564,"ancestor_record_offset":14816,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":2836,"record_offset":24428,"parent_record_offset":24404,"ancestor_record_offset":14816,"inlinee":6826,"source_line":868,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2836,"record_offset":19308,"parent_record_offset":19284,"ancestor_record_offset":14816,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2836,"record_offset":19348,"parent_record_offset":19168,"ancestor_record_offset":14816,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":2836,"record_offset":15392,"parent_record_offset":15364,"ancestor_record_offset":14816,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":2836,"record_offset":17220,"parent_record_offset":16564,"ancestor_record_offset":14816,"inlinee":6443,"source_line":266,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2836,"record_offset":16936,"parent_record_offset":16704,"ancestor_record_offset":14816,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2836,"record_offset":17880,"parent_record_offset":17856,"ancestor_record_offset":14816,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":2836,"record_offset":15864,"parent_record_offset":15840,"ancestor_record_offset":14816,"inlinee":8268,"source_line":575,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":2836,"record_offset":15840,"parent_record_offset":15816,"ancestor_record_offset":14816,"inlinee":8267,"source_line":728,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":2836,"record_offset":16048,"parent_record_offset":16020,"ancestor_record_offset":14816,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$49::slice_contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":2836,"record_offset":16148,"parent_record_offset":16120,"ancestor_record_offset":14816,"inlinee":8260,"source_line":405,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":2836,"record_offset":16120,"parent_record_offset":16092,"ancestor_record_offset":14816,"inlinee":8258,"source_line":2589,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":2836,"record_offset":18684,"parent_record_offset":18660,"ancestor_record_offset":14816,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":2836,"record_offset":22848,"parent_record_offset":18488,"ancestor_record_offset":14816,"inlinee":4887,"source_line":639,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":2836,"record_offset":18708,"parent_record_offset":18684,"ancestor_record_offset":14816,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":2836,"record_offset":19428,"parent_record_offset":19396,"ancestor_record_offset":14816,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":2836,"record_offset":17856,"parent_record_offset":17684,"ancestor_record_offset":14816,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":2836,"record_offset":15792,"parent_record_offset":15768,"ancestor_record_offset":14816,"inlinee":8265,"source_line":81,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":2836,"record_offset":15816,"parent_record_offset":15792,"ancestor_record_offset":14816,"inlinee":8266,"source_line":119,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":2836,"record_offset":16176,"parent_record_offset":16148,"ancestor_record_offset":14816,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":2836,"record_offset":18596,"parent_record_offset":18488,"ancestor_record_offset":14816,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$172::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":2836,"record_offset":15768,"parent_record_offset":15652,"ancestor_record_offset":14816,"inlinee":8264,"source_line":438,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":2836,"record_offset":15180,"parent_record_offset":15156,"ancestor_record_offset":14816,"inlinee":9253,"source_line":377,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":2836,"record_offset":15480,"parent_record_offset":15444,"ancestor_record_offset":14816,"inlinee":8243,"source_line":1527,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":2836,"record_offset":15620,"parent_record_offset":15588,"ancestor_record_offset":14816,"inlinee":8253,"source_line":125,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":2836,"record_offset":15588,"parent_record_offset":15552,"ancestor_record_offset":14816,"inlinee":8251,"source_line":215,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":2836,"record_offset":15528,"parent_record_offset":15504,"ancestor_record_offset":14816,"inlinee":8247,"source_line":315,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":2836,"record_offset":15552,"parent_record_offset":15528,"ancestor_record_offset":14816,"inlinee":8249,"source_line":709,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":2836,"record_offset":15504,"parent_record_offset":15480,"ancestor_record_offset":14816,"inlinee":8245,"source_line":798,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":2836,"record_offset":16092,"parent_record_offset":15552,"ancestor_record_offset":14816,"inlinee":8257,"source_line":659,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":2836,"record_offset":15156,"parent_record_offset":15124,"ancestor_record_offset":14816,"inlinee":9270,"source_line":438,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":2836,"record_offset":15124,"parent_record_offset":15096,"ancestor_record_offset":14816,"inlinee":9269,"source_line":468,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":2836,"record_offset":15284,"parent_record_offset":15252,"ancestor_record_offset":14816,"inlinee":9252,"source_line":507,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":2836,"record_offset":15340,"parent_record_offset":15284,"ancestor_record_offset":14816,"inlinee":9254,"source_line":527,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":2836,"record_offset":15252,"parent_record_offset":15220,"ancestor_record_offset":14816,"inlinee":8274,"source_line":539,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":2836,"record_offset":15096,"parent_record_offset":15052,"ancestor_record_offset":14816,"inlinee":8272,"source_line":62,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":2836,"record_offset":15364,"parent_record_offset":15340,"ancestor_record_offset":14816,"inlinee":8315,"source_line":189,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":2836,"record_offset":15652,"parent_record_offset":15620,"ancestor_record_offset":14816,"inlinee":8255,"source_line":78,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":2836,"record_offset":15936,"parent_record_offset":15652,"ancestor_record_offset":14816,"inlinee":8270,"source_line":16,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":2836,"record_offset":15908,"parent_record_offset":15652,"ancestor_record_offset":14816,"inlinee":8269,"source_line":10,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":2836,"record_offset":15712,"parent_record_offset":15652,"ancestor_record_offset":14816,"inlinee":8262,"source_line":23,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":2836,"record_offset":17348,"parent_record_offset":17324,"ancestor_record_offset":14816,"inlinee":8076,"source_line":245,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":2836,"record_offset":23808,"parent_record_offset":23744,"ancestor_record_offset":14816,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_squared
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":2836,"record_offset":24100,"parent_record_offset":24004,"ancestor_record_offset":14816,"inlinee":26151,"source_line":471,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":2836,"record_offset":23744,"parent_record_offset":23716,"ancestor_record_offset":14816,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxU32::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":2836,"record_offset":23052,"parent_record_offset":23028,"ancestor_record_offset":14816,"inlinee":8634,"source_line":153,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":2836,"record_offset":19396,"parent_record_offset":19372,"ancestor_record_offset":14816,"inlinee":4193,"source_line":153,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] perchance::PerchanceContext::get_u32
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"perchance","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/perchance-0.4.0/src/lib.rs
//     evidence={"module":2836,"record_offset":24644,"parent_record_offset":24620,"ancestor_record_offset":14816,"inlinee":31960,"source_line":154,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] perchance::PerchanceContext::uniform_f32
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"perchance","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/perchance-0.4.0/src/lib.rs
//     evidence={"module":2836,"record_offset":24620,"parent_record_offset":24596,"ancestor_record_offset":14816,"inlinee":31959,"source_line":250,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] perchance::PerchanceContext::uniform_range_f32_raw
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"perchance","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/perchance-0.4.0/src/lib.rs
//     evidence={"module":2836,"record_offset":24596,"parent_record_offset":24572,"ancestor_record_offset":14816,"inlinee":31958,"source_line":275,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] perchance::Rng::next_u32
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"perchance","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/perchance-0.4.0/src/lib.rs
//     evidence={"module":2836,"record_offset":24680,"parent_record_offset":24644,"ancestor_record_offset":14816,"inlinee":31962,"source_line":72,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] perchance::Rng::next_u64
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"perchance","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/perchance-0.4.0/src/lib.rs
//     evidence={"module":2836,"record_offset":24704,"parent_record_offset":24680,"ancestor_record_offset":14816,"inlinee":31963,"source_line":77,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":2836,"record_offset":16492,"parent_record_offset":16456,"ancestor_record_offset":14816,"inlinee":8278,"source_line":425,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":2836,"record_offset":26288,"parent_record_offset":26256,"ancestor_record_offset":14816,"inlinee":8283,"source_line":601,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":2836,"record_offset":26740,"parent_record_offset":26536,"ancestor_record_offset":14816,"inlinee":6607,"source_line":602,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":2836,"record_offset":16632,"parent_record_offset":16604,"ancestor_record_offset":14816,"inlinee":8280,"source_line":92,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":2836,"record_offset":16704,"parent_record_offset":16564,"ancestor_record_offset":14816,"inlinee":6545,"source_line":429,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":2836,"record_offset":16908,"parent_record_offset":16704,"ancestor_record_offset":14816,"inlinee":6548,"source_line":591,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":2836,"record_offset":23840,"parent_record_offset":23808,"ancestor_record_offset":14816,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":2836,"record_offset":16660,"parent_record_offset":16632,"ancestor_record_offset":14816,"inlinee":5707,"source_line":58,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":2836,"record_offset":16564,"parent_record_offset":16528,"ancestor_record_offset":14816,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":2836,"record_offset":16528,"parent_record_offset":16492,"ancestor_record_offset":14816,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] system_critters::butterfly::update_butterfly_states::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-critters","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/critters/src/butterfly.rs
//     evidence={"module":2836,"record_offset":24004,"parent_record_offset":23964,"ancestor_record_offset":14816,"inlinee":111324,"source_line":106,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_critters::butterfly::update_butterfly_states::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 92
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1B31B
// 首条位置: {"module":2858,"inlinee":111387}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_critters::butterfly::update_butterfly_states::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 106
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1B2DC
// 首条位置: {"module":2836,"inlinee":111324}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::Vec2::length_squared
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":2836,"record_offset":24100,"parent_record_offset":24004,"ancestor_record_offset":24004,"inlinee":26151,"source_line":471,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":2836,"record_offset":24036,"parent_record_offset":24004,"ancestor_record_offset":24004,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":2836,"record_offset":24064,"parent_record_offset":24036,"ancestor_record_offset":24004,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":2836,"record_offset":24124,"parent_record_offset":24100,"ancestor_record_offset":24004,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_critters::butterfly::update_butterfly_goals
// 记录形式: compiled_procedure; instances=1
// 源码行: 256
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2836,"record_offset":4648,"rva":36247728,"va":5404956848}
// 编译实例 RVAs: 0x22918b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140df0600
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: butterfly.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21633233","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404956848,"raw_hex":"e82fe7b5fe","size":5,"soft_fail":false,"target_va":5383325184,"va":5404958412,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::uniform_sample_sphere
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-21679465","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404956848,"raw_hex":"e89732b5fe","size":5,"soft_fail":false,"target_va":5383278912,"va":5404958372,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":2836,"record_offset":4832,"parent_record_offset":4648,"ancestor_record_offset":4648,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$58::deref_mut
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":2836,"record_offset":5328,"parent_record_offset":4648,"ancestor_record_offset":4648,"inlinee":9404,"source_line":434,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::query::iter::impl$1::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":2836,"record_offset":5384,"parent_record_offset":4648,"ancestor_record_offset":4648,"inlinee":10317,"source_line":909,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::iter_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":2836,"record_offset":4888,"parent_record_offset":4648,"ancestor_record_offset":4648,"inlinee":18903,"source_line":704,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":2836,"record_offset":13116,"parent_record_offset":4648,"ancestor_record_offset":4648,"inlinee":5478,"source_line":1018,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_none_or
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2836,"record_offset":11356,"parent_record_offset":4648,"ancestor_record_offset":4648,"inlinee":7338,"source_line":710,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":2836,"record_offset":13144,"parent_record_offset":4648,"ancestor_record_offset":4648,"inlinee":9813,"source_line":461,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::normalize_or_zero
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":2836,"record_offset":12696,"parent_record_offset":4648,"ancestor_record_offset":4648,"inlinee":12105,"source_line":674,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$18::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":2836,"record_offset":12640,"parent_record_offset":4648,"ancestor_record_offset":4648,"inlinee":11813,"source_line":1449,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":2836,"record_offset":12348,"parent_record_offset":4648,"ancestor_record_offset":4648,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":2836,"record_offset":12540,"parent_record_offset":4648,"ancestor_record_offset":4648,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_direct] perchance::PerchanceContext::uniform_f32
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"perchance","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/perchance-0.4.0/src/lib.rs
//     evidence={"module":2836,"record_offset":11892,"parent_record_offset":4648,"ancestor_record_offset":4648,"inlinee":31959,"source_line":250,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_direct] perchance::PerchanceContext::uniform_range_f32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"perchance","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/perchance-0.4.0/src/lib.rs
//     evidence={"module":2836,"record_offset":11564,"parent_record_offset":4648,"ancestor_record_offset":4648,"inlinee":31957,"source_line":270,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2836,"record_offset":5160,"parent_record_offset":5136,"ancestor_record_offset":4648,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2836,"record_offset":5208,"parent_record_offset":5184,"ancestor_record_offset":4648,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2836,"record_offset":5184,"parent_record_offset":5160,"ancestor_record_offset":4648,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2836,"record_offset":5136,"parent_record_offset":5112,"ancestor_record_offset":4648,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2836,"record_offset":5112,"parent_record_offset":5080,"ancestor_record_offset":4648,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2836,"record_offset":5716,"parent_record_offset":5692,"ancestor_record_offset":4648,"inlinee":5735,"source_line":3021,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2836,"record_offset":5740,"parent_record_offset":5716,"ancestor_record_offset":4648,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2836,"record_offset":5080,"parent_record_offset":5052,"ancestor_record_offset":4648,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::entities
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":2836,"record_offset":10180,"parent_record_offset":5408,"ancestor_record_offset":4648,"inlinee":10334,"source_line":484,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":2836,"record_offset":7916,"parent_record_offset":5408,"ancestor_record_offset":4648,"inlinee":10331,"source_line":655,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::table_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":2836,"record_offset":8000,"parent_record_offset":5408,"ancestor_record_offset":4648,"inlinee":8568,"source_line":478,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::ArchetypeEntity::table_row
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":2836,"record_offset":10432,"parent_record_offset":5408,"ancestor_record_offset":4648,"inlinee":10337,"source_line":346,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetypes::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":2836,"record_offset":7832,"parent_record_offset":5408,"ancestor_record_offset":4648,"inlinee":10330,"source_line":864,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":2836,"record_offset":4856,"parent_record_offset":4832,"ancestor_record_offset":4648,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$57::set_changed
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":2836,"record_offset":5352,"parent_record_offset":5328,"ancestor_record_offset":4648,"inlinee":9399,"source_line":397,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::StorageSwitch::extract
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":2836,"record_offset":10508,"parent_record_offset":10484,"ancestor_record_offset":4648,"inlinee":9590,"source_line":2964,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_archetype
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":2836,"record_offset":8124,"parent_record_offset":8092,"ancestor_record_offset":4648,"inlinee":10333,"source_line":1530,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_table
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":2836,"record_offset":5808,"parent_record_offset":5776,"ancestor_record_offset":4648,"inlinee":10322,"source_line":1545,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":2836,"record_offset":10484,"parent_record_offset":10460,"ancestor_record_offset":4648,"inlinee":10341,"source_line":1598,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":2836,"record_offset":10532,"parent_record_offset":10508,"ancestor_record_offset":4648,"inlinee":9575,"source_line":1605,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$48::set_archetype
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":2836,"record_offset":9216,"parent_record_offset":8092,"ancestor_record_offset":4648,"inlinee":11800,"source_line":1912,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$48::set_table
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":2836,"record_offset":6840,"parent_record_offset":5776,"ancestor_record_offset":4648,"inlinee":11801,"source_line":1927,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$49::fetch::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":2836,"record_offset":10960,"parent_record_offset":10932,"ancestor_record_offset":4648,"inlinee":9577,"source_line":1994,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$95::fetch
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":2836,"record_offset":10460,"parent_record_offset":5408,"ancestor_record_offset":4648,"inlinee":18908,"source_line":2520,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":2836,"record_offset":5028,"parent_record_offset":4912,"ancestor_record_offset":4648,"inlinee":10316,"source_line":40,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::init
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":2836,"record_offset":5052,"parent_record_offset":5028,"ancestor_record_offset":4648,"inlinee":9124,"source_line":2408,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":2836,"record_offset":5408,"parent_record_offset":5384,"ancestor_record_offset":4648,"inlinee":10318,"source_line":2511,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$4::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/world_query.rs
//     evidence={"module":2836,"record_offset":8092,"parent_record_offset":5408,"ancestor_record_offset":4648,"inlinee":18906,"source_line":182,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$4::set_table
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/world_query.rs
//     evidence={"module":2836,"record_offset":5776,"parent_record_offset":5408,"ancestor_record_offset":4648,"inlinee":18905,"source_line":195,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_array::BlobArray::get_sub_slice
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_array.rs
//     evidence={"module":2836,"record_offset":6264,"parent_record_offset":6240,"ancestor_record_offset":4648,"inlinee":10327,"source_line":143,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":2836,"record_offset":5908,"parent_record_offset":5884,"ancestor_record_offset":4648,"inlinee":8615,"source_line":56,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":2836,"record_offset":5884,"parent_record_offset":5860,"ancestor_record_offset":4648,"inlinee":8626,"source_line":405,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":2836,"record_offset":6088,"parent_record_offset":6064,"ancestor_record_offset":4648,"inlinee":8628,"source_line":406,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":2836,"record_offset":5860,"parent_record_offset":5832,"ancestor_record_offset":4648,"inlinee":8625,"source_line":489,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_data_slice_for
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":2836,"record_offset":5832,"parent_record_offset":5808,"ancestor_record_offset":4648,"inlinee":10323,"source_line":377,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":2836,"record_offset":5692,"parent_record_offset":5408,"ancestor_record_offset":4648,"inlinee":10320,"source_line":646,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":2836,"record_offset":10556,"parent_record_offset":10532,"ancestor_record_offset":4648,"inlinee":8632,"source_line":114,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":2836,"record_offset":5608,"parent_record_offset":5408,"ancestor_record_offset":4648,"inlinee":10319,"source_line":752,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_changed_ticks_slice
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/column.rs
//     evidence={"module":2836,"record_offset":7240,"parent_record_offset":6840,"ancestor_record_offset":4648,"inlinee":11819,"source_line":313,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_data_slice
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/column.rs
//     evidence={"module":2836,"record_offset":6240,"parent_record_offset":6216,"ancestor_record_offset":4648,"inlinee":10326,"source_line":297,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::impl$4::get_data_slice_for::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":2836,"record_offset":6216,"parent_record_offset":6192,"ancestor_record_offset":4648,"inlinee":10325,"source_line":382,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::thin_array_ptr::ThinArrayPtr::as_slice
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/thin_array_ptr.rs
//     evidence={"module":2836,"record_offset":7264,"parent_record_offset":7240,"ancestor_record_offset":4648,"inlinee":11820,"source_line":300,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::copy_unsafe
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":2836,"record_offset":4968,"parent_record_offset":4944,"ancestor_record_offset":4648,"inlinee":11851,"source_line":639,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":2836,"record_offset":4992,"parent_record_offset":4968,"ancestor_record_offset":4648,"inlinee":11852,"source_line":522,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::reborrow
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":2836,"record_offset":4912,"parent_record_offset":4888,"ancestor_record_offset":4648,"inlinee":18904,"source_line":604,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::reborrow_unsafe
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":2836,"record_offset":4944,"parent_record_offset":4912,"ancestor_record_offset":4648,"inlinee":11850,"source_line":621,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ptr","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.17.3/src/lib.rs
//     evidence={"module":2836,"record_offset":10748,"parent_record_offset":10720,"ancestor_record_offset":4648,"inlinee":9579,"source_line":1073,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":2836,"record_offset":12880,"parent_record_offset":12856,"ancestor_record_offset":4648,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::rotate_right
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":2836,"record_offset":11780,"parent_record_offset":11756,"ancestor_record_offset":4648,"inlinee":12398,"source_line":2047,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$10::wrapping_mul
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":2836,"record_offset":11728,"parent_record_offset":11696,"ancestor_record_offset":4648,"inlinee":6064,"source_line":2532,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::rotate_right
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":2836,"record_offset":11756,"parent_record_offset":11696,"ancestor_record_offset":4648,"inlinee":12397,"source_line":414,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":2836,"record_offset":12576,"parent_record_offset":12540,"ancestor_record_offset":4648,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":2836,"record_offset":12392,"parent_record_offset":12348,"ancestor_record_offset":4648,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":2836,"record_offset":11432,"parent_record_offset":11404,"ancestor_record_offset":4648,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":2836,"record_offset":6000,"parent_record_offset":5936,"ancestor_record_offset":4648,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2836,"record_offset":6968,"parent_record_offset":6944,"ancestor_record_offset":4648,"inlinee":5449,"source_line":1541,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2836,"record_offset":6024,"parent_record_offset":6000,"ancestor_record_offset":4648,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2836,"record_offset":6064,"parent_record_offset":5884,"ancestor_record_offset":4648,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2836,"record_offset":5276,"parent_record_offset":5252,"ancestor_record_offset":4648,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":2836,"record_offset":5576,"parent_record_offset":5516,"ancestor_record_offset":4648,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":2836,"record_offset":5548,"parent_record_offset":5516,"ancestor_record_offset":4648,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":2836,"record_offset":5632,"parent_record_offset":5608,"ancestor_record_offset":4648,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":2836,"record_offset":10376,"parent_record_offset":5408,"ancestor_record_offset":4648,"inlinee":4887,"source_line":639,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":2836,"record_offset":5656,"parent_record_offset":5632,"ancestor_record_offset":4648,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":2836,"record_offset":6144,"parent_record_offset":6112,"ancestor_record_offset":4648,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":2836,"record_offset":5252,"parent_record_offset":5080,"ancestor_record_offset":4648,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":2836,"record_offset":5516,"parent_record_offset":5408,"ancestor_record_offset":4648,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":2836,"record_offset":13168,"parent_record_offset":13144,"ancestor_record_offset":4648,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":2836,"record_offset":11520,"parent_record_offset":11492,"ancestor_record_offset":4648,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":2836,"record_offset":12784,"parent_record_offset":12756,"ancestor_record_offset":4648,"inlinee":9776,"source_line":552,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":2836,"record_offset":12756,"parent_record_offset":12724,"ancestor_record_offset":4648,"inlinee":9828,"source_line":571,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_squared
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":2836,"record_offset":11492,"parent_record_offset":11380,"ancestor_record_offset":4648,"inlinee":26155,"source_line":562,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::normalize_or
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":2836,"record_offset":12724,"parent_record_offset":12696,"ancestor_record_offset":4648,"inlinee":12106,"source_line":657,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":2836,"record_offset":11404,"parent_record_offset":11380,"ancestor_record_offset":4648,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxU32::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":2836,"record_offset":10580,"parent_record_offset":10556,"ancestor_record_offset":4648,"inlinee":8634,"source_line":153,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":2836,"record_offset":6112,"parent_record_offset":6088,"ancestor_record_offset":4648,"inlinee":4193,"source_line":153,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] perchance::PerchanceContext::get_u32
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"perchance","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/perchance-0.4.0/src/lib.rs
//     evidence={"module":2836,"record_offset":11636,"parent_record_offset":11612,"ancestor_record_offset":4648,"inlinee":31960,"source_line":154,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] perchance::PerchanceContext::uniform_f32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"perchance","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/perchance-0.4.0/src/lib.rs
//     evidence={"module":2836,"record_offset":11612,"parent_record_offset":11588,"ancestor_record_offset":4648,"inlinee":31959,"source_line":250,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] perchance::PerchanceContext::uniform_range_f32_raw
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"perchance","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/perchance-0.4.0/src/lib.rs
//     evidence={"module":2836,"record_offset":11588,"parent_record_offset":11564,"ancestor_record_offset":4648,"inlinee":31958,"source_line":275,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] perchance::Rng::next_u32
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"perchance","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/perchance-0.4.0/src/lib.rs
//     evidence={"module":2836,"record_offset":11672,"parent_record_offset":11636,"ancestor_record_offset":4648,"inlinee":31962,"source_line":72,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] perchance::Rng::next_u64
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"perchance","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/perchance-0.4.0/src/lib.rs
//     evidence={"module":2836,"record_offset":11696,"parent_record_offset":11672,"ancestor_record_offset":4648,"inlinee":31963,"source_line":77,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":2836,"record_offset":12856,"parent_record_offset":12820,"ancestor_record_offset":4648,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] system_critters::butterfly::update_butterfly_goals::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-critters","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/critters/src/butterfly.rs
//     evidence={"module":2836,"record_offset":11380,"parent_record_offset":11356,"ancestor_record_offset":4648,"inlinee":111322,"source_line":277,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_critters::butterfly::update_butterfly_goals::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 277
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1B2DA
// 首条位置: {"module":2836,"inlinee":111322}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec3::Vec3::length_squared
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":2836,"record_offset":11492,"parent_record_offset":11380,"ancestor_record_offset":11380,"inlinee":26155,"source_line":562,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":2836,"record_offset":11404,"parent_record_offset":11380,"ancestor_record_offset":11380,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":2836,"record_offset":11432,"parent_record_offset":11404,"ancestor_record_offset":11380,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":2836,"record_offset":11520,"parent_record_offset":11492,"ancestor_record_offset":11380,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_critters::butterfly::set_butterfly_velocities_towards_goals
// 记录形式: compiled_procedure; instances=1
// 源码行: 334
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2836,"record_offset":27820,"rva":36255440,"va":5404964560}
// 编译实例 RVAs: 0x22936d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] bevy_ecs::query::iter::QueryIterationCursor$LT$D$C$F$GT$::init
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t6902662","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404964560,"raw_hex":"e886536900","size":5,"soft_fail":false,"target_va":5411867344,"va":5404964677,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$58::deref_mut
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":2836,"record_offset":35780,"parent_record_offset":27820,"ancestor_record_offset":27820,"inlinee":9404,"source_line":434,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::query::iter::impl$1::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":2836,"record_offset":28212,"parent_record_offset":27820,"ancestor_record_offset":27820,"inlinee":10317,"source_line":909,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::iter_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":2836,"record_offset":28000,"parent_record_offset":27820,"ancestor_record_offset":27820,"inlinee":18903,"source_line":704,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":2836,"record_offset":36004,"parent_record_offset":27820,"ancestor_record_offset":27820,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::normalize_or_zero
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":2836,"record_offset":35448,"parent_record_offset":27820,"ancestor_record_offset":27820,"inlinee":12105,"source_line":674,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":2836,"record_offset":35692,"parent_record_offset":27820,"ancestor_record_offset":27820,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$28::mul_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":2836,"record_offset":36200,"parent_record_offset":27820,"ancestor_record_offset":27820,"inlinee":11805,"source_line":1536,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":2836,"record_offset":35356,"parent_record_offset":27820,"ancestor_record_offset":27820,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2836,"record_offset":34220,"parent_record_offset":34196,"ancestor_record_offset":27820,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2836,"record_offset":34268,"parent_record_offset":34244,"ancestor_record_offset":27820,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2836,"record_offset":34244,"parent_record_offset":34220,"ancestor_record_offset":27820,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2836,"record_offset":34196,"parent_record_offset":34172,"ancestor_record_offset":27820,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2836,"record_offset":34172,"parent_record_offset":34148,"ancestor_record_offset":27820,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2836,"record_offset":28524,"parent_record_offset":28500,"ancestor_record_offset":27820,"inlinee":5735,"source_line":3021,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2836,"record_offset":28548,"parent_record_offset":28524,"ancestor_record_offset":27820,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2836,"record_offset":34148,"parent_record_offset":34124,"ancestor_record_offset":27820,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::entities
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":2836,"record_offset":34124,"parent_record_offset":28236,"ancestor_record_offset":27820,"inlinee":10334,"source_line":484,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":2836,"record_offset":28732,"parent_record_offset":28236,"ancestor_record_offset":27820,"inlinee":10331,"source_line":655,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::table_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":2836,"record_offset":31344,"parent_record_offset":28236,"ancestor_record_offset":27820,"inlinee":8568,"source_line":478,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::ArchetypeEntity::table_row
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":2836,"record_offset":34376,"parent_record_offset":28236,"ancestor_record_offset":27820,"inlinee":10337,"source_line":346,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetypes::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":2836,"record_offset":28648,"parent_record_offset":28236,"ancestor_record_offset":27820,"inlinee":10330,"source_line":864,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$57::set_changed
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":2836,"record_offset":35804,"parent_record_offset":35780,"ancestor_record_offset":27820,"inlinee":9399,"source_line":397,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::StorageSwitch::extract
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":2836,"record_offset":34452,"parent_record_offset":34428,"ancestor_record_offset":27820,"inlinee":9590,"source_line":2964,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_archetype
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":2836,"record_offset":31472,"parent_record_offset":31436,"ancestor_record_offset":27820,"inlinee":10333,"source_line":1530,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_table
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":2836,"record_offset":28852,"parent_record_offset":28816,"ancestor_record_offset":27820,"inlinee":10322,"source_line":1545,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":2836,"record_offset":34428,"parent_record_offset":34404,"ancestor_record_offset":27820,"inlinee":10341,"source_line":1598,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":2836,"record_offset":34476,"parent_record_offset":34452,"ancestor_record_offset":27820,"inlinee":9575,"source_line":1605,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$48::set_archetype
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":2836,"record_offset":33160,"parent_record_offset":31436,"ancestor_record_offset":27820,"inlinee":11800,"source_line":1912,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$48::set_table
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":2836,"record_offset":30444,"parent_record_offset":28816,"ancestor_record_offset":27820,"inlinee":11801,"source_line":1927,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$49::fetch::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":2836,"record_offset":34904,"parent_record_offset":34876,"ancestor_record_offset":27820,"inlinee":9577,"source_line":1994,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$98::fetch
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":2836,"record_offset":34404,"parent_record_offset":28236,"ancestor_record_offset":27820,"inlinee":28005,"source_line":2520,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":2836,"record_offset":28136,"parent_record_offset":28024,"ancestor_record_offset":27820,"inlinee":10316,"source_line":40,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":2836,"record_offset":28236,"parent_record_offset":28212,"ancestor_record_offset":27820,"inlinee":10318,"source_line":2511,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$5::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/world_query.rs
//     evidence={"module":2836,"record_offset":31436,"parent_record_offset":28236,"ancestor_record_offset":27820,"inlinee":28008,"source_line":182,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$5::set_table
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/world_query.rs
//     evidence={"module":2836,"record_offset":28816,"parent_record_offset":28236,"ancestor_record_offset":27820,"inlinee":28007,"source_line":195,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_array::BlobArray::get_sub_slice
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_array.rs
//     evidence={"module":2836,"record_offset":29308,"parent_record_offset":29284,"ancestor_record_offset":27820,"inlinee":10327,"source_line":143,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":2836,"record_offset":28952,"parent_record_offset":28928,"ancestor_record_offset":27820,"inlinee":8615,"source_line":56,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":2836,"record_offset":28928,"parent_record_offset":28904,"ancestor_record_offset":27820,"inlinee":8626,"source_line":405,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":2836,"record_offset":29132,"parent_record_offset":29108,"ancestor_record_offset":27820,"inlinee":8628,"source_line":406,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":2836,"record_offset":28904,"parent_record_offset":28876,"ancestor_record_offset":27820,"inlinee":8625,"source_line":489,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_data_slice_for
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":2836,"record_offset":28876,"parent_record_offset":28852,"ancestor_record_offset":27820,"inlinee":10323,"source_line":377,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":2836,"record_offset":28500,"parent_record_offset":28236,"ancestor_record_offset":27820,"inlinee":10320,"source_line":646,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":2836,"record_offset":34500,"parent_record_offset":34476,"ancestor_record_offset":27820,"inlinee":8632,"source_line":114,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":2836,"record_offset":28416,"parent_record_offset":28236,"ancestor_record_offset":27820,"inlinee":10319,"source_line":752,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_changed_ticks_slice
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/column.rs
//     evidence={"module":2836,"record_offset":30844,"parent_record_offset":30444,"ancestor_record_offset":27820,"inlinee":11819,"source_line":313,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_data_slice
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/column.rs
//     evidence={"module":2836,"record_offset":29284,"parent_record_offset":29260,"ancestor_record_offset":27820,"inlinee":10326,"source_line":297,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::impl$4::get_data_slice_for::closure$0
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":2836,"record_offset":29260,"parent_record_offset":29236,"ancestor_record_offset":27820,"inlinee":10325,"source_line":382,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::thin_array_ptr::ThinArrayPtr::as_slice
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/thin_array_ptr.rs
//     evidence={"module":2836,"record_offset":30868,"parent_record_offset":30844,"ancestor_record_offset":27820,"inlinee":11820,"source_line":300,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::copy_unsafe
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":2836,"record_offset":28076,"parent_record_offset":28052,"ancestor_record_offset":27820,"inlinee":11851,"source_line":639,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":2836,"record_offset":28100,"parent_record_offset":28076,"ancestor_record_offset":27820,"inlinee":11852,"source_line":522,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::reborrow
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":2836,"record_offset":28024,"parent_record_offset":28000,"ancestor_record_offset":27820,"inlinee":18904,"source_line":604,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::reborrow_unsafe
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":2836,"record_offset":28052,"parent_record_offset":28024,"ancestor_record_offset":27820,"inlinee":11850,"source_line":621,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::archetypes
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":2836,"record_offset":28172,"parent_record_offset":28136,"ancestor_record_offset":27820,"inlinee":11874,"source_line":264,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"bevy_ptr","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.17.3/src/lib.rs
//     evidence={"module":2836,"record_offset":34692,"parent_record_offset":34664,"ancestor_record_offset":27820,"inlinee":9579,"source_line":1073,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":2836,"record_offset":35640,"parent_record_offset":35616,"ancestor_record_offset":27820,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":2836,"record_offset":35720,"parent_record_offset":35692,"ancestor_record_offset":27820,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$417::mul_assign
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":2836,"record_offset":36228,"parent_record_offset":36200,"ancestor_record_offset":27820,"inlinee":11807,"source_line":924,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":2836,"record_offset":35388,"parent_record_offset":35356,"ancestor_record_offset":27820,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":2836,"record_offset":29044,"parent_record_offset":28980,"ancestor_record_offset":27820,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2836,"record_offset":30572,"parent_record_offset":30548,"ancestor_record_offset":27820,"inlinee":5449,"source_line":1541,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2836,"record_offset":29068,"parent_record_offset":29044,"ancestor_record_offset":27820,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2836,"record_offset":29108,"parent_record_offset":28928,"ancestor_record_offset":27820,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2836,"record_offset":34720,"parent_record_offset":34692,"ancestor_record_offset":27820,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":2836,"record_offset":28380,"parent_record_offset":28352,"ancestor_record_offset":27820,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":2836,"record_offset":28440,"parent_record_offset":28416,"ancestor_record_offset":27820,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":2836,"record_offset":34320,"parent_record_offset":28236,"ancestor_record_offset":27820,"inlinee":4887,"source_line":639,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":2836,"record_offset":28464,"parent_record_offset":28440,"ancestor_record_offset":27820,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":2836,"record_offset":29188,"parent_record_offset":29156,"ancestor_record_offset":27820,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":2836,"record_offset":28352,"parent_record_offset":28236,"ancestor_record_offset":27820,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":2836,"record_offset":35580,"parent_record_offset":35544,"ancestor_record_offset":27820,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":2836,"record_offset":35544,"parent_record_offset":35516,"ancestor_record_offset":27820,"inlinee":9776,"source_line":552,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":2836,"record_offset":35516,"parent_record_offset":35480,"ancestor_record_offset":27820,"inlinee":9828,"source_line":571,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::normalize_or
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":2836,"record_offset":35480,"parent_record_offset":35448,"ancestor_record_offset":27820,"inlinee":12106,"source_line":657,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxU32::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":2836,"record_offset":34524,"parent_record_offset":34500,"ancestor_record_offset":27820,"inlinee":8634,"source_line":153,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":2836,"record_offset":29156,"parent_record_offset":29132,"ancestor_record_offset":27820,"inlinee":4193,"source_line":153,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":2836,"record_offset":35616,"parent_record_offset":35580,"ancestor_record_offset":27820,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_critters::butterfly::update_butterfly_audio
// 记录形式: compiled_procedure; instances=1
// 源码行: 381
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2836,"record_offset":272,"rva":36247088,"va":5404956208}
// 编译实例 RVAs: 0x2291630
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] country_core::resources::critters::ButterflyAudio::set_position
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-17969771","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404956208,"raw_hex":"e895cdedfe","size":5,"soft_fail":false,"target_va":5386986576,"va":5404956342,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::query::iter::impl$1::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":2836,"record_offset":844,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":10317,"source_line":909,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::iter_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":2836,"record_offset":412,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":18903,"source_line":704,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2836,"record_offset":676,"parent_record_offset":652,"ancestor_record_offset":272,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2836,"record_offset":724,"parent_record_offset":700,"ancestor_record_offset":272,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2836,"record_offset":700,"parent_record_offset":676,"ancestor_record_offset":272,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2836,"record_offset":652,"parent_record_offset":628,"ancestor_record_offset":272,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2836,"record_offset":628,"parent_record_offset":596,"ancestor_record_offset":272,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2836,"record_offset":1472,"parent_record_offset":1448,"ancestor_record_offset":272,"inlinee":5735,"source_line":3021,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2836,"record_offset":1496,"parent_record_offset":1472,"ancestor_record_offset":272,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2836,"record_offset":596,"parent_record_offset":572,"ancestor_record_offset":272,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::entities
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":2836,"record_offset":2568,"parent_record_offset":876,"ancestor_record_offset":272,"inlinee":10334,"source_line":484,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":2836,"record_offset":3368,"parent_record_offset":876,"ancestor_record_offset":272,"inlinee":10331,"source_line":655,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::table_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":2836,"record_offset":3452,"parent_record_offset":876,"ancestor_record_offset":272,"inlinee":8568,"source_line":478,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::ArchetypeEntity::table_row
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":2836,"record_offset":2820,"parent_record_offset":876,"ancestor_record_offset":272,"inlinee":10337,"source_line":346,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetypes::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":2836,"record_offset":3284,"parent_record_offset":876,"ancestor_record_offset":272,"inlinee":10330,"source_line":864,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::StorageSwitch::extract
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":2836,"record_offset":1012,"parent_record_offset":988,"ancestor_record_offset":272,"inlinee":9590,"source_line":2964,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_archetype
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":2836,"record_offset":3568,"parent_record_offset":3544,"ancestor_record_offset":272,"inlinee":10333,"source_line":1530,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_table
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":2836,"record_offset":1556,"parent_record_offset":1532,"ancestor_record_offset":272,"inlinee":10322,"source_line":1545,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":2836,"record_offset":988,"parent_record_offset":964,"ancestor_record_offset":272,"inlinee":10341,"source_line":1598,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":2836,"record_offset":1036,"parent_record_offset":1012,"ancestor_record_offset":272,"inlinee":9575,"source_line":1605,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$89::fetch
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":2836,"record_offset":964,"parent_record_offset":876,"ancestor_record_offset":272,"inlinee":10339,"source_line":2520,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":2836,"record_offset":548,"parent_record_offset":436,"ancestor_record_offset":272,"inlinee":10316,"source_line":40,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::init
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":2836,"record_offset":572,"parent_record_offset":548,"ancestor_record_offset":272,"inlinee":9124,"source_line":2408,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":2836,"record_offset":876,"parent_record_offset":844,"ancestor_record_offset":272,"inlinee":10318,"source_line":2511,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$2::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/world_query.rs
//     evidence={"module":2836,"record_offset":3544,"parent_record_offset":876,"ancestor_record_offset":272,"inlinee":10332,"source_line":182,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$2::set_table
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/world_query.rs
//     evidence={"module":2836,"record_offset":1532,"parent_record_offset":876,"ancestor_record_offset":272,"inlinee":10321,"source_line":195,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_array::BlobArray::get_sub_slice
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_array.rs
//     evidence={"module":2836,"record_offset":2012,"parent_record_offset":1988,"ancestor_record_offset":272,"inlinee":10327,"source_line":143,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":2836,"record_offset":1656,"parent_record_offset":1632,"ancestor_record_offset":272,"inlinee":8615,"source_line":56,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":2836,"record_offset":1632,"parent_record_offset":1608,"ancestor_record_offset":272,"inlinee":8626,"source_line":405,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":2836,"record_offset":1836,"parent_record_offset":1812,"ancestor_record_offset":272,"inlinee":8628,"source_line":406,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":2836,"record_offset":1608,"parent_record_offset":1580,"ancestor_record_offset":272,"inlinee":8625,"source_line":489,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_data_slice_for
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":2836,"record_offset":1580,"parent_record_offset":1556,"ancestor_record_offset":272,"inlinee":10323,"source_line":377,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":2836,"record_offset":1448,"parent_record_offset":876,"ancestor_record_offset":272,"inlinee":10320,"source_line":646,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":2836,"record_offset":2944,"parent_record_offset":2920,"ancestor_record_offset":272,"inlinee":8632,"source_line":114,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":2836,"record_offset":1364,"parent_record_offset":876,"ancestor_record_offset":272,"inlinee":10319,"source_line":752,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_data_slice
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/column.rs
//     evidence={"module":2836,"record_offset":1988,"parent_record_offset":1964,"ancestor_record_offset":272,"inlinee":10326,"source_line":297,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::impl$4::get_data_slice_for::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":2836,"record_offset":1964,"parent_record_offset":1940,"ancestor_record_offset":272,"inlinee":10325,"source_line":382,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::copy_unsafe
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":2836,"record_offset":488,"parent_record_offset":464,"ancestor_record_offset":272,"inlinee":11851,"source_line":639,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":2836,"record_offset":512,"parent_record_offset":488,"ancestor_record_offset":272,"inlinee":11852,"source_line":522,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::reborrow
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":2836,"record_offset":436,"parent_record_offset":412,"ancestor_record_offset":272,"inlinee":18904,"source_line":604,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::reborrow_unsafe
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":2836,"record_offset":464,"parent_record_offset":436,"ancestor_record_offset":272,"inlinee":11850,"source_line":621,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ptr","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.17.3/src/lib.rs
//     evidence={"module":2836,"record_offset":1060,"parent_record_offset":1036,"ancestor_record_offset":272,"inlinee":9579,"source_line":1073,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":2836,"record_offset":1748,"parent_record_offset":1684,"ancestor_record_offset":272,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2836,"record_offset":1772,"parent_record_offset":1748,"ancestor_record_offset":272,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2836,"record_offset":1812,"parent_record_offset":1632,"ancestor_record_offset":272,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2836,"record_offset":792,"parent_record_offset":768,"ancestor_record_offset":272,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":2836,"record_offset":1332,"parent_record_offset":1272,"ancestor_record_offset":272,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":2836,"record_offset":1304,"parent_record_offset":1272,"ancestor_record_offset":272,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":2836,"record_offset":1388,"parent_record_offset":1364,"ancestor_record_offset":272,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":2836,"record_offset":2764,"parent_record_offset":876,"ancestor_record_offset":272,"inlinee":4887,"source_line":639,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":2836,"record_offset":1412,"parent_record_offset":1388,"ancestor_record_offset":272,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":2836,"record_offset":1892,"parent_record_offset":1860,"ancestor_record_offset":272,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":2836,"record_offset":768,"parent_record_offset":596,"ancestor_record_offset":272,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":2836,"record_offset":1272,"parent_record_offset":876,"ancestor_record_offset":272,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxU32::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":2836,"record_offset":2976,"parent_record_offset":2944,"ancestor_record_offset":272,"inlinee":8634,"source_line":153,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":2836,"record_offset":1860,"parent_record_offset":1836,"ancestor_record_offset":272,"inlinee":4193,"source_line":153,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_critters::butterfly::butterflies_send_to_gpu
// 记录形式: compiled_procedure; instances=1
// 源码行: 388
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2836,"record_offset":13444,"rva":36249840,"va":5404958960}
// 编译实例 RVAs: 0x22920f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-29280506","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404958960,"raw_hex":"e8063741fe","size":5,"soft_fail":false,"target_va":5375678704,"va":5404959205,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1422bd0f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: butterfly.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t175977","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404958960,"raw_hex":"e869af0200","size":5,"soft_fail":false,"target_va":5405135088,"va":5404959106,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142927b50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: butterfly.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6904235","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404958960,"raw_hex":"e8ab596900","size":5,"soft_fail":false,"target_va":5411863376,"va":5404959136,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142927e20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: butterfly.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6904915","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404958960,"raw_hex":"e8535c6900","size":5,"soft_fail":false,"target_va":5411864096,"va":5404959176,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t11662963","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404958960,"raw_hex":"e873f6b100","size":5,"soft_fail":false,"target_va":5416622208,"va":5404959240,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*24(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5404958960,"raw_hex":"ff5018","size":3,"soft_fail":false,"va":5404959154,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::iter
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":2836,"record_offset":13592,"parent_record_offset":13444,"ancestor_record_offset":13444,"inlinee":10312,"source_line":673,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":2836,"record_offset":14072,"parent_record_offset":13444,"ancestor_record_offset":13444,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2836,"record_offset":14228,"parent_record_offset":13444,"ancestor_record_offset":13444,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":2836,"record_offset":14168,"parent_record_offset":13444,"ancestor_record_offset":13444,"inlinee":8075,"source_line":281,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2836,"record_offset":14416,"parent_record_offset":14392,"ancestor_record_offset":13444,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2836,"record_offset":14440,"parent_record_offset":14416,"ancestor_record_offset":13444,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2836,"record_offset":14464,"parent_record_offset":14440,"ancestor_record_offset":13444,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2836,"record_offset":14392,"parent_record_offset":14300,"ancestor_record_offset":13444,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2836,"record_offset":13876,"parent_record_offset":13852,"ancestor_record_offset":13444,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2836,"record_offset":14328,"parent_record_offset":14300,"ancestor_record_offset":13444,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2836,"record_offset":14300,"parent_record_offset":14276,"ancestor_record_offset":13444,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2836,"record_offset":13924,"parent_record_offset":13900,"ancestor_record_offset":13444,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2836,"record_offset":13900,"parent_record_offset":13876,"ancestor_record_offset":13444,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2836,"record_offset":14276,"parent_record_offset":14252,"ancestor_record_offset":13444,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2836,"record_offset":13852,"parent_record_offset":13828,"ancestor_record_offset":13444,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2836,"record_offset":13828,"parent_record_offset":13796,"ancestor_record_offset":13444,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2836,"record_offset":14100,"parent_record_offset":14072,"ancestor_record_offset":13444,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2836,"record_offset":13796,"parent_record_offset":13764,"ancestor_record_offset":13444,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":2836,"record_offset":13736,"parent_record_offset":13620,"ancestor_record_offset":13444,"inlinee":10316,"source_line":40,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::init
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":2836,"record_offset":13764,"parent_record_offset":13736,"ancestor_record_offset":13444,"inlinee":9124,"source_line":2408,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::as_readonly
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":2836,"record_offset":13620,"parent_record_offset":13592,"ancestor_record_offset":13444,"inlinee":11849,"source_line":545,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::copy_unsafe
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":2836,"record_offset":13676,"parent_record_offset":13652,"ancestor_record_offset":13444,"inlinee":11851,"source_line":639,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":2836,"record_offset":13700,"parent_record_offset":13676,"ancestor_record_offset":13444,"inlinee":11852,"source_line":522,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::reborrow_unsafe
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":2836,"record_offset":13652,"parent_record_offset":13620,"ancestor_record_offset":13444,"inlinee":11850,"source_line":621,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::archetypes
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/unsafe_world_cell.rs
//     evidence={"module":2836,"record_offset":14032,"parent_record_offset":13736,"ancestor_record_offset":13444,"inlinee":11874,"source_line":264,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":2836,"record_offset":14128,"parent_record_offset":14100,"ancestor_record_offset":13444,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":2836,"record_offset":14360,"parent_record_offset":14328,"ancestor_record_offset":13444,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2836,"record_offset":14252,"parent_record_offset":14228,"ancestor_record_offset":13444,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2836,"record_offset":13992,"parent_record_offset":13968,"ancestor_record_offset":13444,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":2836,"record_offset":13968,"parent_record_offset":13796,"ancestor_record_offset":13444,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":2836,"record_offset":14192,"parent_record_offset":14168,"ancestor_record_offset":13444,"inlinee":8076,"source_line":245,"archive":"_pdb/C/modules/02836.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_critters::butterfly::butterflies_send_to_gpu::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 394
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1B31A
// 首条位置: {"module":2858,"inlinee":111386}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::mul_vec3
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs
//     evidence={"module":2858,"record_offset":5120,"parent_record_offset":5092,"ancestor_record_offset":5092,"inlinee":11719,"source_line":788,"archive":"_pdb/C/modules/02858.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":2858,"record_offset":5636,"parent_record_offset":5144,"ancestor_record_offset":5092,"inlinee":7191,"source_line":35,"archive":"_pdb/C/modules/02858.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ss
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":2858,"record_offset":5476,"parent_record_offset":5420,"ancestor_record_offset":5092,"inlinee":11729,"source_line":22,"archive":"_pdb/C/modules/02858.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":2858,"record_offset":5244,"parent_record_offset":5220,"ancestor_record_offset":5092,"inlinee":7190,"source_line":87,"archive":"_pdb/C/modules/02858.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":2858,"record_offset":5276,"parent_record_offset":5192,"ancestor_record_offset":5092,"inlinee":11716,"source_line":1045,"archive":"_pdb/C/modules/02858.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sub_ps
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":2858,"record_offset":5340,"parent_record_offset":5144,"ancestor_record_offset":5092,"inlinee":11730,"source_line":61,"archive":"_pdb/C/modules/02858.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3a
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs
//     evidence={"module":2858,"record_offset":5144,"parent_record_offset":5120,"ancestor_record_offset":5092,"inlinee":11724,"source_line":865,"archive":"_pdb/C/modules/02858.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":2858,"record_offset":5568,"parent_record_offset":5144,"ancestor_record_offset":5092,"inlinee":11725,"source_line":274,"archive":"_pdb/C/modules/02858.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_in_x
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/sse2.rs
//     evidence={"module":2858,"record_offset":5220,"parent_record_offset":5192,"ancestor_record_offset":5092,"inlinee":11728,"source_line":42,"archive":"_pdb/C/modules/02858.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_into_m128
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/sse2.rs
//     evidence={"module":2858,"record_offset":5192,"parent_record_offset":5144,"ancestor_record_offset":5092,"inlinee":11727,"source_line":66,"archive":"_pdb/C/modules/02858.jsonl.gz"}
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
// 完整结构及原始类型记录: butterfly.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: butterfly.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: butterfly.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 5; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::ButterflyAudio> >,tuple$<> >)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::ButterflyAudio> >,tuple$<> >"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::ButterflyAudio> >,tuple$<> >
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::Velocity>,ref_mut$<country_core::resources::critters::Butterfly>,ref_mut$<country_core::resources::critters::CritterGoal> >,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::critters::CritterRng>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::Velocity>,ref_mut$<country_core::resources::critters::Butterfly>,ref_mut$<country_core::resources::critters::CritterGoal> >,tuple$<> >", "bevy_ecs::change_detection::ResMut<country_core::resources::critters::CritterRng>", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::Velocity>,ref_mut$<country_core::resources::critters::Butterfly>,ref_mut$<country_core::resources::critters::CritterGoal> >,tuple$<> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::critters::CritterRng>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::Butterfly> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::Butterfly> >,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::Butterfly> >,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::Velocity>,ref_mut$<country_core::resources::critters::Butterfly>,ref_mut$<country_core::resources::critters::ButterflyAudio>,ref$<country_core::resources::critters::CritterGoal> >,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::critters::CritterRng>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::sheep::Sheep> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::Velocity>,ref_mut$<country_core::resources::critters::Butterfly>,ref_mut$<country_core::resources::critters::ButterflyAudio>,ref$<country_core::resources::critters::CritterGoal> >,tuple$<> >", "bevy_ecs::change_detection::ResMut<country_core::resources::critters::CritterRng>", "bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>", "bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::sheep::Sheep> >,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>", "bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::Velocity>,ref_mut$<country_core::resources::critters::Butterfly>,ref_mut$<country_core::resources::critters::ButterflyAudio>,ref$<country_core::resources::critters::CritterGoal> >,tuple$<> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::critters::CritterRng>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::sheep::Sheep> >,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::Butterfly>,ref$<country_core::resources::critters::CritterGoal>,ref_mut$<country_core::resources::critters::CritterVelocityTarget>,ref_mut$<country_core::resources::critters::CritterVelocityMultiplier> >,tuple$<> >)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::Butterfly>,ref$<country_core::resources::critters::CritterGoal>,ref_mut$<country_core::resources::critters::CritterVelocityTarget>,ref_mut$<country_core::resources::critters::CritterVelocityMultiplier> >,tuple$<> >"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::Butterfly>,ref$<country_core::resources::critters::CritterGoal>,ref_mut$<country_core::resources::critters::CritterVelocityTarget>,ref_mut$<country_core::resources::critters::CritterVelocityMultiplier> >,tuple$<> >

// ------------------------------------------------------------------
// 函数: system_critters::butterfly::butterflies_send_to_gpu
// 信息 ID: d0787ae7c22f48d0dc85867a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 75081655efbd10589c88ee75: void (*)(bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::Butterfly> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::Butterfly> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),tuple$<>,void (*)(bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::Butterfly> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)> @ 0x11608f0 -> 0x22920f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::Butterfly> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),tuple$<>,void (*)(bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::Butterfly> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)> > @ 0x22a6b91 -> 0x22920f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_critters::butterfly::butterflies_send_to_gpu::closure$0
// 信息 ID: 838fc4a6b1cb2b0331aeb915
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_critters::butterfly::butterflies_send_to_gpu", "captures": "unknown"}
// [闭包环境类型引用] system_critters::butterfly::butterflies_send_to_gpu::closure_env$0 <- core::iter::adapters::map::impl$2::next<country_core::systems::render_loop::resources::critters::ButterflySsbo,bevy_ecs::query::iter::QueryIter<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::Butterfly> >,tuple$<> >,system_critters::butterfly::butterflies_send_to_gpu::closure_env$0>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_critters::butterfly::butterflies_send_to_gpu::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<country_core::systems::render_loop::resources::critters::ButterflySsbo,core::iter::adapters::map::Map<bevy_ecs::query::iter::QueryIter<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::Butterfly> >,tuple$<> >,system_critters::butterfly::butterflies_send_to_gpu::closure_env$0> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::next<country_core::systems::render_loop::resources::critters::ButterflySsbo,bevy_ecs::query::iter::QueryIter<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::Butterfly> >,tuple$<> >,system_critters::butterfly::butterflies_send_to_gpu::closure_env$0>; module=2858, inline_record=5092; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<country_core::systems::render_loop::resources::critters::ButterflySsbo,core::iter::adapters::map::Map<bevy_ecs::query::iter::QueryIter<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::Butterfly> >,tuple$<> >,system_critters::butterfly::butterflies_send_to_gpu::closure_env$0> >; module=2858, inline_record=41620; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_critters::butterfly::set_butterfly_velocities_towards_goals
// 信息 ID: 328d9789ebca11ed37026bcf
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] ce1bfdaee8193447c8deea3c: void (*)(bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::Butterfly>,ref$<country_core::resources::critters::CritterGoal>,ref_mut$<country_core::resources::critters::CritterVelocityTarget>,ref_mut$<country_core::resources::critters::CritterVelocityMultiplier> >,tuple$<> >)
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::Butterfly>,ref$<country_core::resources::critters::CritterGoal>,ref_mut$<country_core::resources::critters::CritterVelocityTarget>,ref_mut$<country_core::resources::critters::CritterVelocityMultiplier> >,tuple$<> >),tuple$<>,void (*)(bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::Butterfly>,ref$<country_core::resources::critters::CritterGoal>,ref_mut$<country_core::resources::critters::CritterVelocityTarget>,ref_mut$<country_core::resources::critters::CritterVelocityMultiplier> >,tuple$<> >)> @ 0x1160798 -> 0x22936d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::Butterfly>,ref$<country_core::resources::critters::CritterGoal>,ref_mut$<country_core::resources::critters::CritterVelocityTarget>,ref_mut$<country_core::resources::critters::CritterVelocityMultiplier> >,tuple$<> >),tuple$<>,void (*)(bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::Butterfly>,ref$<country_core::resources::critters::CritterGoal>,ref_mut$<country_core::resources::critters::CritterVelocityTarget>,ref_mut$<country_core::resources::critters::CritterVelocityMultiplier> >,tuple$<> >)> > @ 0x22a7547 -> 0x22936d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::Butterfly>,ref$<country_core::resources::critters::CritterGoal>,ref_mut$<country_core::resources::critters::CritterVelocityTarget>,ref_mut$<country_core::resources::critters::CritterVelocityMultiplier> >,tuple$<> >),tuple$<>,void (*)(bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::Butterfly>,ref$<country_core::resources::critters::CritterGoal>,ref_mut$<country_core::resources::critters::CritterVelocityTarget>,ref_mut$<country_core::resources::critters::CritterVelocityMultiplier> >,tuple$<> >)> > @ 0x22affbf -> 0x22936d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_critters::butterfly::update_butterfly_audio
// 信息 ID: 6ac9155b4c83c32f77f0d98b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] a9c34386f1f131e2c1b19506: void (*)(bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::ButterflyAudio> >,tuple$<> >)
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::ButterflyAudio> >,tuple$<> >),tuple$<>,void (*)(bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::ButterflyAudio> >,tuple$<> >)> @ 0x11606ba -> 0x2291630 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::ButterflyAudio> >,tuple$<> >),tuple$<>,void (*)(bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::ButterflyAudio> >,tuple$<> >)> > @ 0x22a5f39 -> 0x2291630 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::ButterflyAudio> >,tuple$<> >),tuple$<>,void (*)(bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::ButterflyAudio> >,tuple$<> >)> > @ 0x22aed21 -> 0x2291630 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_critters::butterfly::update_butterfly_goals
// 信息 ID: 7dc1ab5e23a42731b8d3d6cd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 86ba0f51fc4b045d0de06ac6: void (*)(bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::Velocity>,ref_mut$<country_core::resources::critters::Butterfly>,ref_mut$<country_core::resources::critters::CritterGoal> >,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::critters::CritterRng>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::Velocity>,ref_mut$<country_core::resources::critters::Butterfly>,ref_mut$<country_core::resources::critters::CritterGoal> >,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::critters::CritterRng>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>),tuple$<>,void (*)(bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::Velocity>,ref_mut$<country_core::resources::critters::Butterfly>,ref_mut$<country_core::resources::critters::CritterGoal> >,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::critters::CritterRng>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)> @ 0x1160b03 -> 0x22918b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::Velocity>,ref_mut$<country_core::resources::critters::Butterfly>,ref_mut$<country_core::resources::critters::CritterGoal> >,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::critters::CritterRng>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>),tuple$<>,void (*)(bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::Velocity>,ref_mut$<country_core::resources::critters::Butterfly>,ref_mut$<country_core::resources::critters::CritterGoal> >,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::critters::CritterRng>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)> > @ 0x22a4c87 -> 0x22918b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_critters::butterfly::update_butterfly_goals::closure$0
// 信息 ID: f372d96598679d2bf8350dd1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_critters::butterfly::update_butterfly_goals", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_critters::butterfly::update_butterfly_goals; module=2836, inline_record=11380; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_critters::butterfly::update_butterfly_states
// 信息 ID: 41d6f8a9308417d7c45fa266
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 9d5cf95ffc1e7532999cfee7: void (*)(bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::Velocity>,ref_mut$<country_core::resources::critters::Butterfly>,ref_mut$<country_core::resources::critters::ButterflyAudio>,ref$<country_core::resources::critters::CritterGoal> >,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::critters::CritterRng>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::sheep::Sheep> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::Velocity>,ref_mut$<country_core::resources::critters::Butterfly>,ref_mut$<country_core::resources::critters::ButterflyAudio>,ref$<country_core::resources::critters::CritterGoal> >,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::critters::CritterRng>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::sheep::Sheep> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>),tuple$<>,void (*)(bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::Velocity>,ref_mut$<country_core::resources::critters::Butterfly>,ref_mut$<country_core::resources::critters::ButterflyAudio>,ref$<country_core::resources::critters::CritterGoal> >,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::critters::CritterRng>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::sheep::Sheep> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>)> @ 0x1160fab -> 0x2292250 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::Velocity>,ref_mut$<country_core::resources::critters::Butterfly>,ref_mut$<country_core::resources::critters::ButterflyAudio>,ref$<country_core::resources::critters::CritterGoal> >,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::critters::CritterRng>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::sheep::Sheep> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>),tuple$<>,void (*)(bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::Velocity>,ref_mut$<country_core::resources::critters::Butterfly>,ref_mut$<country_core::resources::critters::ButterflyAudio>,ref$<country_core::resources::critters::CritterGoal> >,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::critters::CritterRng>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::sheep::Sheep> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>)> > @ 0x22adbd4 -> 0x2292250 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_critters::butterfly::update_butterfly_states::closure$0
// 信息 ID: 251bbe30b7275fb33682c08f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_critters::butterfly::update_butterfly_states", "captures": "unknown"}
// [闭包环境类型引用] system_critters::butterfly::update_butterfly_states::closure_env$0 <- core::iter::adapters::map::impl$2::next<glam::f32::vec2::Vec2,bevy_ecs::query::iter::QueryIter<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::sheep::Sheep> >,tuple$<> >,system_critters::butterfly::update_butterfly_states::closure_env$0>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_critters::butterfly::update_butterfly_states::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::vec2::Vec2,core::iter::adapters::map::Map<bevy_ecs::query::iter::QueryIter<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::sheep::Sheep> >,tuple$<> >,system_critters::butterfly::update_butterfly_states::closure_env$0> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::next<glam::f32::vec2::Vec2,bevy_ecs::query::iter::QueryIter<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::sheep::Sheep> >,tuple$<> >,system_critters::butterfly::update_butterfly_states::closure_env$0>; module=2858, inline_record=10212; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_critters::butterfly::update_butterfly_states::closure$1
// 信息 ID: eeb021197eb76cd406243ef7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_critters::butterfly::update_butterfly_states", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_critters::butterfly::update_butterfly_states; module=2836, inline_record=24004; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===

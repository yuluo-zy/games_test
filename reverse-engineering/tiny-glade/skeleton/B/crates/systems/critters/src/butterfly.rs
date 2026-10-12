// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/systems/critters/src/butterfly.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: c0611c9f440879d5f2d919cc065d1c685354ba5e6f0b507c09d7a66e593abd5a
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: butterfly.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 1169, 2707, 2739
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use bevy_ptr; // namespace candidate
//   package identities: [{"name":"bevy_ptr","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.29.3"}]
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
// 首条位置: {"module":2707,"record_offset":2044,"rva":30952768,"va":5399661888}
// 编译实例 RVAs: 0x1d84d40
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: butterfly.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24584021","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399661888,"raw_hex":"e8abe088fe","size":5,"soft_fail":false,"target_va":5375081504,"va":5399665520,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067dd80
//     owner=unresolved_namespace; count=1
//     target_aliases=69 candidates; complete list: butterfly.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24146792","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399661888,"raw_hex":"e8988c8ffe","size":5,"soft_fail":false,"target_va":5375516032,"va":5399662819,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409ea800
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: butterfly.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20558432","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399661888,"raw_hex":"e8a04dc6fe","size":5,"soft_fail":false,"target_va":5379106816,"va":5399665243,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a14df0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: butterfly.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20384928","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399661888,"raw_hex":"e860f3c8fe","size":5,"soft_fail":false,"target_va":5379280368,"va":5399665291,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141dbfba0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: butterfly.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t240080","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399661888,"raw_hex":"e8d0a90300","size":5,"soft_fail":false,"target_va":5399903136,"va":5399663051,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14230e780
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: butterfly.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5805876","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399661888,"raw_hex":"e834975800","size":5,"soft_fail":false,"target_va":5405468544,"va":5399662663,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923bf0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: butterfly.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12184408","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399661888,"raw_hex":"e858ebb900","size":5,"soft_fail":false,"target_va":5411847152,"va":5399662739,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923f10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: butterfly.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12184575","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399661888,"raw_hex":"e8ffebb900","size":5,"soft_fail":false,"target_va":5411847952,"va":5399663372,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::query::iter::QueryIterationCursor$LT$D$C$F$GT$::init
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t5806210","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399661888,"raw_hex":"e882985800","size":5,"soft_fail":false,"target_va":5405469312,"va":5399663097,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t11875802","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399661888,"raw_hex":"e8da35b500","size":5,"soft_fail":false,"target_va":5411541584,"va":5399665777,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t11875925","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399661888,"raw_hex":"e85536b500","size":5,"soft_fail":false,"target_va":5411541776,"va":5399665846,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t11816603","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399661888,"raw_hex":"e89b4eb400","size":5,"soft_fail":false,"target_va":5411482433,"va":5399665825,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-1231","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399661888,"raw_hex":"e831fbffff","size":5,"soft_fail":false,"target_va":5399660816,"va":5399662042,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t11881236","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399661888,"raw_hex":"e8144bb500","size":5,"soft_fail":false,"target_va":5411543360,"va":5399662119,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast::position
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-20392673","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399661888,"raw_hex":"e81fd5c8fe","size":5,"soft_fail":false,"target_va":5379270224,"va":5399662892,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::camera_rig::MainCamera::eye_position
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-21444839","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399661888,"raw_hex":"e819c7b8fe","size":5,"soft_fail":false,"target_va":5378219472,"va":5399664306,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::camera_rig::MainCamera::mode
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-21447795","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399661888,"raw_hex":"e88dbbb8fe","size":5,"soft_fail":false,"target_va":5378216496,"va":5399664286,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] perchance::PerchanceContext::uniform_f32
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"perchance","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-7732643","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399661888,"raw_hex":"e85d028aff","size":5,"soft_fail":false,"target_va":5391932128,"va":5399664766,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-18277027","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399661888,"raw_hex":"e85d1de9fe","size":5,"soft_fail":false,"target_va":5381385536,"va":5399662558,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-18271838","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399661888,"raw_hex":"e8a231e9fe","size":5,"soft_fail":false,"target_va":5381390640,"va":5399662473,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-18279120","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399661888,"raw_hex":"e83015e9fe","size":5,"soft_fail":false,"target_va":5381386480,"va":5399665595,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-18277317","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399661888,"raw_hex":"e83b1ce9fe","size":5,"soft_fail":false,"target_va":5381384704,"va":5399662016,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::native::lazy::Storage$LT$T$C$D$GT$::initialize
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t12145656","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399661888,"raw_hex":"e8f853b900","size":5,"soft_fail":false,"target_va":5411811376,"va":5399665715,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::LocalKey$LT$T$GT$::with
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5801455","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399661888,"raw_hex":"e8ef855800","size":5,"soft_fail":false,"target_va":5405468144,"va":5399666684,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t11871334","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399661888,"raw_hex":"e86624b500","size":5,"soft_fail":false,"target_va":5411537104,"va":5399665765,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*%rax","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5399661888,"raw_hex":"ffd0","size":2,"soft_fail":false,"va":5399662770,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":2707,"record_offset":4696,"parent_record_offset":2044,"ancestor_record_offset":2044,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$58::deref_mut
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":2707,"record_offset":5908,"parent_record_offset":2044,"ancestor_record_offset":2044,"inlinee":9177,"source_line":434,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::query::iter::impl$1::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":2707,"record_offset":6020,"parent_record_offset":2044,"ancestor_record_offset":2044,"inlinee":11673,"source_line":884,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::iter
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":2707,"record_offset":5088,"parent_record_offset":2044,"ancestor_record_offset":2044,"inlinee":13704,"source_line":670,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::iter_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":2707,"record_offset":5664,"parent_record_offset":2044,"ancestor_record_offset":2044,"inlinee":17020,"source_line":701,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::abs
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":2707,"record_offset":11972,"parent_record_offset":2044,"ancestor_record_offset":2044,"inlinee":10511,"source_line":1446,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":2707,"record_offset":11832,"parent_record_offset":2044,"ancestor_record_offset":2044,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":2707,"record_offset":12448,"parent_record_offset":2044,"ancestor_record_offset":2044,"inlinee":4754,"source_line":912,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\collect.rs
//     evidence={"module":2707,"record_offset":5812,"parent_record_offset":2044,"ancestor_record_offset":2044,"inlinee":4756,"source_line":322,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":2707,"record_offset":5608,"parent_record_offset":2044,"ancestor_record_offset":2044,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":2707,"record_offset":5544,"parent_record_offset":2044,"ancestor_record_offset":2044,"inlinee":5102,"source_line":745,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_direct] core::ops::range::Range::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":2707,"record_offset":12000,"parent_record_offset":2044,"ancestor_record_offset":2044,"inlinee":7522,"source_line":120,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":2707,"record_offset":12476,"parent_record_offset":2044,"ancestor_record_offset":2044,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":2707,"record_offset":4996,"parent_record_offset":2044,"ancestor_record_offset":2044,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":2707,"record_offset":12652,"parent_record_offset":2044,"ancestor_record_offset":2044,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":2707,"record_offset":11580,"parent_record_offset":2044,"ancestor_record_offset":2044,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":2707,"record_offset":4756,"parent_record_offset":2044,"ancestor_record_offset":2044,"inlinee":4445,"source_line":242,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::ssbo::impl$6::mapped_data
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":2707,"record_offset":5032,"parent_record_offset":2044,"ancestor_record_offset":2044,"inlinee":31029,"source_line":301,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::world_raster::WorldRaster::as_float_grid
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/world_raster.rs
//     evidence={"module":2707,"record_offset":5060,"parent_record_offset":2044,"ancestor_record_offset":2044,"inlinee":25412,"source_line":158,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":2707,"record_offset":11372,"parent_record_offset":2044,"ancestor_record_offset":2044,"inlinee":9564,"source_line":422,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::length_squared
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":2707,"record_offset":11524,"parent_record_offset":2044,"ancestor_record_offset":2044,"inlinee":25585,"source_line":406,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":2707,"record_offset":5844,"parent_record_offset":2044,"ancestor_record_offset":2044,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_direct] perchance::PerchanceContext::uniform_range_f32
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"perchance","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/perchance-0.4.0/src/lib.rs
//     evidence={"module":2707,"record_offset":12168,"parent_record_offset":2044,"ancestor_record_offset":2044,"inlinee":26762,"source_line":270,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":2707,"record_offset":3712,"parent_record_offset":2044,"ancestor_record_offset":2044,"inlinee":4416,"source_line":589,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":2707,"record_offset":2280,"parent_record_offset":2044,"ancestor_record_offset":2044,"inlinee":4368,"source_line":626,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":2707,"record_offset":2736,"parent_record_offset":2044,"ancestor_record_offset":2044,"inlinee":4385,"source_line":659,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":2707,"record_offset":12840,"parent_record_offset":12816,"ancestor_record_offset":2044,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":2707,"record_offset":12816,"parent_record_offset":12724,"ancestor_record_offset":2044,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2707,"record_offset":5348,"parent_record_offset":5324,"ancestor_record_offset":2044,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2707,"record_offset":12752,"parent_record_offset":12724,"ancestor_record_offset":2044,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2707,"record_offset":12724,"parent_record_offset":12700,"ancestor_record_offset":2044,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2707,"record_offset":5396,"parent_record_offset":5372,"ancestor_record_offset":2044,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2707,"record_offset":5372,"parent_record_offset":5348,"ancestor_record_offset":2044,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2707,"record_offset":12700,"parent_record_offset":12676,"ancestor_record_offset":2044,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":2707,"record_offset":5324,"parent_record_offset":5300,"ancestor_record_offset":2044,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":2707,"record_offset":5300,"parent_record_offset":5268,"ancestor_record_offset":2044,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":2707,"record_offset":6324,"parent_record_offset":6300,"ancestor_record_offset":2044,"inlinee":4613,"source_line":2705,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":2707,"record_offset":6348,"parent_record_offset":6324,"ancestor_record_offset":2044,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":2707,"record_offset":5632,"parent_record_offset":5608,"ancestor_record_offset":2044,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":2707,"record_offset":5268,"parent_record_offset":5236,"ancestor_record_offset":2044,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::entities
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":2707,"record_offset":10504,"parent_record_offset":6044,"ancestor_record_offset":2044,"inlinee":11682,"source_line":466,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":2707,"record_offset":6544,"parent_record_offset":6044,"ancestor_record_offset":2044,"inlinee":11679,"source_line":615,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::table_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":2707,"record_offset":8568,"parent_record_offset":6044,"ancestor_record_offset":2044,"inlinee":5495,"source_line":460,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::ArchetypeEntity::table_row
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":2707,"record_offset":10700,"parent_record_offset":6044,"ancestor_record_offset":2044,"inlinee":17012,"source_line":325,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetypes::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":2707,"record_offset":6460,"parent_record_offset":6044,"ancestor_record_offset":2044,"inlinee":11678,"source_line":895,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":2707,"record_offset":4724,"parent_record_offset":4696,"ancestor_record_offset":2044,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$57::set_changed
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":2707,"record_offset":5932,"parent_record_offset":5908,"ancestor_record_offset":2044,"inlinee":9179,"source_line":397,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::StorageSwitch::extract
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":2707,"record_offset":10412,"parent_record_offset":10388,"ancestor_record_offset":2044,"inlinee":4876,"source_line":2472,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::set_archetype
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":2707,"record_offset":8692,"parent_record_offset":8660,"ancestor_record_offset":2044,"inlinee":4856,"source_line":1160,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::set_table
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":2707,"record_offset":6660,"parent_record_offset":6628,"ancestor_record_offset":2044,"inlinee":4857,"source_line":1175,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$27::fetch
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":2707,"record_offset":10388,"parent_record_offset":10364,"ancestor_record_offset":2044,"inlinee":4874,"source_line":1229,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$27::fetch::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":2707,"record_offset":10436,"parent_record_offset":10412,"ancestor_record_offset":2044,"inlinee":4878,"source_line":1235,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":2707,"record_offset":9712,"parent_record_offset":8660,"ancestor_record_offset":2044,"inlinee":13655,"source_line":1524,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_table
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":2707,"record_offset":7620,"parent_record_offset":6628,"ancestor_record_offset":2044,"inlinee":13656,"source_line":1539,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":2707,"record_offset":11092,"parent_record_offset":11064,"ancestor_record_offset":2044,"inlinee":9533,"source_line":1602,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$67::fetch
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":2707,"record_offset":10364,"parent_record_offset":6044,"ancestor_record_offset":2044,"inlinee":99526,"source_line":2083,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":2707,"record_offset":5204,"parent_record_offset":5116,"ancestor_record_offset":2044,"inlinee":13709,"source_line":39,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::init
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":2707,"record_offset":5236,"parent_record_offset":5204,"ancestor_record_offset":2044,"inlinee":9292,"source_line":2359,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":2707,"record_offset":6044,"parent_record_offset":6020,"ancestor_record_offset":2044,"inlinee":11674,"source_line":2458,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$5::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/world_query.rs
//     evidence={"module":2707,"record_offset":8660,"parent_record_offset":6044,"ancestor_record_offset":2044,"inlinee":99524,"source_line":182,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$5::set_table
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/world_query.rs
//     evidence={"module":2707,"record_offset":6628,"parent_record_offset":6044,"ancestor_record_offset":2044,"inlinee":99523,"source_line":195,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_array::BlobArray::get_sub_slice
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_array.rs
//     evidence={"module":2707,"record_offset":7080,"parent_record_offset":7056,"ancestor_record_offset":2044,"inlinee":4872,"source_line":143,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":2707,"record_offset":6760,"parent_record_offset":6736,"ancestor_record_offset":2044,"inlinee":4864,"source_line":58,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":2707,"record_offset":6736,"parent_record_offset":6712,"ancestor_record_offset":2044,"inlinee":4862,"source_line":395,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":2707,"record_offset":6940,"parent_record_offset":6916,"ancestor_record_offset":2044,"inlinee":4866,"source_line":396,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":2707,"record_offset":6712,"parent_record_offset":6684,"ancestor_record_offset":2044,"inlinee":4860,"source_line":475,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_data_slice_for
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":2707,"record_offset":6684,"parent_record_offset":6660,"ancestor_record_offset":2044,"inlinee":4859,"source_line":363,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":2707,"record_offset":6300,"parent_record_offset":6044,"ancestor_record_offset":2044,"inlinee":11675,"source_line":615,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::as_usize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":2707,"record_offset":10460,"parent_record_offset":10436,"ancestor_record_offset":2044,"inlinee":5472,"source_line":127,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":2707,"record_offset":6216,"parent_record_offset":6044,"ancestor_record_offset":2044,"inlinee":4854,"source_line":721,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_changed_ticks_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/column.rs
//     evidence={"module":2707,"record_offset":8044,"parent_record_offset":7620,"ancestor_record_offset":2044,"inlinee":13683,"source_line":319,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_data_slice
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/column.rs
//     evidence={"module":2707,"record_offset":7056,"parent_record_offset":7032,"ancestor_record_offset":2044,"inlinee":4870,"source_line":303,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::impl$4::get_data_slice_for::closure$0
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":2707,"record_offset":7032,"parent_record_offset":7008,"ancestor_record_offset":2044,"inlinee":4868,"source_line":368,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::thin_array_ptr::ThinArrayPtr::as_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/thin_array_ptr.rs
//     evidence={"module":2707,"record_offset":8068,"parent_record_offset":8044,"ancestor_record_offset":2044,"inlinee":13684,"source_line":300,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::as_readonly
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":2707,"record_offset":5116,"parent_record_offset":5088,"ancestor_record_offset":2044,"inlinee":13705,"source_line":542,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::copy_unsafe
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":2707,"record_offset":5172,"parent_record_offset":5148,"ancestor_record_offset":2044,"inlinee":13707,"source_line":636,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::reborrow
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":2707,"record_offset":5688,"parent_record_offset":5664,"ancestor_record_offset":2044,"inlinee":17021,"source_line":601,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::reborrow_unsafe
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":2707,"record_offset":5148,"parent_record_offset":5116,"ancestor_record_offset":2044,"inlinee":13706,"source_line":618,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::archetypes
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":2707,"record_offset":5504,"parent_record_offset":5204,"ancestor_record_offset":2044,"inlinee":13723,"source_line":263,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ptr","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.16.0/src/lib.rs
//     evidence={"module":2707,"record_offset":10880,"parent_record_offset":10852,"ancestor_record_offset":2044,"inlinee":4880,"source_line":526,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":2707,"record_offset":4048,"parent_record_offset":4020,"ancestor_record_offset":2044,"inlinee":4432,"source_line":1817,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":2707,"record_offset":4100,"parent_record_offset":4076,"ancestor_record_offset":2044,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":2707,"record_offset":4076,"parent_record_offset":4048,"ancestor_record_offset":2044,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":2707,"record_offset":3996,"parent_record_offset":3968,"ancestor_record_offset":2044,"inlinee":4429,"source_line":1089,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":2707,"record_offset":4020,"parent_record_offset":3996,"ancestor_record_offset":2044,"inlinee":4430,"source_line":1121,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":2707,"record_offset":4192,"parent_record_offset":4020,"ancestor_record_offset":2044,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$44::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":2707,"record_offset":4304,"parent_record_offset":4280,"ancestor_record_offset":2044,"inlinee":4443,"source_line":1808,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":2707,"record_offset":12048,"parent_record_offset":12024,"ancestor_record_offset":2044,"inlinee":4984,"source_line":1943,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":2707,"record_offset":12072,"parent_record_offset":12048,"ancestor_record_offset":2044,"inlinee":4986,"source_line":1788,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":2707,"record_offset":4824,"parent_record_offset":4784,"ancestor_record_offset":2044,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":2707,"record_offset":5572,"parent_record_offset":5544,"ancestor_record_offset":2044,"inlinee":5104,"source_line":68,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":2707,"record_offset":4124,"parent_record_offset":4100,"ancestor_record_offset":2044,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":2707,"record_offset":12784,"parent_record_offset":12752,"ancestor_record_offset":2044,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":2707,"record_offset":2436,"parent_record_offset":2404,"ancestor_record_offset":2044,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":2707,"record_offset":5872,"parent_record_offset":5844,"ancestor_record_offset":2044,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":2707,"record_offset":3864,"parent_record_offset":3820,"ancestor_record_offset":2044,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":2707,"record_offset":12024,"parent_record_offset":12000,"ancestor_record_offset":2044,"inlinee":4982,"source_line":817,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":2707,"record_offset":7744,"parent_record_offset":7720,"ancestor_record_offset":2044,"inlinee":4518,"source_line":1447,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":2707,"record_offset":6876,"parent_record_offset":6852,"ancestor_record_offset":2044,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":2707,"record_offset":6916,"parent_record_offset":6736,"ancestor_record_offset":2044,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":2707,"record_offset":2684,"parent_record_offset":2656,"ancestor_record_offset":2044,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":2707,"record_offset":4652,"parent_record_offset":3820,"ancestor_record_offset":2044,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":2707,"record_offset":4256,"parent_record_offset":3968,"ancestor_record_offset":2044,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2707,"record_offset":5464,"parent_record_offset":5440,"ancestor_record_offset":2044,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":2707,"record_offset":3148,"parent_record_offset":3124,"ancestor_record_offset":2044,"inlinee":4410,"source_line":533,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":2707,"record_offset":3124,"parent_record_offset":3100,"ancestor_record_offset":2044,"inlinee":4409,"source_line":686,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":2707,"record_offset":3332,"parent_record_offset":3304,"ancestor_record_offset":2044,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":2707,"record_offset":4148,"parent_record_offset":4124,"ancestor_record_offset":2044,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":2707,"record_offset":4908,"parent_record_offset":4880,"ancestor_record_offset":2044,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":2707,"record_offset":4936,"parent_record_offset":4908,"ancestor_record_offset":2044,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":2707,"record_offset":6240,"parent_record_offset":6216,"ancestor_record_offset":2044,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":2707,"record_offset":7936,"parent_record_offset":7912,"ancestor_record_offset":2044,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":2707,"record_offset":6264,"parent_record_offset":6240,"ancestor_record_offset":2044,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":2707,"record_offset":7912,"parent_record_offset":7880,"ancestor_record_offset":2044,"inlinee":4162,"source_line":242,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":2707,"record_offset":5440,"parent_record_offset":5268,"ancestor_record_offset":2044,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":2707,"record_offset":3076,"parent_record_offset":3052,"ancestor_record_offset":2044,"inlinee":4407,"source_line":80,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":2707,"record_offset":3100,"parent_record_offset":3076,"ancestor_record_offset":2044,"inlinee":4408,"source_line":118,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":2707,"record_offset":3404,"parent_record_offset":3376,"ancestor_record_offset":2044,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":2707,"record_offset":6144,"parent_record_offset":6044,"ancestor_record_offset":2044,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$183::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":2707,"record_offset":3052,"parent_record_offset":2936,"ancestor_record_offset":2044,"inlinee":4405,"source_line":392,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":2707,"record_offset":2404,"parent_record_offset":2376,"ancestor_record_offset":2044,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":2707,"record_offset":2764,"parent_record_offset":2736,"ancestor_record_offset":2044,"inlinee":4386,"source_line":1488,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":2707,"record_offset":2904,"parent_record_offset":2872,"ancestor_record_offset":2044,"inlinee":4396,"source_line":128,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":2707,"record_offset":2872,"parent_record_offset":2836,"ancestor_record_offset":2044,"inlinee":4394,"source_line":218,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":2707,"record_offset":2812,"parent_record_offset":2788,"ancestor_record_offset":2044,"inlinee":4390,"source_line":315,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":2707,"record_offset":2836,"parent_record_offset":2812,"ancestor_record_offset":2044,"inlinee":4392,"source_line":709,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":2707,"record_offset":2788,"parent_record_offset":2764,"ancestor_record_offset":2044,"inlinee":4388,"source_line":798,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":2707,"record_offset":3376,"parent_record_offset":2836,"ancestor_record_offset":2044,"inlinee":4400,"source_line":659,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":2707,"record_offset":2376,"parent_record_offset":2344,"ancestor_record_offset":2044,"inlinee":4373,"source_line":433,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":2707,"record_offset":2344,"parent_record_offset":2316,"ancestor_record_offset":2044,"inlinee":4372,"source_line":463,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":2707,"record_offset":2536,"parent_record_offset":2504,"ancestor_record_offset":2044,"inlinee":4380,"source_line":501,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":2707,"record_offset":2632,"parent_record_offset":2536,"ancestor_record_offset":2044,"inlinee":4381,"source_line":521,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":2707,"record_offset":2504,"parent_record_offset":2480,"ancestor_record_offset":2044,"inlinee":4379,"source_line":533,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":2707,"record_offset":4852,"parent_record_offset":4824,"ancestor_record_offset":2044,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":2707,"record_offset":2316,"parent_record_offset":2280,"ancestor_record_offset":2044,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":2707,"record_offset":2656,"parent_record_offset":2632,"ancestor_record_offset":2044,"inlinee":4383,"source_line":189,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":2707,"record_offset":2936,"parent_record_offset":2904,"ancestor_record_offset":2044,"inlinee":4398,"source_line":79,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":2707,"record_offset":3220,"parent_record_offset":2936,"ancestor_record_offset":2044,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":2707,"record_offset":3192,"parent_record_offset":2936,"ancestor_record_offset":2044,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":2707,"record_offset":2996,"parent_record_offset":2936,"ancestor_record_offset":2044,"inlinee":4403,"source_line":24,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":2707,"record_offset":4784,"parent_record_offset":4756,"ancestor_record_offset":2044,"inlinee":4446,"source_line":206,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":2707,"record_offset":11452,"parent_record_offset":11396,"ancestor_record_offset":2044,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_squared
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":2707,"record_offset":11768,"parent_record_offset":11684,"ancestor_record_offset":2044,"inlinee":25585,"source_line":406,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":2707,"record_offset":11396,"parent_record_offset":11372,"ancestor_record_offset":2044,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":2707,"record_offset":6964,"parent_record_offset":6940,"ancestor_record_offset":2044,"inlinee":4171,"source_line":153,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] perchance::PerchanceContext::uniform_range_f32_raw
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"perchance","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/perchance-0.4.0/src/lib.rs
//     evidence={"module":2707,"record_offset":12192,"parent_record_offset":12168,"ancestor_record_offset":2044,"inlinee":26763,"source_line":275,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":2707,"record_offset":3748,"parent_record_offset":3712,"ancestor_record_offset":2044,"inlinee":4418,"source_line":425,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":2707,"record_offset":12952,"parent_record_offset":12920,"ancestor_record_offset":2044,"inlinee":4966,"source_line":601,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":2707,"record_offset":13468,"parent_record_offset":13208,"ancestor_record_offset":2044,"inlinee":4968,"source_line":602,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":2707,"record_offset":3892,"parent_record_offset":3864,"ancestor_record_offset":2044,"inlinee":4423,"source_line":90,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":2707,"record_offset":3968,"parent_record_offset":3820,"ancestor_record_offset":2044,"inlinee":4427,"source_line":429,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":2707,"record_offset":4228,"parent_record_offset":3968,"ancestor_record_offset":2044,"inlinee":4441,"source_line":591,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":2707,"record_offset":11484,"parent_record_offset":11452,"ancestor_record_offset":2044,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":2707,"record_offset":3920,"parent_record_offset":3892,"ancestor_record_offset":2044,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":2707,"record_offset":3820,"parent_record_offset":3784,"ancestor_record_offset":2044,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":2707,"record_offset":3784,"parent_record_offset":3748,"ancestor_record_offset":2044,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] system_critters::butterfly::update_butterfly_states::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-critters","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/critters/src/butterfly.rs
//     evidence={"module":2707,"record_offset":11684,"parent_record_offset":11580,"ancestor_record_offset":2044,"inlinee":99527,"source_line":106,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_critters::butterfly::update_butterfly_states::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 92
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF0E5
// 首条位置: {"module":1169,"inlinee":61669}
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
// IPI IDs: 0x184C7
// 首条位置: {"module":2707,"inlinee":99527}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::Vec2::length_squared
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":2707,"record_offset":11768,"parent_record_offset":11684,"ancestor_record_offset":11684,"inlinee":25585,"source_line":406,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":2707,"record_offset":11712,"parent_record_offset":11684,"ancestor_record_offset":11684,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":2707,"record_offset":11736,"parent_record_offset":11712,"ancestor_record_offset":11684,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":2707,"record_offset":11792,"parent_record_offset":11768,"ancestor_record_offset":11684,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/02707.jsonl.gz"}
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
// 首条位置: {"module":2707,"record_offset":14412,"rva":30957664,"va":5399666784}
// 编译实例 RVAs: 0x1d86060
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140c90030
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: butterfly.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17786235","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399666784,"raw_hex":"e8859af0fe","size":5,"soft_fail":false,"target_va":5381881904,"va":5399668134,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] perchance::PerchanceContext::uniform_f32
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"perchance","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-7735900","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399666784,"raw_hex":"e8a4f589ff","size":5,"soft_fail":false,"target_va":5391932128,"va":5399668023,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::uniform_sample_sphere
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-17784249","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399666784,"raw_hex":"e847a2f0fe","size":5,"soft_fail":false,"target_va":5381883856,"va":5399668100,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":2707,"record_offset":14592,"parent_record_offset":14412,"ancestor_record_offset":14412,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$58::deref_mut
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":2707,"record_offset":15060,"parent_record_offset":14412,"ancestor_record_offset":14412,"inlinee":9177,"source_line":434,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::query::iter::impl$1::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":2707,"record_offset":15116,"parent_record_offset":14412,"ancestor_record_offset":14412,"inlinee":11673,"source_line":884,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::iter_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":2707,"record_offset":14648,"parent_record_offset":14412,"ancestor_record_offset":14412,"inlinee":17020,"source_line":701,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":2707,"record_offset":22156,"parent_record_offset":14412,"ancestor_record_offset":14412,"inlinee":4754,"source_line":912,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_none_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":2707,"record_offset":21092,"parent_record_offset":14412,"ancestor_record_offset":14412,"inlinee":9187,"source_line":672,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":2707,"record_offset":22184,"parent_record_offset":14412,"ancestor_record_offset":14412,"inlinee":4748,"source_line":396,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::normalize_or_zero
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":2707,"record_offset":21764,"parent_record_offset":14412,"ancestor_record_offset":14412,"inlinee":4891,"source_line":573,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$18::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":2707,"record_offset":21708,"parent_record_offset":14412,"ancestor_record_offset":14412,"inlinee":13671,"source_line":1193,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":2707,"record_offset":21416,"parent_record_offset":14412,"ancestor_record_offset":14412,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":2707,"record_offset":21608,"parent_record_offset":14412,"ancestor_record_offset":14412,"inlinee":4752,"source_line":1333,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_direct] perchance::PerchanceContext::uniform_range_f32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"perchance","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/perchance-0.4.0/src/lib.rs
//     evidence={"module":2707,"record_offset":21304,"parent_record_offset":14412,"ancestor_record_offset":14412,"inlinee":26762,"source_line":270,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2707,"record_offset":14892,"parent_record_offset":14868,"ancestor_record_offset":14412,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2707,"record_offset":14940,"parent_record_offset":14916,"ancestor_record_offset":14412,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2707,"record_offset":14916,"parent_record_offset":14892,"ancestor_record_offset":14412,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":2707,"record_offset":14868,"parent_record_offset":14844,"ancestor_record_offset":14412,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":2707,"record_offset":14844,"parent_record_offset":14812,"ancestor_record_offset":14412,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":2707,"record_offset":15496,"parent_record_offset":15472,"ancestor_record_offset":14412,"inlinee":4613,"source_line":2705,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":2707,"record_offset":15520,"parent_record_offset":15496,"ancestor_record_offset":14412,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":2707,"record_offset":14812,"parent_record_offset":14784,"ancestor_record_offset":14412,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::entities
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":2707,"record_offset":20112,"parent_record_offset":15140,"ancestor_record_offset":14412,"inlinee":11682,"source_line":466,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":2707,"record_offset":17732,"parent_record_offset":15140,"ancestor_record_offset":14412,"inlinee":11679,"source_line":615,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::table_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":2707,"record_offset":17816,"parent_record_offset":15140,"ancestor_record_offset":14412,"inlinee":5495,"source_line":460,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::ArchetypeEntity::table_row
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":2707,"record_offset":20308,"parent_record_offset":15140,"ancestor_record_offset":14412,"inlinee":17012,"source_line":325,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetypes::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":2707,"record_offset":17648,"parent_record_offset":15140,"ancestor_record_offset":14412,"inlinee":11678,"source_line":895,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":2707,"record_offset":14616,"parent_record_offset":14592,"ancestor_record_offset":14412,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$57::set_changed
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":2707,"record_offset":15084,"parent_record_offset":15060,"ancestor_record_offset":14412,"inlinee":9179,"source_line":397,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::StorageSwitch::extract
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":2707,"record_offset":20020,"parent_record_offset":19996,"ancestor_record_offset":14412,"inlinee":4876,"source_line":2472,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::set_archetype
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":2707,"record_offset":17940,"parent_record_offset":17908,"ancestor_record_offset":14412,"inlinee":4856,"source_line":1160,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::set_table
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":2707,"record_offset":15588,"parent_record_offset":15556,"ancestor_record_offset":14412,"inlinee":4857,"source_line":1175,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$27::fetch
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":2707,"record_offset":19996,"parent_record_offset":19972,"ancestor_record_offset":14412,"inlinee":4874,"source_line":1229,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$27::fetch::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":2707,"record_offset":20044,"parent_record_offset":20020,"ancestor_record_offset":14412,"inlinee":4878,"source_line":1235,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_archetype
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":2707,"record_offset":18960,"parent_record_offset":17908,"ancestor_record_offset":14412,"inlinee":13655,"source_line":1524,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_table
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":2707,"record_offset":16548,"parent_record_offset":15556,"ancestor_record_offset":14412,"inlinee":13656,"source_line":1539,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":2707,"record_offset":20696,"parent_record_offset":20668,"ancestor_record_offset":14412,"inlinee":9533,"source_line":1602,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$65::fetch
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":2707,"record_offset":19972,"parent_record_offset":15140,"ancestor_record_offset":14412,"inlinee":17024,"source_line":2083,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":2707,"record_offset":14760,"parent_record_offset":14672,"ancestor_record_offset":14412,"inlinee":13709,"source_line":39,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::init
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":2707,"record_offset":14784,"parent_record_offset":14760,"ancestor_record_offset":14412,"inlinee":9292,"source_line":2359,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":2707,"record_offset":15140,"parent_record_offset":15116,"ancestor_record_offset":14412,"inlinee":11674,"source_line":2458,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$4::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/world_query.rs
//     evidence={"module":2707,"record_offset":17908,"parent_record_offset":15140,"ancestor_record_offset":14412,"inlinee":17025,"source_line":182,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$4::set_table
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/world_query.rs
//     evidence={"module":2707,"record_offset":15556,"parent_record_offset":15140,"ancestor_record_offset":14412,"inlinee":17022,"source_line":195,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_array::BlobArray::get_sub_slice
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_array.rs
//     evidence={"module":2707,"record_offset":16008,"parent_record_offset":15984,"ancestor_record_offset":14412,"inlinee":4872,"source_line":143,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":2707,"record_offset":15688,"parent_record_offset":15664,"ancestor_record_offset":14412,"inlinee":4864,"source_line":58,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":2707,"record_offset":15664,"parent_record_offset":15640,"ancestor_record_offset":14412,"inlinee":4862,"source_line":395,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":2707,"record_offset":15868,"parent_record_offset":15844,"ancestor_record_offset":14412,"inlinee":4866,"source_line":396,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":2707,"record_offset":15640,"parent_record_offset":15612,"ancestor_record_offset":14412,"inlinee":4860,"source_line":475,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_data_slice_for
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":2707,"record_offset":15612,"parent_record_offset":15588,"ancestor_record_offset":14412,"inlinee":4859,"source_line":363,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":2707,"record_offset":15472,"parent_record_offset":15140,"ancestor_record_offset":14412,"inlinee":11675,"source_line":615,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::as_usize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":2707,"record_offset":20068,"parent_record_offset":20044,"ancestor_record_offset":14412,"inlinee":5472,"source_line":127,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":2707,"record_offset":15388,"parent_record_offset":15140,"ancestor_record_offset":14412,"inlinee":4854,"source_line":721,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_changed_ticks_slice
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/column.rs
//     evidence={"module":2707,"record_offset":16972,"parent_record_offset":16548,"ancestor_record_offset":14412,"inlinee":13683,"source_line":319,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_data_slice
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/column.rs
//     evidence={"module":2707,"record_offset":15984,"parent_record_offset":15960,"ancestor_record_offset":14412,"inlinee":4870,"source_line":303,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::impl$4::get_data_slice_for::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":2707,"record_offset":15960,"parent_record_offset":15936,"ancestor_record_offset":14412,"inlinee":4868,"source_line":368,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::thin_array_ptr::ThinArrayPtr::as_slice
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/thin_array_ptr.rs
//     evidence={"module":2707,"record_offset":16996,"parent_record_offset":16972,"ancestor_record_offset":14412,"inlinee":13684,"source_line":300,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::copy_unsafe
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":2707,"record_offset":14728,"parent_record_offset":14704,"ancestor_record_offset":14412,"inlinee":13707,"source_line":636,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::reborrow
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":2707,"record_offset":14672,"parent_record_offset":14648,"ancestor_record_offset":14412,"inlinee":17021,"source_line":601,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::reborrow_unsafe
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":2707,"record_offset":14704,"parent_record_offset":14672,"ancestor_record_offset":14412,"inlinee":13706,"source_line":618,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ptr","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.16.0/src/lib.rs
//     evidence={"module":2707,"record_offset":20484,"parent_record_offset":20456,"ancestor_record_offset":14412,"inlinee":4880,"source_line":526,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":2707,"record_offset":21644,"parent_record_offset":21608,"ancestor_record_offset":14412,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":2707,"record_offset":21460,"parent_record_offset":21416,"ancestor_record_offset":14412,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":2707,"record_offset":21172,"parent_record_offset":21144,"ancestor_record_offset":14412,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":2707,"record_offset":15780,"parent_record_offset":15716,"ancestor_record_offset":14412,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":2707,"record_offset":16672,"parent_record_offset":16648,"ancestor_record_offset":14412,"inlinee":4518,"source_line":1447,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":2707,"record_offset":15804,"parent_record_offset":15780,"ancestor_record_offset":14412,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":2707,"record_offset":15844,"parent_record_offset":15664,"ancestor_record_offset":14412,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2707,"record_offset":15008,"parent_record_offset":14984,"ancestor_record_offset":14412,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":2707,"record_offset":15348,"parent_record_offset":15324,"ancestor_record_offset":14412,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":2707,"record_offset":15272,"parent_record_offset":15236,"ancestor_record_offset":14412,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":2707,"record_offset":15412,"parent_record_offset":15388,"ancestor_record_offset":14412,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":2707,"record_offset":16864,"parent_record_offset":16840,"ancestor_record_offset":14412,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":2707,"record_offset":15436,"parent_record_offset":15412,"ancestor_record_offset":14412,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":2707,"record_offset":16840,"parent_record_offset":16808,"ancestor_record_offset":14412,"inlinee":4162,"source_line":242,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":2707,"record_offset":14984,"parent_record_offset":14812,"ancestor_record_offset":14412,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":2707,"record_offset":15324,"parent_record_offset":15300,"ancestor_record_offset":14412,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":2707,"record_offset":15236,"parent_record_offset":15140,"ancestor_record_offset":14412,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":2707,"record_offset":15300,"parent_record_offset":15236,"ancestor_record_offset":14412,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":2707,"record_offset":22208,"parent_record_offset":22184,"ancestor_record_offset":14412,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":2707,"record_offset":21260,"parent_record_offset":21232,"ancestor_record_offset":14412,"inlinee":4895,"source_line":203,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":2707,"record_offset":21852,"parent_record_offset":21824,"ancestor_record_offset":14412,"inlinee":4894,"source_line":451,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":2707,"record_offset":21824,"parent_record_offset":21792,"ancestor_record_offset":14412,"inlinee":4893,"source_line":470,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_squared
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":2707,"record_offset":21232,"parent_record_offset":21120,"ancestor_record_offset":14412,"inlinee":24882,"source_line":461,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::normalize_or
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":2707,"record_offset":21792,"parent_record_offset":21764,"ancestor_record_offset":14412,"inlinee":4892,"source_line":556,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":2707,"record_offset":21144,"parent_record_offset":21120,"ancestor_record_offset":14412,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":2707,"record_offset":15892,"parent_record_offset":15868,"ancestor_record_offset":14412,"inlinee":4171,"source_line":153,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] perchance::PerchanceContext::uniform_range_f32_raw
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"perchance","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/perchance-0.4.0/src/lib.rs
//     evidence={"module":2707,"record_offset":21328,"parent_record_offset":21304,"ancestor_record_offset":14412,"inlinee":26763,"source_line":275,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":2707,"record_offset":21924,"parent_record_offset":21888,"ancestor_record_offset":14412,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] system_critters::butterfly::update_butterfly_goals::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-critters","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/critters/src/butterfly.rs
//     evidence={"module":2707,"record_offset":21120,"parent_record_offset":21092,"ancestor_record_offset":14412,"inlinee":99529,"source_line":277,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_critters::butterfly::update_butterfly_goals::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 277
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x184C9
// 首条位置: {"module":2707,"inlinee":99529}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec3::Vec3::length_squared
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":2707,"record_offset":21232,"parent_record_offset":21120,"ancestor_record_offset":21120,"inlinee":24882,"source_line":461,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":2707,"record_offset":21144,"parent_record_offset":21120,"ancestor_record_offset":21120,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":2707,"record_offset":21172,"parent_record_offset":21144,"ancestor_record_offset":21120,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":2707,"record_offset":21260,"parent_record_offset":21232,"ancestor_record_offset":21120,"inlinee":4895,"source_line":203,"archive":"_pdb/B/modules/02707.jsonl.gz"}
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
// 首条位置: {"module":2707,"record_offset":22428,"rva":30959536,"va":5399668656}
// 编译实例 RVAs: 0x1d867b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] bevy_ecs::query::iter::QueryIterationCursor$LT$D$C$F$GT$::init
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t5800911","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399668656,"raw_hex":"e8cf835800","size":5,"soft_fail":false,"target_va":5405469680,"va":5399668764,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$58::deref_mut
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":2707,"record_offset":30172,"parent_record_offset":22428,"ancestor_record_offset":22428,"inlinee":9177,"source_line":434,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::query::iter::impl$1::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":2707,"record_offset":22796,"parent_record_offset":22428,"ancestor_record_offset":22428,"inlinee":11673,"source_line":884,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::iter_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":2707,"record_offset":22612,"parent_record_offset":22428,"ancestor_record_offset":22428,"inlinee":17020,"source_line":701,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":2707,"record_offset":30396,"parent_record_offset":22428,"ancestor_record_offset":22428,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":2707,"record_offset":29748,"parent_record_offset":22428,"ancestor_record_offset":22428,"inlinee":4666,"source_line":998,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::normalize_or_zero
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":2707,"record_offset":29872,"parent_record_offset":22428,"ancestor_record_offset":22428,"inlinee":4891,"source_line":573,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":2707,"record_offset":30080,"parent_record_offset":22428,"ancestor_record_offset":22428,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$28::mul_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":2707,"record_offset":30592,"parent_record_offset":22428,"ancestor_record_offset":22428,"inlinee":13663,"source_line":1280,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":2707,"record_offset":29780,"parent_record_offset":22428,"ancestor_record_offset":22428,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2707,"record_offset":28808,"parent_record_offset":28784,"ancestor_record_offset":22428,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2707,"record_offset":28856,"parent_record_offset":28832,"ancestor_record_offset":22428,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2707,"record_offset":28832,"parent_record_offset":28808,"ancestor_record_offset":22428,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":2707,"record_offset":28784,"parent_record_offset":28760,"ancestor_record_offset":22428,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":2707,"record_offset":28760,"parent_record_offset":28736,"ancestor_record_offset":22428,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":2707,"record_offset":23092,"parent_record_offset":23068,"ancestor_record_offset":22428,"inlinee":4613,"source_line":2705,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":2707,"record_offset":23116,"parent_record_offset":23092,"ancestor_record_offset":22428,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":2707,"record_offset":28736,"parent_record_offset":28712,"ancestor_record_offset":22428,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::entities
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":2707,"record_offset":28712,"parent_record_offset":22820,"ancestor_record_offset":22428,"inlinee":11682,"source_line":466,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":2707,"record_offset":23312,"parent_record_offset":22820,"ancestor_record_offset":22428,"inlinee":11679,"source_line":615,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::table_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":2707,"record_offset":25856,"parent_record_offset":22820,"ancestor_record_offset":22428,"inlinee":5495,"source_line":460,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::ArchetypeEntity::table_row
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":2707,"record_offset":28908,"parent_record_offset":22820,"ancestor_record_offset":22428,"inlinee":17012,"source_line":325,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetypes::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":2707,"record_offset":23228,"parent_record_offset":22820,"ancestor_record_offset":22428,"inlinee":11678,"source_line":895,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$57::set_changed
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":2707,"record_offset":30196,"parent_record_offset":30172,"ancestor_record_offset":22428,"inlinee":9179,"source_line":397,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::StorageSwitch::extract
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":2707,"record_offset":28620,"parent_record_offset":28596,"ancestor_record_offset":22428,"inlinee":4876,"source_line":2472,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::set_archetype
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":2707,"record_offset":25980,"parent_record_offset":25948,"ancestor_record_offset":22428,"inlinee":4856,"source_line":1160,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::set_table
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":2707,"record_offset":23424,"parent_record_offset":23396,"ancestor_record_offset":22428,"inlinee":4857,"source_line":1175,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$27::fetch
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":2707,"record_offset":28596,"parent_record_offset":28572,"ancestor_record_offset":22428,"inlinee":4874,"source_line":1229,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$27::fetch::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":2707,"record_offset":28644,"parent_record_offset":28620,"ancestor_record_offset":22428,"inlinee":4878,"source_line":1235,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_archetype
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":2707,"record_offset":27560,"parent_record_offset":25948,"ancestor_record_offset":22428,"inlinee":13655,"source_line":1524,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_table
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":2707,"record_offset":24908,"parent_record_offset":23396,"ancestor_record_offset":22428,"inlinee":13656,"source_line":1539,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":2707,"record_offset":29296,"parent_record_offset":29268,"ancestor_record_offset":22428,"inlinee":9533,"source_line":1602,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$67::fetch
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":2707,"record_offset":28572,"parent_record_offset":22820,"ancestor_record_offset":22428,"inlinee":99526,"source_line":2083,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":2707,"record_offset":22720,"parent_record_offset":22636,"ancestor_record_offset":22428,"inlinee":13709,"source_line":39,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":2707,"record_offset":22820,"parent_record_offset":22796,"ancestor_record_offset":22428,"inlinee":11674,"source_line":2458,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$5::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/world_query.rs
//     evidence={"module":2707,"record_offset":25948,"parent_record_offset":22820,"ancestor_record_offset":22428,"inlinee":99524,"source_line":182,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$5::set_table
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/world_query.rs
//     evidence={"module":2707,"record_offset":23396,"parent_record_offset":22820,"ancestor_record_offset":22428,"inlinee":99523,"source_line":195,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_array::BlobArray::get_sub_slice
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_array.rs
//     evidence={"module":2707,"record_offset":23844,"parent_record_offset":23820,"ancestor_record_offset":22428,"inlinee":4872,"source_line":143,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":2707,"record_offset":23524,"parent_record_offset":23500,"ancestor_record_offset":22428,"inlinee":4864,"source_line":58,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":2707,"record_offset":23500,"parent_record_offset":23476,"ancestor_record_offset":22428,"inlinee":4862,"source_line":395,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":2707,"record_offset":23704,"parent_record_offset":23680,"ancestor_record_offset":22428,"inlinee":4866,"source_line":396,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":2707,"record_offset":23476,"parent_record_offset":23448,"ancestor_record_offset":22428,"inlinee":4860,"source_line":475,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_data_slice_for
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":2707,"record_offset":23448,"parent_record_offset":23424,"ancestor_record_offset":22428,"inlinee":4859,"source_line":363,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":2707,"record_offset":23068,"parent_record_offset":22820,"ancestor_record_offset":22428,"inlinee":11675,"source_line":615,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::as_usize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":2707,"record_offset":28668,"parent_record_offset":28644,"ancestor_record_offset":22428,"inlinee":5472,"source_line":127,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":2707,"record_offset":22984,"parent_record_offset":22820,"ancestor_record_offset":22428,"inlinee":4854,"source_line":721,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_changed_ticks_slice
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/column.rs
//     evidence={"module":2707,"record_offset":25332,"parent_record_offset":24908,"ancestor_record_offset":22428,"inlinee":13683,"source_line":319,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_data_slice
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/column.rs
//     evidence={"module":2707,"record_offset":23820,"parent_record_offset":23796,"ancestor_record_offset":22428,"inlinee":4870,"source_line":303,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::impl$4::get_data_slice_for::closure$0
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":2707,"record_offset":23796,"parent_record_offset":23772,"ancestor_record_offset":22428,"inlinee":4868,"source_line":368,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::thin_array_ptr::ThinArrayPtr::as_slice
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/thin_array_ptr.rs
//     evidence={"module":2707,"record_offset":25356,"parent_record_offset":25332,"ancestor_record_offset":22428,"inlinee":13684,"source_line":300,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::copy_unsafe
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":2707,"record_offset":22688,"parent_record_offset":22664,"ancestor_record_offset":22428,"inlinee":13707,"source_line":636,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::reborrow
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":2707,"record_offset":22636,"parent_record_offset":22612,"ancestor_record_offset":22428,"inlinee":17021,"source_line":601,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::reborrow_unsafe
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":2707,"record_offset":22664,"parent_record_offset":22636,"ancestor_record_offset":22428,"inlinee":13706,"source_line":618,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::archetypes
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":2707,"record_offset":22756,"parent_record_offset":22720,"ancestor_record_offset":22428,"inlinee":13723,"source_line":263,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"bevy_ptr","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.16.0/src/lib.rs
//     evidence={"module":2707,"record_offset":29084,"parent_record_offset":29056,"ancestor_record_offset":22428,"inlinee":4880,"source_line":526,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":2707,"record_offset":30108,"parent_record_offset":30080,"ancestor_record_offset":22428,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$417::mul_assign
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":2707,"record_offset":30620,"parent_record_offset":30592,"ancestor_record_offset":22428,"inlinee":13665,"source_line":886,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":2707,"record_offset":29812,"parent_record_offset":29780,"ancestor_record_offset":22428,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":2707,"record_offset":23616,"parent_record_offset":23552,"ancestor_record_offset":22428,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":2707,"record_offset":25032,"parent_record_offset":25008,"ancestor_record_offset":22428,"inlinee":4518,"source_line":1447,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":2707,"record_offset":23640,"parent_record_offset":23616,"ancestor_record_offset":22428,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":2707,"record_offset":23680,"parent_record_offset":23500,"ancestor_record_offset":22428,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2707,"record_offset":29112,"parent_record_offset":29084,"ancestor_record_offset":22428,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":2707,"record_offset":22948,"parent_record_offset":22912,"ancestor_record_offset":22428,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":2707,"record_offset":23008,"parent_record_offset":22984,"ancestor_record_offset":22428,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":2707,"record_offset":25224,"parent_record_offset":25200,"ancestor_record_offset":22428,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":2707,"record_offset":23032,"parent_record_offset":23008,"ancestor_record_offset":22428,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":2707,"record_offset":25200,"parent_record_offset":25168,"ancestor_record_offset":22428,"inlinee":4162,"source_line":242,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":2707,"record_offset":22912,"parent_record_offset":22820,"ancestor_record_offset":22428,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":2707,"record_offset":29996,"parent_record_offset":29960,"ancestor_record_offset":22428,"inlinee":4895,"source_line":203,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":2707,"record_offset":29960,"parent_record_offset":29932,"ancestor_record_offset":22428,"inlinee":4894,"source_line":451,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":2707,"record_offset":29932,"parent_record_offset":29900,"ancestor_record_offset":22428,"inlinee":4893,"source_line":470,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::normalize_or
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":2707,"record_offset":29900,"parent_record_offset":29872,"ancestor_record_offset":22428,"inlinee":4892,"source_line":556,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":2707,"record_offset":23728,"parent_record_offset":23704,"ancestor_record_offset":22428,"inlinee":4171,"source_line":153,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":2707,"record_offset":30032,"parent_record_offset":29996,"ancestor_record_offset":22428,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/02707.jsonl.gz"}
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
// 首条位置: {"module":2707,"record_offset":30768,"rva":30961328,"va":5399670448}
// 编译实例 RVAs: 0x1d86eb0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] country_core::resources::critters::ButterflyAudio::set_position
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-14685084","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399670448,"raw_hex":"e864ec1fff","size":5,"soft_fail":false,"target_va":5384985504,"va":5399670583,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::query::iter::impl$1::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":2707,"record_offset":31312,"parent_record_offset":30768,"ancestor_record_offset":30768,"inlinee":11673,"source_line":884,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::iter_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":2707,"record_offset":30908,"parent_record_offset":30768,"ancestor_record_offset":30768,"inlinee":17020,"source_line":701,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2707,"record_offset":31144,"parent_record_offset":31120,"ancestor_record_offset":30768,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2707,"record_offset":31192,"parent_record_offset":31168,"ancestor_record_offset":30768,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2707,"record_offset":31168,"parent_record_offset":31144,"ancestor_record_offset":30768,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":2707,"record_offset":31120,"parent_record_offset":31096,"ancestor_record_offset":30768,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":2707,"record_offset":31096,"parent_record_offset":31064,"ancestor_record_offset":30768,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":2707,"record_offset":31960,"parent_record_offset":31936,"ancestor_record_offset":30768,"inlinee":4613,"source_line":2705,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":2707,"record_offset":31984,"parent_record_offset":31960,"ancestor_record_offset":30768,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":2707,"record_offset":31064,"parent_record_offset":31040,"ancestor_record_offset":30768,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::entities
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":2707,"record_offset":32984,"parent_record_offset":31344,"ancestor_record_offset":30768,"inlinee":11682,"source_line":466,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":2707,"record_offset":33640,"parent_record_offset":31344,"ancestor_record_offset":30768,"inlinee":11679,"source_line":615,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::table_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":2707,"record_offset":33724,"parent_record_offset":31344,"ancestor_record_offset":30768,"inlinee":5495,"source_line":460,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::ArchetypeEntity::table_row
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":2707,"record_offset":33180,"parent_record_offset":31344,"ancestor_record_offset":30768,"inlinee":17012,"source_line":325,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetypes::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":2707,"record_offset":33556,"parent_record_offset":31344,"ancestor_record_offset":30768,"inlinee":11678,"source_line":895,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::StorageSwitch::extract
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":2707,"record_offset":31468,"parent_record_offset":31444,"ancestor_record_offset":30768,"inlinee":4876,"source_line":2472,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::set_archetype
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":2707,"record_offset":33840,"parent_record_offset":33816,"ancestor_record_offset":30768,"inlinee":4856,"source_line":1160,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::set_table
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":2707,"record_offset":32044,"parent_record_offset":32020,"ancestor_record_offset":30768,"inlinee":4857,"source_line":1175,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$27::fetch
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":2707,"record_offset":31444,"parent_record_offset":31420,"ancestor_record_offset":30768,"inlinee":4874,"source_line":1229,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$27::fetch::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":2707,"record_offset":31492,"parent_record_offset":31468,"ancestor_record_offset":30768,"inlinee":4878,"source_line":1235,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$61::fetch
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":2707,"record_offset":31420,"parent_record_offset":31344,"ancestor_record_offset":30768,"inlinee":17019,"source_line":2083,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":2707,"record_offset":31016,"parent_record_offset":30932,"ancestor_record_offset":30768,"inlinee":13709,"source_line":39,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::init
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":2707,"record_offset":31040,"parent_record_offset":31016,"ancestor_record_offset":30768,"inlinee":9292,"source_line":2359,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":2707,"record_offset":31344,"parent_record_offset":31312,"ancestor_record_offset":30768,"inlinee":11674,"source_line":2458,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$2::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/world_query.rs
//     evidence={"module":2707,"record_offset":33816,"parent_record_offset":31344,"ancestor_record_offset":30768,"inlinee":13725,"source_line":182,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$2::set_table
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/world_query.rs
//     evidence={"module":2707,"record_offset":32020,"parent_record_offset":31344,"ancestor_record_offset":30768,"inlinee":17013,"source_line":195,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_array::BlobArray::get_sub_slice
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_array.rs
//     evidence={"module":2707,"record_offset":32464,"parent_record_offset":32440,"ancestor_record_offset":30768,"inlinee":4872,"source_line":143,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":2707,"record_offset":32144,"parent_record_offset":32120,"ancestor_record_offset":30768,"inlinee":4864,"source_line":58,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":2707,"record_offset":32120,"parent_record_offset":32096,"ancestor_record_offset":30768,"inlinee":4862,"source_line":395,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":2707,"record_offset":32324,"parent_record_offset":32300,"ancestor_record_offset":30768,"inlinee":4866,"source_line":396,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":2707,"record_offset":32096,"parent_record_offset":32068,"ancestor_record_offset":30768,"inlinee":4860,"source_line":475,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_data_slice_for
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":2707,"record_offset":32068,"parent_record_offset":32044,"ancestor_record_offset":30768,"inlinee":4859,"source_line":363,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":2707,"record_offset":31936,"parent_record_offset":31344,"ancestor_record_offset":30768,"inlinee":11675,"source_line":615,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::as_usize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":2707,"record_offset":31516,"parent_record_offset":31492,"ancestor_record_offset":30768,"inlinee":5472,"source_line":127,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":2707,"record_offset":31852,"parent_record_offset":31344,"ancestor_record_offset":30768,"inlinee":4854,"source_line":721,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_data_slice
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/column.rs
//     evidence={"module":2707,"record_offset":32440,"parent_record_offset":32416,"ancestor_record_offset":30768,"inlinee":4870,"source_line":303,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::impl$4::get_data_slice_for::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":2707,"record_offset":32416,"parent_record_offset":32392,"ancestor_record_offset":30768,"inlinee":4868,"source_line":368,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::copy_unsafe
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":2707,"record_offset":30984,"parent_record_offset":30960,"ancestor_record_offset":30768,"inlinee":13707,"source_line":636,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::reborrow
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":2707,"record_offset":30932,"parent_record_offset":30908,"ancestor_record_offset":30768,"inlinee":17021,"source_line":601,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::reborrow_unsafe
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":2707,"record_offset":30960,"parent_record_offset":30932,"ancestor_record_offset":30768,"inlinee":13706,"source_line":618,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ptr","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.16.0/src/lib.rs
//     evidence={"module":2707,"record_offset":31628,"parent_record_offset":31604,"ancestor_record_offset":30768,"inlinee":4880,"source_line":526,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":2707,"record_offset":32236,"parent_record_offset":32172,"ancestor_record_offset":30768,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":2707,"record_offset":32260,"parent_record_offset":32236,"ancestor_record_offset":30768,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":2707,"record_offset":32300,"parent_record_offset":32120,"ancestor_record_offset":30768,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2707,"record_offset":31260,"parent_record_offset":31236,"ancestor_record_offset":30768,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":2707,"record_offset":31812,"parent_record_offset":31788,"ancestor_record_offset":30768,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":2707,"record_offset":31736,"parent_record_offset":31700,"ancestor_record_offset":30768,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":2707,"record_offset":31876,"parent_record_offset":31852,"ancestor_record_offset":30768,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":2707,"record_offset":31900,"parent_record_offset":31876,"ancestor_record_offset":30768,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":2707,"record_offset":31236,"parent_record_offset":31064,"ancestor_record_offset":30768,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":2707,"record_offset":31788,"parent_record_offset":31764,"ancestor_record_offset":30768,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":2707,"record_offset":31700,"parent_record_offset":31344,"ancestor_record_offset":30768,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":2707,"record_offset":31764,"parent_record_offset":31700,"ancestor_record_offset":30768,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":2707,"record_offset":32348,"parent_record_offset":32324,"ancestor_record_offset":30768,"inlinee":4171,"source_line":153,"archive":"_pdb/B/modules/02707.jsonl.gz"}
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
// 首条位置: {"module":2707,"record_offset":34848,"rva":30961984,"va":5399671104}
// 编译实例 RVAs: 0x1d87140
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: butterfly.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24589928","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399671104,"raw_hex":"e898c988fe","size":5,"soft_fail":false,"target_va":5375081504,"va":5399671427,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141dbe3c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: butterfly.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t225753","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399671104,"raw_hex":"e8d9710300","size":5,"soft_fail":false,"target_va":5399897024,"va":5399671266,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14230e780
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: butterfly.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5797239","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399671104,"raw_hex":"e877755800","size":5,"soft_fail":false,"target_va":5405468544,"va":5399671300,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14230edb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: butterfly.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5798724","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399671104,"raw_hex":"e8447b5800","size":5,"soft_fail":false,"target_va":5405470128,"va":5399671399,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923bf0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: butterfly.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12175785","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399671104,"raw_hex":"e8a9c9b900","size":5,"soft_fail":false,"target_va":5411847152,"va":5399671362,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t11810925","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399671104,"raw_hex":"e86d38b400","size":5,"soft_fail":false,"target_va":5411482433,"va":5399671503,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t11870596","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399671104,"raw_hex":"e88421b500","size":5,"soft_fail":false,"target_va":5411542064,"va":5399671463,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*24(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5399671104,"raw_hex":"ff5018","size":3,"soft_fail":false,"va":5399671378,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::iter
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":2707,"record_offset":34996,"parent_record_offset":34848,"ancestor_record_offset":34848,"inlinee":13704,"source_line":670,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":2707,"record_offset":35444,"parent_record_offset":34848,"ancestor_record_offset":34848,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":2707,"record_offset":35740,"parent_record_offset":34848,"ancestor_record_offset":34848,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":2707,"record_offset":35500,"parent_record_offset":34848,"ancestor_record_offset":34848,"inlinee":4445,"source_line":242,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":2707,"record_offset":35928,"parent_record_offset":35904,"ancestor_record_offset":34848,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":2707,"record_offset":35904,"parent_record_offset":35812,"ancestor_record_offset":34848,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2707,"record_offset":35276,"parent_record_offset":35252,"ancestor_record_offset":34848,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2707,"record_offset":35840,"parent_record_offset":35812,"ancestor_record_offset":34848,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2707,"record_offset":35812,"parent_record_offset":35788,"ancestor_record_offset":34848,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2707,"record_offset":35324,"parent_record_offset":35300,"ancestor_record_offset":34848,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2707,"record_offset":35300,"parent_record_offset":35276,"ancestor_record_offset":34848,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2707,"record_offset":35788,"parent_record_offset":35764,"ancestor_record_offset":34848,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":2707,"record_offset":35252,"parent_record_offset":35228,"ancestor_record_offset":34848,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":2707,"record_offset":35228,"parent_record_offset":35196,"ancestor_record_offset":34848,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":2707,"record_offset":35468,"parent_record_offset":35444,"ancestor_record_offset":34848,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":2707,"record_offset":35196,"parent_record_offset":35164,"ancestor_record_offset":34848,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":2707,"record_offset":35104,"parent_record_offset":35020,"ancestor_record_offset":34848,"inlinee":13709,"source_line":39,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::init
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":2707,"record_offset":35164,"parent_record_offset":35104,"ancestor_record_offset":34848,"inlinee":9292,"source_line":2359,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::as_readonly
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":2707,"record_offset":35020,"parent_record_offset":34996,"ancestor_record_offset":34848,"inlinee":13705,"source_line":542,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::copy_unsafe
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":2707,"record_offset":35072,"parent_record_offset":35048,"ancestor_record_offset":34848,"inlinee":13707,"source_line":636,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::reborrow_unsafe
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":2707,"record_offset":35048,"parent_record_offset":35020,"ancestor_record_offset":34848,"inlinee":13706,"source_line":618,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::archetypes
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":2707,"record_offset":35136,"parent_record_offset":35104,"ancestor_record_offset":34848,"inlinee":13723,"source_line":263,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":2707,"record_offset":35568,"parent_record_offset":35528,"ancestor_record_offset":34848,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":2707,"record_offset":35872,"parent_record_offset":35840,"ancestor_record_offset":34848,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":2707,"record_offset":35764,"parent_record_offset":35740,"ancestor_record_offset":34848,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2707,"record_offset":35392,"parent_record_offset":35368,"ancestor_record_offset":34848,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":2707,"record_offset":35652,"parent_record_offset":35624,"ancestor_record_offset":34848,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":2707,"record_offset":35680,"parent_record_offset":35652,"ancestor_record_offset":34848,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":2707,"record_offset":35368,"parent_record_offset":35196,"ancestor_record_offset":34848,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":2707,"record_offset":35596,"parent_record_offset":35568,"ancestor_record_offset":34848,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":2707,"record_offset":35528,"parent_record_offset":35500,"ancestor_record_offset":34848,"inlinee":4446,"source_line":206,"archive":"_pdb/B/modules/02707.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_critters::butterfly::butterflies_send_to_gpu::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 394
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF0E3
// 首条位置: {"module":1169,"inlinee":61667}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::mul_vec3
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":2739,"record_offset":13180,"parent_record_offset":13148,"ancestor_record_offset":13148,"inlinee":11318,"source_line":711,"archive":"_pdb/B/modules/02739.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":2739,"record_offset":13696,"parent_record_offset":13204,"ancestor_record_offset":13148,"inlinee":11327,"source_line":33,"archive":"_pdb/B/modules/02739.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ss
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":2739,"record_offset":13536,"parent_record_offset":13480,"ancestor_record_offset":13148,"inlinee":11326,"source_line":21,"archive":"_pdb/B/modules/02739.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":2739,"record_offset":13304,"parent_record_offset":13280,"ancestor_record_offset":13148,"inlinee":11323,"source_line":81,"archive":"_pdb/B/modules/02739.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":2739,"record_offset":13336,"parent_record_offset":13252,"ancestor_record_offset":13148,"inlinee":11324,"source_line":1016,"archive":"_pdb/B/modules/02739.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sub_ps
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":2739,"record_offset":13400,"parent_record_offset":13204,"ancestor_record_offset":13148,"inlinee":11325,"source_line":57,"archive":"_pdb/B/modules/02739.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3a
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":2739,"record_offset":13204,"parent_record_offset":13180,"ancestor_record_offset":13148,"inlinee":11319,"source_line":788,"archive":"_pdb/B/modules/02739.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":2739,"record_offset":13628,"parent_record_offset":13204,"ancestor_record_offset":13148,"inlinee":11329,"source_line":229,"archive":"_pdb/B/modules/02739.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_in_x
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/sse2.rs
//     evidence={"module":2739,"record_offset":13280,"parent_record_offset":13252,"ancestor_record_offset":13148,"inlinee":11322,"source_line":42,"archive":"_pdb/B/modules/02739.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_into_m128
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/sse2.rs
//     evidence={"module":2739,"record_offset":13252,"parent_record_offset":13204,"ancestor_record_offset":13148,"inlinee":11321,"source_line":66,"archive":"_pdb/B/modules/02739.jsonl.gz"}
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
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 11; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::Butterfly> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::Butterfly> >,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::Butterfly> >,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::ButterflyAudio> >,tuple$<> >)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::ButterflyAudio> >,tuple$<> >"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::ButterflyAudio> >,tuple$<> >
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::Velocity>,ref_mut$<country_core::resources::critters::Butterfly>,ref_mut$<country_core::resources::critters::CritterGoal> >,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::critters::CritterRng>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::Velocity>,ref_mut$<country_core::resources::critters::Butterfly>,ref_mut$<country_core::resources::critters::CritterGoal> >,tuple$<> >", "bevy_ecs::change_detection::ResMut<country_core::resources::critters::CritterRng>", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::Velocity>,ref_mut$<country_core::resources::critters::Butterfly>,ref_mut$<country_core::resources::critters::CritterGoal> >,tuple$<> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::critters::CritterRng>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::Butterfly>,ref$<country_core::resources::critters::CritterGoal>,ref_mut$<country_core::resources::critters::CritterVelocityTarget>,ref_mut$<country_core::resources::critters::CritterVelocityMultiplier> >,tuple$<> >)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::Butterfly>,ref$<country_core::resources::critters::CritterGoal>,ref_mut$<country_core::resources::critters::CritterVelocityTarget>,ref_mut$<country_core::resources::critters::CritterVelocityMultiplier> >,tuple$<> >"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::Butterfly>,ref$<country_core::resources::critters::CritterGoal>,ref_mut$<country_core::resources::critters::CritterVelocityTarget>,ref_mut$<country_core::resources::critters::CritterVelocityMultiplier> >,tuple$<> >
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
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref_mut$<country_core::resources::critters::sheep::SheepAudio> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref_mut$<country_core::resources::critters::sheep::SheepAudio> >,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref_mut$<country_core::resources::critters::sheep::SheepAudio> >,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<utils::components::transform::Transform>,ref$<enum2$<country_core::resources::critters::DuckState> > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::system::system_param::Local<std::collections::hash::map::HashMap<bevy_ecs::entity::Entity,system_critters::duck::PrevRipple,std::hash::random::RandomState> >)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<utils::components::transform::Transform>,ref$<enum2$<country_core::resources::critters::DuckState> > >,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::system::system_param::Local<std::collections::hash::map::HashMap<bevy_ecs::entity::Entity,system_critters::duck::PrevRipple,std::hash::random::RandomState> >"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<utils::components::transform::Transform>,ref$<enum2$<country_core::resources::critters::DuckState> > >,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<std::collections::hash::map::HashMap<bevy_ecs::entity::Entity,system_critters::duck::PrevRipple,std::hash::random::RandomState> >
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::Firefly>,ref$<country_core::resources::critters::CritterGoal>,ref_mut$<country_core::resources::critters::CritterVelocityTarget>,ref_mut$<country_core::resources::critters::CritterVelocityMultiplier> >,tuple$<> >)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::Firefly>,ref$<country_core::resources::critters::CritterGoal>,ref_mut$<country_core::resources::critters::CritterVelocityTarget>,ref_mut$<country_core::resources::critters::CritterVelocityMultiplier> >,tuple$<> >"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::Firefly>,ref$<country_core::resources::critters::CritterGoal>,ref_mut$<country_core::resources::critters::CritterVelocityTarget>,ref_mut$<country_core::resources::critters::CritterVelocityMultiplier> >,tuple$<> >
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: system_critters::butterfly::butterflies_send_to_gpu
// 信息 ID: d633008e5983509b17df9d52
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 75081655efbd10589c88ee75: void (*)(bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::Butterfly> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::Butterfly> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),void (*)(bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::Butterfly> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)> > @ 0xf7c31c -> 0x1d87140 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::Butterfly> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),void (*)(bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::Butterfly> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)> > > @ 0x1d9c785 -> 0x1d87140 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_critters::butterfly::butterflies_send_to_gpu::closure$0
// 信息 ID: 3d9caecf735f5912b727e47f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_critters::butterfly::butterflies_send_to_gpu", "captures": "unknown"}
// [闭包环境类型引用] system_critters::butterfly::butterflies_send_to_gpu::closure_env$0 <- core::iter::adapters::map::impl$2::next<country_core::systems::render_loop::resources::critters::ButterflySsbo,bevy_ecs::query::iter::QueryIter<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::Butterfly> >,tuple$<> >,system_critters::butterfly::butterflies_send_to_gpu::closure_env$0>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_critters::butterfly::butterflies_send_to_gpu::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<country_core::systems::render_loop::resources::critters::ButterflySsbo,core::iter::adapters::map::Map<bevy_ecs::query::iter::QueryIter<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::Butterfly> >,tuple$<> >,system_critters::butterfly::butterflies_send_to_gpu::closure_env$0> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::next<country_core::systems::render_loop::resources::critters::ButterflySsbo,bevy_ecs::query::iter::QueryIter<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::Butterfly> >,tuple$<> >,system_critters::butterfly::butterflies_send_to_gpu::closure_env$0>; module=2739, inline_record=13148; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<country_core::systems::render_loop::resources::critters::ButterflySsbo,core::iter::adapters::map::Map<bevy_ecs::query::iter::QueryIter<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::Butterfly> >,tuple$<> >,system_critters::butterfly::butterflies_send_to_gpu::closure_env$0> >; module=2739, inline_record=50008; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_critters::butterfly::set_butterfly_velocities_towards_goals
// 信息 ID: 007fc63abeaec7107da00410
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] ce1bfdaee8193447c8deea3c: void (*)(bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::Butterfly>,ref$<country_core::resources::critters::CritterGoal>,ref_mut$<country_core::resources::critters::CritterVelocityTarget>,ref_mut$<country_core::resources::critters::CritterVelocityMultiplier> >,tuple$<> >)
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::Butterfly>,ref$<country_core::resources::critters::CritterGoal>,ref_mut$<country_core::resources::critters::CritterVelocityTarget>,ref_mut$<country_core::resources::critters::CritterVelocityMultiplier> >,tuple$<> >),void (*)(bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::Butterfly>,ref$<country_core::resources::critters::CritterGoal>,ref_mut$<country_core::resources::critters::CritterVelocityTarget>,ref_mut$<country_core::resources::critters::CritterVelocityMultiplier> >,tuple$<> >)> > @ 0xf7bf05 -> 0x1d867b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::Butterfly>,ref$<country_core::resources::critters::CritterGoal>,ref_mut$<country_core::resources::critters::CritterVelocityTarget>,ref_mut$<country_core::resources::critters::CritterVelocityMultiplier> >,tuple$<> >),void (*)(bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::Butterfly>,ref$<country_core::resources::critters::CritterGoal>,ref_mut$<country_core::resources::critters::CritterVelocityTarget>,ref_mut$<country_core::resources::critters::CritterVelocityMultiplier> >,tuple$<> >)> > > @ 0x1d9fd7e -> 0x1d867b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::Butterfly>,ref$<country_core::resources::critters::CritterGoal>,ref_mut$<country_core::resources::critters::CritterVelocityTarget>,ref_mut$<country_core::resources::critters::CritterVelocityMultiplier> >,tuple$<> >),void (*)(bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::Butterfly>,ref$<country_core::resources::critters::CritterGoal>,ref_mut$<country_core::resources::critters::CritterVelocityTarget>,ref_mut$<country_core::resources::critters::CritterVelocityMultiplier> >,tuple$<> >)> > > @ 0x1da550e -> 0x1d867b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_critters::butterfly::update_butterfly_audio
// 信息 ID: edb377dc0ea53caef658b617
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] a9c34386f1f131e2c1b19506: void (*)(bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::ButterflyAudio> >,tuple$<> >)
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::ButterflyAudio> >,tuple$<> >),void (*)(bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::ButterflyAudio> >,tuple$<> >)> > @ 0xf7c225 -> 0x1d86eb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::ButterflyAudio> >,tuple$<> >),void (*)(bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::ButterflyAudio> >,tuple$<> >)> > > @ 0x1d9ec1e -> 0x1d86eb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::ButterflyAudio> >,tuple$<> >),void (*)(bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::ButterflyAudio> >,tuple$<> >)> > > @ 0x1da5c6e -> 0x1d86eb0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_critters::butterfly::update_butterfly_goals
// 信息 ID: 63865b1bdc024017316b0e49
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 86ba0f51fc4b045d0de06ac6: void (*)(bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::Velocity>,ref_mut$<country_core::resources::critters::Butterfly>,ref_mut$<country_core::resources::critters::CritterGoal> >,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::critters::CritterRng>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::Velocity>,ref_mut$<country_core::resources::critters::Butterfly>,ref_mut$<country_core::resources::critters::CritterGoal> >,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::critters::CritterRng>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>),void (*)(bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::Velocity>,ref_mut$<country_core::resources::critters::Butterfly>,ref_mut$<country_core::resources::critters::CritterGoal> >,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::critters::CritterRng>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)> > @ 0xf7c07e -> 0x1d86060 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::Velocity>,ref_mut$<country_core::resources::critters::Butterfly>,ref_mut$<country_core::resources::critters::CritterGoal> >,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::critters::CritterRng>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>),void (*)(bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::Velocity>,ref_mut$<country_core::resources::critters::Butterfly>,ref_mut$<country_core::resources::critters::CritterGoal> >,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::critters::CritterRng>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)> > > @ 0x1da39e5 -> 0x1d86060 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_critters::butterfly::update_butterfly_goals::closure$0
// 信息 ID: 221dc0d5114583621ddef83c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_critters::butterfly::update_butterfly_goals", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_critters::butterfly::update_butterfly_goals; module=2707, inline_record=21120; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_critters::butterfly::update_butterfly_states
// 信息 ID: 1ce46a546e7c7f0f694305f3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 9d5cf95ffc1e7532999cfee7: void (*)(bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::Velocity>,ref_mut$<country_core::resources::critters::Butterfly>,ref_mut$<country_core::resources::critters::ButterflyAudio>,ref$<country_core::resources::critters::CritterGoal> >,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::critters::CritterRng>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::sheep::Sheep> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::Velocity>,ref_mut$<country_core::resources::critters::Butterfly>,ref_mut$<country_core::resources::critters::ButterflyAudio>,ref$<country_core::resources::critters::CritterGoal> >,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::critters::CritterRng>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::sheep::Sheep> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>),void (*)(bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::Velocity>,ref_mut$<country_core::resources::critters::Butterfly>,ref_mut$<country_core::resources::critters::ButterflyAudio>,ref$<country_core::resources::critters::CritterGoal> >,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::critters::CritterRng>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::sheep::Sheep> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>)> > @ 0xf7bce5 -> 0x1d84d40 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::Velocity>,ref_mut$<country_core::resources::critters::Butterfly>,ref_mut$<country_core::resources::critters::ButterflyAudio>,ref$<country_core::resources::critters::CritterGoal> >,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::critters::CritterRng>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::sheep::Sheep> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>),void (*)(bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::Velocity>,ref_mut$<country_core::resources::critters::Butterfly>,ref_mut$<country_core::resources::critters::ButterflyAudio>,ref$<country_core::resources::critters::CritterGoal> >,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::critters::CritterRng>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::sheep::Sheep> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>)> > > @ 0x1d9d2e4 -> 0x1d84d40 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_critters::butterfly::update_butterfly_states::closure$0
// 信息 ID: 70d3975e4788c624da670f10
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_critters::butterfly::update_butterfly_states", "captures": "unknown"}
// [闭包环境类型引用] system_critters::butterfly::update_butterfly_states::closure_env$0 <- core::iter::adapters::map::impl$2::next<glam::f32::vec2::Vec2,bevy_ecs::query::iter::QueryIter<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::sheep::Sheep> >,tuple$<> >,system_critters::butterfly::update_butterfly_states::closure_env$0>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_critters::butterfly::update_butterfly_states::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::vec2::Vec2,core::iter::adapters::map::Map<bevy_ecs::query::iter::QueryIter<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::sheep::Sheep> >,tuple$<> >,system_critters::butterfly::update_butterfly_states::closure_env$0> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::next<glam::f32::vec2::Vec2,bevy_ecs::query::iter::QueryIter<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::sheep::Sheep> >,tuple$<> >,system_critters::butterfly::update_butterfly_states::closure_env$0>; module=2739, inline_record=18184; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_critters::butterfly::update_butterfly_states::closure$1
// 信息 ID: 674a09f29f75c0669e984c3b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_critters::butterfly::update_butterfly_states", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_critters::butterfly::update_butterfly_states; module=2707, inline_record=11684; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===

// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/systems/time_of_day.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 1226711a5d122b4598681adfa264fb505bbf289409926cae59c4d3f3bb8b409b
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: time_of_day.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 553
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
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.29.3"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"}]
// use itertools; // namespace candidate
//   package identities: [{"name":"itertools","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.0"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.1"}]
// use std; // namespace candidate
//   package identities: []
// use suncalc; // namespace candidate
//   package identities: [{"name":"suncalc","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.0"}]
// use time; // namespace candidate
//   package identities: [{"name":"time","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.3.36"}]
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::time_of_day::time_of_day
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":553,"record_offset":44580,"rva":10957920,"va":5379667040}
// 编译实例 RVAs: 0xa73460
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14089aef0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: time_of_day.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1936528","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379667040,"raw_hex":"e87073e2ff","size":5,"soft_fail":false,"target_va":5377732336,"va":5379668859,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c904a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: time_of_day.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2213264","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379667040,"raw_hex":"e890c52100","size":5,"soft_fail":false,"target_va":5381883040,"va":5379669771,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428f87d0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: time_of_day.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t31998840","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379667040,"raw_hex":"e87843e801","size":5,"soft_fail":false,"target_va":5411669968,"va":5379671123,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923e70
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: time_of_day.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32178348","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379667040,"raw_hex":"e8ac00eb01","size":5,"soft_fail":false,"target_va":5411847792,"va":5379669439,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923e80
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: time_of_day.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32178265","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379667040,"raw_hex":"e85900eb01","size":5,"soft_fail":false,"target_va":5411847808,"va":5379669538,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923eb0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: time_of_day.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32178745","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379667040,"raw_hex":"e83902eb01","size":5,"soft_fail":false,"target_va":5411847856,"va":5379669106,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923ec0
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: time_of_day.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32179857","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379667040,"raw_hex":"e89106eb01","size":5,"soft_fail":false,"target_va":5411847872,"va":5379668010,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923f10
//     owner=unresolved_namespace; count=5
//     target_aliases=1 candidates; complete list: time_of_day.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32179994","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379667040,"raw_hex":"e81a07eb01","size":5,"soft_fail":false,"target_va":5411847952,"va":5379667953,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923fc0
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: time_of_day.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32178656","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379667040,"raw_hex":"e8e001eb01","size":5,"soft_fail":false,"target_va":5411848128,"va":5379669467,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923fd0
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: time_of_day.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32178613","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379667040,"raw_hex":"e8b501eb01","size":5,"soft_fail":false,"target_va":5411848144,"va":5379669526,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31870394","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379667040,"raw_hex":"e8ba4de601","size":5,"soft_fail":false,"target_va":5411541584,"va":5379671185,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31871607","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379667040,"raw_hex":"e87752e601","size":5,"soft_fail":false,"target_va":5411542880,"va":5379671268,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-6976","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379667040,"raw_hex":"e8c0e4ffff","size":5,"soft_fail":false,"target_va":5379660240,"va":5379667211,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t31876074","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379667040,"raw_hex":"e8ea63e601","size":5,"soft_fail":false,"target_va":5411543360,"va":5379667281,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::directional_light::DirectionalLight::set_direction_towards_light
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t673275","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379667040,"raw_hex":"e8fb450a00","size":5,"soft_fail":false,"target_va":5380343856,"va":5379670576,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::audio_resources::AudioCoreInner::set_parameter
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-129489","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379667040,"raw_hex":"e82f06feff","size":5,"soft_fail":false,"target_va":5379538352,"va":5379667836,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::time_of_day::TodSequence::snapshot_at_tod
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t600428","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379667040,"raw_hex":"e86c290900","size":5,"soft_fail":false,"target_va":5380270032,"va":5379669599,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::time_of_day::remap_hour_to_fmod_hour
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2715","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379667040,"raw_hex":"e89b0a0000","size":5,"soft_fail":false,"target_va":5379671952,"va":5379669232,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t1717801","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379667040,"raw_hex":"e829361a00","size":5,"soft_fail":false,"target_va":5381385536,"va":5379667730,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t1722990","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379667040,"raw_hex":"e86e4a1a00","size":5,"soft_fail":false,"target_va":5381390640,"va":5379667645,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t1715472","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379667040,"raw_hex":"e8102d1a00","size":5,"soft_fail":false,"target_va":5381386480,"va":5379671003,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t1717522","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379667040,"raw_hex":"e812351a00","size":5,"soft_fail":false,"target_va":5381384704,"va":5379667177,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::LocalKey$LT$T$GT$::with
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t9153437","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379667040,"raw_hex":"e89dab8b00","size":5,"soft_fail":false,"target_va":5388825296,"va":5379671854,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31865926","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379667040,"raw_hex":"e8463ce601","size":5,"soft_fail":false,"target_va":5411537104,"va":5379671173,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] suncalc::get_position
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"suncalc","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t13858579","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379667040,"raw_hex":"e81377d300","size":5,"soft_fail":false,"target_va":5393527984,"va":5379669400,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] time::offset_date_time::OffsetDateTime::unix_timestamp
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"time","version":"0.3.36","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2343425","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379667040,"raw_hex":"e801c22300","size":5,"soft_fail":false,"target_va":5382012752,"va":5379669322,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::time_continuous_lerp_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2213657","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379667040,"raw_hex":"e819c72100","size":5,"soft_fail":false,"target_va":5381883504,"va":5379669842,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=14
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":553,"record_offset":47532,"parent_record_offset":44580,"ancestor_record_offset":44580,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::iterators::impl$0::last
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":553,"record_offset":48788,"parent_record_offset":44580,"ancestor_record_offset":44580,"inlinee":10715,"source_line":26,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::reader::EventReader::read
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/reader.rs
//     evidence={"module":553,"record_offset":48224,"parent_record_offset":44580,"ancestor_record_offset":44580,"inlinee":8034,"source_line":27,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_direct] bevy_input::button_input::ButtonInput::just_pressed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_input","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_input-0.16.0/src/button_input.rs
//     evidence={"module":553,"record_offset":49728,"parent_record_offset":44580,"ancestor_record_offset":44580,"inlinee":26137,"source_line":188,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::abs
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":553,"record_offset":47652,"parent_record_offset":44580,"ancestor_record_offset":44580,"inlinee":10511,"source_line":1446,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":553,"record_offset":47896,"parent_record_offset":44580,"ancestor_record_offset":44580,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":553,"record_offset":49924,"parent_record_offset":44580,"ancestor_record_offset":44580,"inlinee":4754,"source_line":912,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_direct] core::ops::range::Range::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":553,"record_offset":50864,"parent_record_offset":44580,"ancestor_record_offset":44580,"inlinee":7522,"source_line":120,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":553,"record_offset":51500,"parent_record_offset":44580,"ancestor_record_offset":44580,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":553,"record_offset":52556,"parent_record_offset":44580,"ancestor_record_offset":44580,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::audio_resources::AudioCore::set_parameter
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/audio_resources.rs
//     evidence={"module":553,"record_offset":47232,"parent_record_offset":44580,"ancestor_record_offset":44580,"inlinee":27248,"source_line":314,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::session::ActiveSession::is_winter
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":553,"record_offset":50188,"parent_record_offset":44580,"ancestor_record_offset":44580,"inlinee":7619,"source_line":274,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::time_of_day::TimeOfDay::hour
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/time_of_day.rs
//     evidence={"module":553,"record_offset":50040,"parent_record_offset":44580,"ancestor_record_offset":44580,"inlinee":31610,"source_line":59,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::time_of_day::suncalc_pos_to_vec
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/time_of_day.rs
//     evidence={"module":553,"record_offset":50480,"parent_record_offset":44580,"ancestor_record_offset":44580,"inlinee":31612,"source_line":348,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::mat3::Mat3::from_axis_angle
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/mat3.rs
//     evidence={"module":553,"record_offset":50632,"parent_record_offset":44580,"ancestor_record_offset":44580,"inlinee":31615,"source_line":235,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::from_rotation_y
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":553,"record_offset":50748,"parent_record_offset":44580,"ancestor_record_offset":44580,"inlinee":25694,"source_line":168,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":553,"record_offset":51036,"parent_record_offset":44580,"ancestor_record_offset":44580,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":553,"record_offset":46248,"parent_record_offset":44580,"ancestor_record_offset":44580,"inlinee":4416,"source_line":589,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":553,"record_offset":44816,"parent_record_offset":44580,"ancestor_record_offset":44580,"inlinee":4368,"source_line":626,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":553,"record_offset":45272,"parent_record_offset":44580,"ancestor_record_offset":44580,"inlinee":4385,"source_line":659,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::exp
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":553,"record_offset":47460,"parent_record_offset":44580,"ancestor_record_offset":44580,"inlinee":19230,"source_line":396,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::exp2
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":553,"record_offset":50072,"parent_record_offset":44580,"ancestor_record_offset":44580,"inlinee":7676,"source_line":421,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::rem_euclid
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":553,"record_offset":47996,"parent_record_offset":44580,"ancestor_record_offset":44580,"inlinee":7617,"source_line":285,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_direct] time::date::Date::from_ordinal_date
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"time","version":"0.3.36","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/time-0.3.36/src/date.rs
//     evidence={"module":553,"record_offset":50332,"parent_record_offset":44580,"ancestor_record_offset":44580,"inlinee":13949,"source_line":157,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_direct] time::primitive_date_time::PrimitiveDateTime::assume_utc
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"time","version":"0.3.36","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/time-0.3.36/src/primitive_date_time.rs
//     evidence={"module":553,"record_offset":50396,"parent_record_offset":44580,"ancestor_record_offset":44580,"inlinee":23313,"source_line":491,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_direct] utils::comparative_space::impl$3::project_unbounded
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":553,"record_offset":47428,"parent_record_offset":44580,"ancestor_record_offset":44580,"inlinee":14890,"source_line":154,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_direct] utils::comparative_space::impl$3::subtract
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":553,"record_offset":47488,"parent_record_offset":44580,"ancestor_record_offset":44580,"inlinee":31608,"source_line":161,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":553,"record_offset":47292,"parent_record_offset":47232,"ancestor_record_offset":44580,"inlinee":5345,"source_line":2205,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":553,"record_offset":48460,"parent_record_offset":48436,"ancestor_record_offset":44580,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":553,"record_offset":48436,"parent_record_offset":48336,"ancestor_record_offset":44580,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=14
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":553,"record_offset":47560,"parent_record_offset":47532,"ancestor_record_offset":44580,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::event_cursor::EventCursor::read_with_id
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/event_cursor.rs
//     evidence={"module":553,"record_offset":48304,"parent_record_offset":48256,"ancestor_record_offset":44580,"inlinee":8037,"source_line":88,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::iterators::EventIteratorWithId::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":553,"record_offset":48336,"parent_record_offset":48304,"ancestor_record_offset":44580,"inlinee":8039,"source_line":54,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::iterators::impl$3::last
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":553,"record_offset":48820,"parent_record_offset":48788,"ancestor_record_offset":44580,"inlinee":10716,"source_line":112,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$19::deref_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/system_param.rs
//     evidence={"module":553,"record_offset":48256,"parent_record_offset":48224,"ancestor_record_offset":44580,"inlinee":10713,"source_line":1234,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] bevy_platform::collections::hash_set::HashSet::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_platform","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_platform-0.16.0/src/collections/hash_set.rs
//     evidence={"module":553,"record_offset":49752,"parent_record_offset":49728,"ancestor_record_offset":44580,"inlinee":26138,"source_line":730,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":553,"record_offset":46584,"parent_record_offset":46556,"ancestor_record_offset":44580,"inlinee":4432,"source_line":1817,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":553,"record_offset":46636,"parent_record_offset":46612,"ancestor_record_offset":44580,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":553,"record_offset":46612,"parent_record_offset":46584,"ancestor_record_offset":44580,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":553,"record_offset":46532,"parent_record_offset":46504,"ancestor_record_offset":44580,"inlinee":4429,"source_line":1089,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":553,"record_offset":46556,"parent_record_offset":46532,"ancestor_record_offset":44580,"inlinee":4430,"source_line":1121,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":553,"record_offset":46728,"parent_record_offset":46556,"ancestor_record_offset":44580,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$44::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":553,"record_offset":46840,"parent_record_offset":46816,"ancestor_record_offset":44580,"inlinee":4443,"source_line":1808,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::last
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":553,"record_offset":48852,"parent_record_offset":48820,"ancestor_record_offset":44580,"inlinee":10717,"source_line":176,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":553,"record_offset":46660,"parent_record_offset":46636,"ancestor_record_offset":44580,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":553,"record_offset":48564,"parent_record_offset":48540,"ancestor_record_offset":44580,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":553,"record_offset":48380,"parent_record_offset":48336,"ancestor_record_offset":44580,"inlinee":8040,"source_line":1859,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":553,"record_offset":44972,"parent_record_offset":44940,"ancestor_record_offset":44580,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":553,"record_offset":51068,"parent_record_offset":51036,"ancestor_record_offset":44580,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":553,"record_offset":46400,"parent_record_offset":46356,"ancestor_record_offset":44580,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":553,"record_offset":50888,"parent_record_offset":50864,"ancestor_record_offset":44580,"inlinee":4982,"source_line":817,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":553,"record_offset":48880,"parent_record_offset":48852,"ancestor_record_offset":44580,"inlinee":4518,"source_line":1447,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":553,"record_offset":47264,"parent_record_offset":47232,"ancestor_record_offset":44580,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":553,"record_offset":49824,"parent_record_offset":49800,"ancestor_record_offset":44580,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$39::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":553,"record_offset":48940,"parent_record_offset":48820,"ancestor_record_offset":44580,"inlinee":4927,"source_line":2486,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":553,"record_offset":45220,"parent_record_offset":45192,"ancestor_record_offset":44580,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":553,"record_offset":47188,"parent_record_offset":46356,"ancestor_record_offset":44580,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":553,"record_offset":46792,"parent_record_offset":46504,"ancestor_record_offset":44580,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":553,"record_offset":45684,"parent_record_offset":45660,"ancestor_record_offset":44580,"inlinee":4410,"source_line":533,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":553,"record_offset":45660,"parent_record_offset":45636,"ancestor_record_offset":44580,"inlinee":4409,"source_line":686,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":553,"record_offset":45868,"parent_record_offset":45840,"ancestor_record_offset":44580,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":553,"record_offset":46684,"parent_record_offset":46660,"ancestor_record_offset":44580,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":553,"record_offset":48492,"parent_record_offset":48336,"ancestor_record_offset":44580,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":553,"record_offset":48540,"parent_record_offset":48516,"ancestor_record_offset":44580,"inlinee":8042,"source_line":364,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":553,"record_offset":48516,"parent_record_offset":48492,"ancestor_record_offset":44580,"inlinee":8041,"source_line":537,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":553,"record_offset":45612,"parent_record_offset":45588,"ancestor_record_offset":44580,"inlinee":4407,"source_line":80,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":553,"record_offset":45636,"parent_record_offset":45612,"ancestor_record_offset":44580,"inlinee":4408,"source_line":118,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":553,"record_offset":45940,"parent_record_offset":45912,"ancestor_record_offset":44580,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$183::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":553,"record_offset":45588,"parent_record_offset":45472,"ancestor_record_offset":44580,"inlinee":4405,"source_line":392,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":553,"record_offset":44940,"parent_record_offset":44912,"ancestor_record_offset":44580,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":553,"record_offset":45300,"parent_record_offset":45272,"ancestor_record_offset":44580,"inlinee":4386,"source_line":1488,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":553,"record_offset":45440,"parent_record_offset":45408,"ancestor_record_offset":44580,"inlinee":4396,"source_line":128,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":553,"record_offset":45408,"parent_record_offset":45372,"ancestor_record_offset":44580,"inlinee":4394,"source_line":218,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":553,"record_offset":45348,"parent_record_offset":45324,"ancestor_record_offset":44580,"inlinee":4390,"source_line":315,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":553,"record_offset":45372,"parent_record_offset":45348,"ancestor_record_offset":44580,"inlinee":4392,"source_line":709,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":553,"record_offset":45324,"parent_record_offset":45300,"ancestor_record_offset":44580,"inlinee":4388,"source_line":798,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":553,"record_offset":45912,"parent_record_offset":45372,"ancestor_record_offset":44580,"inlinee":4400,"source_line":659,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":553,"record_offset":44912,"parent_record_offset":44880,"ancestor_record_offset":44580,"inlinee":4373,"source_line":433,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":553,"record_offset":44880,"parent_record_offset":44852,"ancestor_record_offset":44580,"inlinee":4372,"source_line":463,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":553,"record_offset":45072,"parent_record_offset":45040,"ancestor_record_offset":44580,"inlinee":4380,"source_line":501,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":553,"record_offset":45168,"parent_record_offset":45072,"ancestor_record_offset":44580,"inlinee":4381,"source_line":521,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":553,"record_offset":45040,"parent_record_offset":45016,"ancestor_record_offset":44580,"inlinee":4379,"source_line":533,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":553,"record_offset":44852,"parent_record_offset":44816,"ancestor_record_offset":44580,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":553,"record_offset":45192,"parent_record_offset":45168,"ancestor_record_offset":44580,"inlinee":4383,"source_line":189,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":553,"record_offset":45472,"parent_record_offset":45440,"ancestor_record_offset":44580,"inlinee":4398,"source_line":79,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":553,"record_offset":45756,"parent_record_offset":45472,"ancestor_record_offset":44580,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":553,"record_offset":45728,"parent_record_offset":45472,"ancestor_record_offset":44580,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":553,"record_offset":45532,"parent_record_offset":45472,"ancestor_record_offset":44580,"inlinee":4403,"source_line":24,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::GladeType::is_winter
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":553,"record_offset":50212,"parent_record_offset":50188,"ancestor_record_offset":44580,"inlinee":26885,"source_line":118,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::sin_cos
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/math.rs
//     evidence={"module":553,"record_offset":50656,"parent_record_offset":50632,"ancestor_record_offset":44580,"inlinee":24915,"source_line":167,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::contains_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/map.rs
//     evidence={"module":553,"record_offset":49800,"parent_record_offset":49776,"ancestor_record_offset":44580,"inlinee":4474,"source_line":1414,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/set.rs
//     evidence={"module":553,"record_offset":49776,"parent_record_offset":49752,"ancestor_record_offset":44580,"inlinee":4472,"source_line":862,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":553,"record_offset":46284,"parent_record_offset":46248,"ancestor_record_offset":44580,"inlinee":4418,"source_line":425,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":553,"record_offset":51564,"parent_record_offset":51532,"ancestor_record_offset":44580,"inlinee":4966,"source_line":601,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":553,"record_offset":52080,"parent_record_offset":51820,"ancestor_record_offset":44580,"inlinee":4968,"source_line":602,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":553,"record_offset":46428,"parent_record_offset":46400,"ancestor_record_offset":44580,"inlinee":4423,"source_line":90,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":553,"record_offset":46504,"parent_record_offset":46356,"ancestor_record_offset":44580,"inlinee":4427,"source_line":429,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":553,"record_offset":46764,"parent_record_offset":46504,"ancestor_record_offset":44580,"inlinee":4441,"source_line":591,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sin
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":553,"record_offset":50704,"parent_record_offset":50680,"ancestor_record_offset":44580,"inlinee":14894,"source_line":651,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sin_cos
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":553,"record_offset":50680,"parent_record_offset":50656,"ancestor_record_offset":44580,"inlinee":24916,"source_line":860,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] std::f64::impl$0::cos
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f64.rs
//     evidence={"module":553,"record_offset":50516,"parent_record_offset":50480,"ancestor_record_offset":44580,"inlinee":31613,"source_line":675,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] std::f64::impl$0::sin
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f64.rs
//     evidence={"module":553,"record_offset":50572,"parent_record_offset":50480,"ancestor_record_offset":44580,"inlinee":31614,"source_line":651,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":553,"record_offset":46456,"parent_record_offset":46428,"ancestor_record_offset":44580,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":553,"record_offset":46356,"parent_record_offset":46320,"ancestor_record_offset":44580,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":553,"record_offset":46320,"parent_record_offset":46284,"ancestor_record_offset":44580,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] time::date::Date::__from_ordinal_date_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"time","version":"0.3.36","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/time-0.3.36/src/date.rs
//     evidence={"module":553,"record_offset":50364,"parent_record_offset":50332,"ancestor_record_offset":44580,"inlinee":13950,"source_line":85,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] time::offset_date_time::OffsetDateTime::new_in_offset
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"time","version":"0.3.36","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/time-0.3.36/src/offset_date_time.rs
//     evidence={"module":553,"record_offset":50444,"parent_record_offset":50420,"ancestor_record_offset":44580,"inlinee":14809,"source_line":143,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] time::primitive_date_time::PrimitiveDateTime::assume_offset
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"time","version":"0.3.36","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/time-0.3.36/src/primitive_date_time.rs
//     evidence={"module":553,"record_offset":50420,"parent_record_offset":50396,"ancestor_record_offset":44580,"inlinee":23314,"source_line":477,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::time_of_day::remap_hour_to_fmod_hour
// 记录形式: compiled_procedure; instances=1
// 源码行: 287
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":553,"record_offset":52700,"rva":10962832,"va":5379671952}
// 编译实例 RVAs: 0xa74790
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: time_of_day.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4590534","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379671952,"raw_hex":"e83af4b9ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5379672017,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: time_of_day.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4591257","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379671952,"raw_hex":"e867f1b9ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5379672756,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c10
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: time_of_day.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32174902","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379671952,"raw_hex":"e836f3ea01","size":5,"soft_fail":false,"target_va":5411847184,"va":5379672277,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t31868091","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379671952,"raw_hex":"e8bb44e601","size":5,"soft_fail":false,"target_va":5411540992,"va":5379672896,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t-4343849","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379671952,"raw_hex":"e8d7b7bdff","size":5,"soft_fail":false,"target_va":5375328336,"va":5379672180,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::vec::Vec$LT$T$C$A$GT$::insert::assert_failed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t31868281","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379671952,"raw_hex":"e87945e601","size":5,"soft_fail":false,"target_va":5411541121,"va":5379672835,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::fit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2215463","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379671952,"raw_hex":"e827ce2100","size":5,"soft_fail":false,"target_va":5381888176,"va":5379672708,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":553,"record_offset":52868,"parent_record_offset":52700,"ancestor_record_offset":52700,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":553,"record_offset":53016,"parent_record_offset":52700,"ancestor_record_offset":52700,"inlinee":7065,"source_line":526,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::insert
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":553,"record_offset":53332,"parent_record_offset":52700,"ancestor_record_offset":52700,"inlinee":7068,"source_line":1977,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":553,"record_offset":54628,"parent_record_offset":52700,"ancestor_record_offset":52700,"inlinee":4538,"source_line":85,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_direct] core::ops::range::Range::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":553,"record_offset":55824,"parent_record_offset":52700,"ancestor_record_offset":52700,"inlinee":7522,"source_line":120,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":553,"record_offset":55968,"parent_record_offset":52700,"ancestor_record_offset":52700,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":553,"record_offset":54460,"parent_record_offset":52700,"ancestor_record_offset":52700,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":553,"record_offset":52924,"parent_record_offset":52900,"ancestor_record_offset":52700,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":553,"record_offset":52948,"parent_record_offset":52924,"ancestor_record_offset":52700,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":553,"record_offset":56156,"parent_record_offset":56132,"ancestor_record_offset":52700,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":553,"record_offset":52900,"parent_record_offset":52868,"ancestor_record_offset":52700,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":553,"record_offset":56132,"parent_record_offset":56040,"ancestor_record_offset":52700,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":553,"record_offset":53396,"parent_record_offset":53372,"ancestor_record_offset":52700,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":553,"record_offset":56068,"parent_record_offset":56040,"ancestor_record_offset":52700,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":553,"record_offset":56040,"parent_record_offset":56016,"ancestor_record_offset":52700,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":553,"record_offset":53444,"parent_record_offset":53420,"ancestor_record_offset":52700,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":553,"record_offset":53420,"parent_record_offset":53396,"ancestor_record_offset":52700,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":553,"record_offset":56016,"parent_record_offset":55992,"ancestor_record_offset":52700,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::into_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":553,"record_offset":53040,"parent_record_offset":53016,"ancestor_record_offset":52700,"inlinee":7066,"source_line":99,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":553,"record_offset":53372,"parent_record_offset":53332,"ancestor_record_offset":52700,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":553,"record_offset":53064,"parent_record_offset":53040,"ancestor_record_offset":52700,"inlinee":7067,"source_line":934,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":553,"record_offset":53624,"parent_record_offset":53572,"ancestor_record_offset":52700,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":553,"record_offset":53540,"parent_record_offset":53332,"ancestor_record_offset":52700,"inlinee":5297,"source_line":1895,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":553,"record_offset":55888,"parent_record_offset":55856,"ancestor_record_offset":52700,"inlinee":4984,"source_line":1943,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":553,"record_offset":55920,"parent_record_offset":55888,"ancestor_record_offset":52700,"inlinee":4986,"source_line":1788,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":553,"record_offset":53484,"parent_record_offset":53332,"ancestor_record_offset":52700,"inlinee":5303,"source_line":4520,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":553,"record_offset":54940,"parent_record_offset":54916,"ancestor_record_offset":52700,"inlinee":4570,"source_line":83,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":553,"record_offset":54988,"parent_record_offset":54964,"ancestor_record_offset":52700,"inlinee":4573,"source_line":84,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$3::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":553,"record_offset":54664,"parent_record_offset":54628,"ancestor_record_offset":52700,"inlinee":4925,"source_line":166,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::impl$0::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":553,"record_offset":55012,"parent_record_offset":54988,"ancestor_record_offset":52700,"inlinee":4575,"source_line":4031,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":553,"record_offset":56100,"parent_record_offset":56068,"ancestor_record_offset":52700,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":553,"record_offset":55856,"parent_record_offset":55824,"ancestor_record_offset":52700,"inlinee":4982,"source_line":817,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":553,"record_offset":54964,"parent_record_offset":54940,"ancestor_record_offset":52700,"inlinee":4571,"source_line":1546,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":553,"record_offset":55992,"parent_record_offset":55968,"ancestor_record_offset":52700,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":553,"record_offset":53772,"parent_record_offset":53572,"ancestor_record_offset":52700,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":553,"record_offset":54876,"parent_record_offset":54852,"ancestor_record_offset":52700,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":553,"record_offset":54796,"parent_record_offset":54752,"ancestor_record_offset":52700,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":553,"record_offset":52972,"parent_record_offset":52948,"ancestor_record_offset":52700,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":553,"record_offset":53512,"parent_record_offset":53332,"ancestor_record_offset":52700,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":553,"record_offset":54484,"parent_record_offset":54460,"ancestor_record_offset":52700,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":553,"record_offset":54852,"parent_record_offset":54828,"ancestor_record_offset":52700,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":553,"record_offset":54752,"parent_record_offset":54716,"ancestor_record_offset":52700,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":553,"record_offset":54828,"parent_record_offset":54752,"ancestor_record_offset":52700,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$31::collect_from_iter_no_buf
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":553,"record_offset":54916,"parent_record_offset":54716,"ancestor_record_offset":52700,"inlinee":4568,"source_line":374,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$7::next
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":553,"record_offset":54716,"parent_record_offset":54664,"ancestor_record_offset":52700,"inlinee":4536,"source_line":193,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::time_of_day::suncalc_pos_to_vec
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 348
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7B7C
// 首条位置: {"module":553,"inlinee":31612}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::f64::impl$0::cos
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f64.rs
//     evidence={"module":553,"record_offset":50516,"parent_record_offset":50480,"ancestor_record_offset":50480,"inlinee":31613,"source_line":675,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_direct] std::f64::impl$0::sin
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f64.rs
//     evidence={"module":553,"record_offset":50572,"parent_record_offset":50480,"ancestor_record_offset":50480,"inlinee":31614,"source_line":651,"archive":"_pdb/B/modules/00553.jsonl.gz"}
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
// 完整结构及原始类型记录: time_of_day.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: time_of_day.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: time_of_day.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 2; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TodSequence>,bevy_ecs::change_detection::Res<country_core::resources::weather::Weather>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<bevy_input::button_input::ButtonInput<enum2$<bevy_input::keyboard::KeyCode> > >,bevy_ecs::change_detection::ResMut<country_core::render::directional_light::DirectionalLight>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::render::atmosphere::DerivedAtmosphere>,bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::event::reader::EventReader<country_core::resources::session::NewSessionStartedCmd>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::TimeOfDayScreen>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TodSequence>", "bevy_ecs::change_detection::Res<country_core::resources::weather::Weather>", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::change_detection::Res<bevy_input::button_input::ButtonInput<enum2$<bevy_input::keyboard::KeyCode> > >", "bevy_ecs::change_detection::ResMut<country_core::render::directional_light::DirectionalLight>", "bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>", "bevy_ecs::change_detection::ResMut<country_core::render::atmosphere::DerivedAtmosphere>", "bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>", "bevy_ecs::event::reader::EventReader<country_core::resources::session::NewSessionStartedCmd>", "bevy_ecs::event::reader::EventReader<country_core::systems::save::load_game::LoadingGameComplete>", "bevy_ecs::change_detection::ResMut<country_core::resources::gui::TimeOfDayScreen>", "bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TodSequence>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::weather::Weather>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_input::button_input::ButtonInput<enum2$<bevy_input::keyboard::KeyCode> > >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::render::directional_light::DirectionalLight>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::render::atmosphere::DerivedAtmosphere>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::session::NewSessionStartedCmd>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::save::load_game::LoadingGameComplete>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::gui::TimeOfDayScreen>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>

// ------------------------------------------------------------------
// 函数: country_core::systems::time_of_day::remap_hour_to_fmod_hour
// 信息 ID: 0f23a226fbab2cef060aca95
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::ops::function::FnOnce::call_once<tiny_glade::gui::gui_debug::do_gui_debug::closure$0::closure$0::closure$8::closure_env$1,tuple$<ref_mut$<egui::ui::Ui> > > @ 0xb468e -> 0xa74790 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<tiny_glade::gui::gui_debug::do_tod_sequence_entry::closure$0::closure$0::closure_env$0,tuple$<ref_mut$<egui::ui::Ui> > > @ 0x147a68 -> 0xa74790 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::time_of_day::time_of_day @ 0xa73cf0 -> 0xa74790 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::time_of_day::suncalc_pos_to_vec
// 信息 ID: ef96f4818dabcce3f08b3dba
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::time_of_day::time_of_day; module=553, inline_record=50480; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::time_of_day::time_of_day
// 信息 ID: e2ee992899f34b6cee78317b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 4a8b5611ba4655840297a5f1: void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TodSequence>,bevy_ecs::change_detection::Res<country_core::resources::weather::Weather>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<bevy_input::button_input::ButtonInput<enum2$<bevy_input::keyboard::KeyCode> > >,bevy_ecs::change_detection::ResMut<country_core::render::directional_light::DirectionalLight>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::render::atmosphere::DerivedAtmosphere>,bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::event::reader::EventReader<country_core::resources::session::NewSessionStartedCmd>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::TimeOfDayScreen>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TodSequence>,bevy_ecs::change_detection::Res<country_core::resources::weather::Weather>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<bevy_input::button_input::ButtonInput<enum2$<bevy_input::keyboard::KeyCode> > >,bevy_ecs::change_detection::ResMut<country_core::render::directional_light::DirectionalLight>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::render::atmosphere::DerivedAtmosphere>,bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::event::reader::EventReader<country_core::resources::session::NewSessionStartedCmd>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::TimeOfDayScreen>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TodSequence>,bevy_ecs::change_detection::Res<country_core::resources::weather::Weather>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<bevy_input::button_input::ButtonInput<enum2$<bevy_input::keyboard::KeyCode> > >,bevy_ecs::change_detection::ResMut<country_core::render::directional_light::DirectionalLight>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::render::atmosphere::DerivedAtmosphere>,bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::event::reader::EventReader<country_core::resources::session::NewSessionStartedCmd>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::TimeOfDayScreen>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>)> > @ 0x2caf8e -> 0xa73460 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TodSequence>,bevy_ecs::change_detection::Res<country_core::resources::weather::Weather>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<bevy_input::button_input::ButtonInput<enum2$<bevy_input::keyboard::KeyCode> > >,bevy_ecs::change_detection::ResMut<country_core::render::directional_light::DirectionalLight>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::render::atmosphere::DerivedAtmosphere>,bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::event::reader::EventReader<country_core::resources::session::NewSessionStartedCmd>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::TimeOfDayScreen>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TodSequence>,bevy_ecs::change_detection::Res<country_core::resources::weather::Weather>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<bevy_input::button_input::ButtonInput<enum2$<bevy_input::keyboard::KeyCode> > >,bevy_ecs::change_detection::ResMut<country_core::render::directional_light::DirectionalLight>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::render::atmosphere::DerivedAtmosphere>,bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::event::reader::EventReader<country_core::resources::session::NewSessionStartedCmd>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::TimeOfDayScreen>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>)> > > @ 0x461f19 -> 0xa73460 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===

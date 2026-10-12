// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/country-core/src/systems/time_of_day.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: cd0ccf158a185ea8fc123700057dd020c3be701f60c9dbc754934806480354b5
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: time_of_day.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 553
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
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.30.10"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.1"}]
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
// 首条位置: {"module":553,"record_offset":51900,"rva":11372096,"va":5380081216}
// 编译实例 RVAs: 0xad8640
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140de3df0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: time_of_day.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3189953","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380081216,"raw_hex":"e8c1ac3000","size":5,"soft_fail":false,"target_va":5383273968,"va":5380084010,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1416f4de0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: time_of_day.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12694541","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380081216,"raw_hex":"e80db4c100","size":5,"soft_fail":false,"target_va":5392780768,"va":5380086222,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1417a68f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: time_of_day.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13425697","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380081216,"raw_hex":"e821dccc00","size":5,"soft_fail":false,"target_va":5393508592,"va":5380082890,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d77a00
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: time_of_day.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t36299613","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380081216,"raw_hex":"e85de32902","size":5,"soft_fail":false,"target_va":5416385024,"va":5380085406,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bd90
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: time_of_day.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t36974168","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380081216,"raw_hex":"e8582e3402","size":5,"soft_fail":false,"target_va":5417057680,"va":5380083507,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bda0
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: time_of_day.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t36974085","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380081216,"raw_hex":"e8052e3402","size":5,"soft_fail":false,"target_va":5417057696,"va":5380083606,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bdd0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: time_of_day.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t36974578","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380081216,"raw_hex":"e8f22f3402","size":5,"soft_fail":false,"target_va":5417057744,"va":5380083161,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bde0
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: time_of_day.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t36975722","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380081216,"raw_hex":"e86a343402","size":5,"soft_fail":false,"target_va":5417057760,"va":5380082033,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1be30
//     owner=unresolved_namespace; count=5
//     target_aliases=1 candidates; complete list: time_of_day.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t36975852","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380081216,"raw_hex":"e8ec343402","size":5,"soft_fail":false,"target_va":5417057840,"va":5380081983,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bee0
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: time_of_day.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t36974476","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380081216,"raw_hex":"e88c2f3402","size":5,"soft_fail":false,"target_va":5417058016,"va":5380083535,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bef0
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: time_of_day.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t36974433","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380081216,"raw_hex":"e8612f3402","size":5,"soft_fail":false,"target_va":5417058032,"va":5380083594,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t36536007","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380081216,"raw_hex":"e8c77e2d02","size":5,"soft_fail":false,"target_va":5416621408,"va":5380085396,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t36536096","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380081216,"raw_hex":"e8207f2d02","size":5,"soft_fail":false,"target_va":5416621632,"va":5380085531,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t14733","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380081216,"raw_hex":"e88d390000","size":5,"soft_fail":false,"target_va":5380096128,"va":5380081390,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t36535802","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380081216,"raw_hex":"e8fa7d2d02","size":5,"soft_fail":false,"target_va":5416621376,"va":5380085569,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::directional_light::DirectionalLight::set_direction_towards_light
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t937211","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380081216,"raw_hex":"e8fb4c0e00","size":5,"soft_fail":false,"target_va":5381022064,"va":5380084848,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::audio_resources::AudioCoreInner::set_parameter
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-398992","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380081216,"raw_hex":"e870e9f9ff","size":5,"soft_fail":false,"target_va":5379682880,"va":5380081867,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::time_of_day::TodSequence::snapshot_at_tod
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t635292","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380081216,"raw_hex":"e89cb10900","size":5,"soft_fail":false,"target_va":5380718960,"va":5380083663,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::time_of_day::remap_hour_to_fmod_hour
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3019","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380081216,"raw_hex":"e8cb0b0000","size":5,"soft_fail":false,"target_va":5380086320,"va":5380083296,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t3882040","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380081216,"raw_hex":"e8383c3b00","size":5,"soft_fail":false,"target_va":5383963792,"va":5380081747,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::__RUST_STD_INTERNAL_VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t3890547","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380081216,"raw_hex":"e8735d3b00","size":5,"soft_fail":false,"target_va":5383972224,"va":5380081672,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t3879632","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380081216,"raw_hex":"e8d0323b00","size":5,"soft_fail":false,"target_va":5383964912,"va":5380085275,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t3881462","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380081216,"raw_hex":"e8f6393b00","size":5,"soft_fail":false,"target_va":5383962816,"va":5380081349,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t36533907","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380081216,"raw_hex":"e893762d02","size":5,"soft_fail":false,"target_va":5416619360,"va":5380085448,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] suncalc::get_position
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"suncalc","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t18091920","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380081216,"raw_hex":"e8900f1401","size":5,"soft_fail":false,"target_va":5398175392,"va":5380083467,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] time::offset_date_time::OffsetDateTime::unix_timestamp
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"time","version":"0.3.36","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t3302671","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380081216,"raw_hex":"e80f653200","size":5,"soft_fail":false,"target_va":5383386064,"va":5380083388,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::time_continuous_lerp_mut
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3195436","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380081216,"raw_hex":"e82cc23000","size":5,"soft_fail":false,"target_va":5383279328,"va":5380083887,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=17
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":553,"record_offset":54648,"parent_record_offset":51900,"ancestor_record_offset":51900,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::iterators::impl$0::last
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":553,"record_offset":55932,"parent_record_offset":51900,"ancestor_record_offset":51900,"inlinee":13153,"source_line":26,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::message_reader::MessageReader::read
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_reader.rs
//     evidence={"module":553,"record_offset":55368,"parent_record_offset":51900,"ancestor_record_offset":51900,"inlinee":9339,"source_line":27,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_direct] bevy_input::button_input::ButtonInput::just_pressed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_input","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_input-0.17.3/src/button_input.rs
//     evidence={"module":553,"record_offset":56872,"parent_record_offset":51900,"ancestor_record_offset":51900,"inlinee":34554,"source_line":188,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::abs
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":553,"record_offset":54768,"parent_record_offset":51900,"ancestor_record_offset":51900,"inlinee":11731,"source_line":1569,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":553,"record_offset":55012,"parent_record_offset":51900,"ancestor_record_offset":51900,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":553,"record_offset":57012,"parent_record_offset":51900,"ancestor_record_offset":51900,"inlinee":5478,"source_line":1018,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_direct] core::ops::range::Range::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":553,"record_offset":58144,"parent_record_offset":51900,"ancestor_record_offset":51900,"inlinee":11437,"source_line":123,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":553,"record_offset":58800,"parent_record_offset":51900,"ancestor_record_offset":51900,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":553,"record_offset":59680,"parent_record_offset":51900,"ancestor_record_offset":51900,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::audio_resources::AudioCore::set_parameter
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/audio_resources.rs
//     evidence={"module":553,"record_offset":54348,"parent_record_offset":51900,"ancestor_record_offset":51900,"inlinee":29638,"source_line":314,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::session::ActiveSession::is_winter
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":553,"record_offset":57300,"parent_record_offset":51900,"ancestor_record_offset":51900,"inlinee":16149,"source_line":296,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::time_of_day::TimeOfDay::hour
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/time_of_day.rs
//     evidence={"module":553,"record_offset":57124,"parent_record_offset":51900,"ancestor_record_offset":51900,"inlinee":34557,"source_line":62,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::time_of_day::suncalc_pos_to_vec
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/time_of_day.rs
//     evidence={"module":553,"record_offset":57592,"parent_record_offset":51900,"ancestor_record_offset":51900,"inlinee":34562,"source_line":378,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::mat3::Mat3::from_axis_angle
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/mat3.rs
//     evidence={"module":553,"record_offset":57744,"parent_record_offset":51900,"ancestor_record_offset":51900,"inlinee":34564,"source_line":243,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::from_rotation_y
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs
//     evidence={"module":553,"record_offset":57860,"parent_record_offset":51900,"ancestor_record_offset":51900,"inlinee":11736,"source_line":178,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":553,"record_offset":58316,"parent_record_offset":51900,"ancestor_record_offset":51900,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":553,"record_offset":53540,"parent_record_offset":51900,"ancestor_record_offset":51900,"inlinee":8276,"source_line":589,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":553,"record_offset":52136,"parent_record_offset":51900,"ancestor_record_offset":51900,"inlinee":9267,"source_line":626,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":553,"record_offset":52528,"parent_record_offset":51900,"ancestor_record_offset":51900,"inlinee":8242,"source_line":659,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::exp
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":553,"record_offset":54576,"parent_record_offset":51900,"ancestor_record_offset":51900,"inlinee":26173,"source_line":411,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::exp2
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":553,"record_offset":57156,"parent_record_offset":51900,"ancestor_record_offset":51900,"inlinee":17937,"source_line":436,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::rem_euclid
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":553,"record_offset":55112,"parent_record_offset":51900,"ancestor_record_offset":51900,"inlinee":11231,"source_line":289,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_direct] time::date::Date::from_ordinal_date
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"time","version":"0.3.36","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/time-0.3.36/src/date.rs
//     evidence={"module":553,"record_offset":57444,"parent_record_offset":51900,"ancestor_record_offset":51900,"inlinee":17842,"source_line":157,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_direct] time::primitive_date_time::PrimitiveDateTime::assume_utc
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"time","version":"0.3.36","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/time-0.3.36/src/primitive_date_time.rs
//     evidence={"module":553,"record_offset":57508,"parent_record_offset":51900,"ancestor_record_offset":51900,"inlinee":34559,"source_line":491,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_direct] utils::comparative_space::impl$3::project_unbounded
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":553,"record_offset":54544,"parent_record_offset":51900,"ancestor_record_offset":51900,"inlinee":11236,"source_line":154,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_direct] utils::comparative_space::impl$3::subtract
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":553,"record_offset":54604,"parent_record_offset":51900,"ancestor_record_offset":51900,"inlinee":34553,"source_line":161,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$34::deref
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":553,"record_offset":54408,"parent_record_offset":54348,"ancestor_record_offset":51900,"inlinee":4944,"source_line":2421,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":553,"record_offset":55604,"parent_record_offset":55580,"ancestor_record_offset":51900,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":553,"record_offset":55580,"parent_record_offset":55480,"ancestor_record_offset":51900,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=17
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":553,"record_offset":54676,"parent_record_offset":54648,"ancestor_record_offset":51900,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::iterators::MessageIteratorWithId::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":553,"record_offset":55480,"parent_record_offset":55448,"ancestor_record_offset":51900,"inlinee":9344,"source_line":54,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::iterators::impl$3::last
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":553,"record_offset":55964,"parent_record_offset":55932,"ancestor_record_offset":51900,"inlinee":10306,"source_line":112,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::message_cursor::MessageCursor::read_with_id
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_cursor.rs
//     evidence={"module":553,"record_offset":55448,"parent_record_offset":55400,"ancestor_record_offset":51900,"inlinee":9342,"source_line":89,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$14::deref_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/system_param.rs
//     evidence={"module":553,"record_offset":55400,"parent_record_offset":55368,"ancestor_record_offset":51900,"inlinee":9614,"source_line":1064,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] bevy_platform::collections::hash_set::HashSet::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_platform","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_platform-0.17.3/src/collections/hash_set.rs
//     evidence={"module":553,"record_offset":56896,"parent_record_offset":56872,"ancestor_record_offset":51900,"inlinee":34555,"source_line":735,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":553,"record_offset":53868,"parent_record_offset":53840,"ancestor_record_offset":51900,"inlinee":6507,"source_line":2061,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":553,"record_offset":53896,"parent_record_offset":53868,"ancestor_record_offset":51900,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":553,"record_offset":53816,"parent_record_offset":53788,"ancestor_record_offset":51900,"inlinee":6546,"source_line":1221,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":553,"record_offset":53840,"parent_record_offset":53816,"ancestor_record_offset":51900,"inlinee":6505,"source_line":1255,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":553,"record_offset":53956,"parent_record_offset":53840,"ancestor_record_offset":51900,"inlinee":5177,"source_line":2443,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$52::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":553,"record_offset":54068,"parent_record_offset":54044,"ancestor_record_offset":51900,"inlinee":4995,"source_line":2052,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::rem_euclid
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":553,"record_offset":55136,"parent_record_offset":55112,"ancestor_record_offset":51900,"inlinee":11232,"source_line":1982,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::last
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":553,"record_offset":55996,"parent_record_offset":55964,"ancestor_record_offset":51900,"inlinee":10307,"source_line":174,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":553,"record_offset":53920,"parent_record_offset":53896,"ancestor_record_offset":51900,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":553,"record_offset":55708,"parent_record_offset":55684,"ancestor_record_offset":51900,"inlinee":7937,"source_line":917,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":553,"record_offset":55524,"parent_record_offset":55480,"ancestor_record_offset":51900,"inlinee":9345,"source_line":2344,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":553,"record_offset":58344,"parent_record_offset":58316,"ancestor_record_offset":51900,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":553,"record_offset":53688,"parent_record_offset":53648,"ancestor_record_offset":51900,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":553,"record_offset":58168,"parent_record_offset":58144,"ancestor_record_offset":51900,"inlinee":6826,"source_line":868,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":553,"record_offset":56024,"parent_record_offset":55996,"ancestor_record_offset":51900,"inlinee":5449,"source_line":1541,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":553,"record_offset":54380,"parent_record_offset":54348,"ancestor_record_offset":51900,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$47::branch
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":553,"record_offset":56084,"parent_record_offset":55964,"ancestor_record_offset":51900,"inlinee":5260,"source_line":2765,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":553,"record_offset":52476,"parent_record_offset":52448,"ancestor_record_offset":51900,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":553,"record_offset":54304,"parent_record_offset":53648,"ancestor_record_offset":51900,"inlinee":6443,"source_line":266,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":553,"record_offset":54020,"parent_record_offset":53788,"ancestor_record_offset":51900,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":553,"record_offset":52948,"parent_record_offset":52924,"ancestor_record_offset":51900,"inlinee":8268,"source_line":575,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":553,"record_offset":52924,"parent_record_offset":52900,"ancestor_record_offset":51900,"inlinee":8267,"source_line":728,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":553,"record_offset":53132,"parent_record_offset":53104,"ancestor_record_offset":51900,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$49::slice_contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":553,"record_offset":53232,"parent_record_offset":53204,"ancestor_record_offset":51900,"inlinee":8260,"source_line":405,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":553,"record_offset":53204,"parent_record_offset":53176,"ancestor_record_offset":51900,"inlinee":8258,"source_line":2589,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":553,"record_offset":55636,"parent_record_offset":55480,"ancestor_record_offset":51900,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":553,"record_offset":55684,"parent_record_offset":55660,"ancestor_record_offset":51900,"inlinee":8594,"source_line":366,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::get
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":553,"record_offset":55660,"parent_record_offset":55636,"ancestor_record_offset":51900,"inlinee":8593,"source_line":545,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":553,"record_offset":52876,"parent_record_offset":52852,"ancestor_record_offset":51900,"inlinee":8265,"source_line":81,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":553,"record_offset":52900,"parent_record_offset":52876,"ancestor_record_offset":51900,"inlinee":8266,"source_line":119,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":553,"record_offset":53260,"parent_record_offset":53232,"ancestor_record_offset":51900,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$172::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":553,"record_offset":52852,"parent_record_offset":52736,"ancestor_record_offset":51900,"inlinee":8264,"source_line":438,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":553,"record_offset":52264,"parent_record_offset":52240,"ancestor_record_offset":51900,"inlinee":9253,"source_line":377,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":553,"record_offset":52564,"parent_record_offset":52528,"ancestor_record_offset":51900,"inlinee":8243,"source_line":1527,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":553,"record_offset":52704,"parent_record_offset":52672,"ancestor_record_offset":51900,"inlinee":8253,"source_line":125,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":553,"record_offset":52672,"parent_record_offset":52636,"ancestor_record_offset":51900,"inlinee":8251,"source_line":215,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":553,"record_offset":52612,"parent_record_offset":52588,"ancestor_record_offset":51900,"inlinee":8247,"source_line":315,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":553,"record_offset":52636,"parent_record_offset":52612,"ancestor_record_offset":51900,"inlinee":8249,"source_line":709,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":553,"record_offset":52588,"parent_record_offset":52564,"ancestor_record_offset":51900,"inlinee":8245,"source_line":798,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":553,"record_offset":53176,"parent_record_offset":52636,"ancestor_record_offset":51900,"inlinee":8257,"source_line":659,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":553,"record_offset":52240,"parent_record_offset":52208,"ancestor_record_offset":51900,"inlinee":9270,"source_line":438,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":553,"record_offset":52208,"parent_record_offset":52180,"ancestor_record_offset":51900,"inlinee":9269,"source_line":468,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":553,"record_offset":52368,"parent_record_offset":52336,"ancestor_record_offset":51900,"inlinee":9252,"source_line":507,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":553,"record_offset":52424,"parent_record_offset":52368,"ancestor_record_offset":51900,"inlinee":9254,"source_line":527,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":553,"record_offset":52336,"parent_record_offset":52304,"ancestor_record_offset":51900,"inlinee":8274,"source_line":539,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":553,"record_offset":52180,"parent_record_offset":52136,"ancestor_record_offset":51900,"inlinee":8272,"source_line":62,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":553,"record_offset":52448,"parent_record_offset":52424,"ancestor_record_offset":51900,"inlinee":8315,"source_line":189,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":553,"record_offset":52736,"parent_record_offset":52704,"ancestor_record_offset":51900,"inlinee":8255,"source_line":78,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":553,"record_offset":53020,"parent_record_offset":52736,"ancestor_record_offset":51900,"inlinee":8270,"source_line":16,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":553,"record_offset":52992,"parent_record_offset":52736,"ancestor_record_offset":51900,"inlinee":8269,"source_line":10,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":553,"record_offset":52796,"parent_record_offset":52736,"ancestor_record_offset":51900,"inlinee":8262,"source_line":23,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::GladeType::is_winter
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":553,"record_offset":57324,"parent_record_offset":57300,"ancestor_record_offset":51900,"inlinee":27994,"source_line":117,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::sin_cos
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/math.rs
//     evidence={"module":553,"record_offset":57768,"parent_record_offset":57744,"ancestor_record_offset":51900,"inlinee":11738,"source_line":182,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/set.rs
//     evidence={"module":553,"record_offset":56920,"parent_record_offset":56896,"ancestor_record_offset":51900,"inlinee":5589,"source_line":862,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":553,"record_offset":53576,"parent_record_offset":53540,"ancestor_record_offset":51900,"inlinee":8278,"source_line":425,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":553,"record_offset":58864,"parent_record_offset":58832,"ancestor_record_offset":51900,"inlinee":8283,"source_line":601,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":553,"record_offset":59316,"parent_record_offset":59112,"ancestor_record_offset":51900,"inlinee":6607,"source_line":602,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":553,"record_offset":53716,"parent_record_offset":53688,"ancestor_record_offset":51900,"inlinee":8280,"source_line":92,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":553,"record_offset":53788,"parent_record_offset":53648,"ancestor_record_offset":51900,"inlinee":6545,"source_line":429,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":553,"record_offset":53992,"parent_record_offset":53788,"ancestor_record_offset":51900,"inlinee":6548,"source_line":591,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sin
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":553,"record_offset":57816,"parent_record_offset":57792,"ancestor_record_offset":51900,"inlinee":11242,"source_line":699,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sin_cos
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":553,"record_offset":57792,"parent_record_offset":57768,"ancestor_record_offset":51900,"inlinee":11739,"source_line":910,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] std::f64::impl$0::cos
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f64.rs
//     evidence={"module":553,"record_offset":57628,"parent_record_offset":57592,"ancestor_record_offset":51900,"inlinee":34563,"source_line":723,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] std::f64::impl$0::sin
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f64.rs
//     evidence={"module":553,"record_offset":57684,"parent_record_offset":57592,"ancestor_record_offset":51900,"inlinee":33029,"source_line":699,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":553,"record_offset":53744,"parent_record_offset":53716,"ancestor_record_offset":51900,"inlinee":5707,"source_line":58,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":553,"record_offset":53648,"parent_record_offset":53612,"ancestor_record_offset":51900,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":553,"record_offset":53612,"parent_record_offset":53576,"ancestor_record_offset":51900,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] time::date::Date::__from_ordinal_date_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"time","version":"0.3.36","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/time-0.3.36/src/date.rs
//     evidence={"module":553,"record_offset":57476,"parent_record_offset":57444,"ancestor_record_offset":51900,"inlinee":17843,"source_line":85,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] time::offset_date_time::OffsetDateTime::new_in_offset
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"time","version":"0.3.36","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/time-0.3.36/src/offset_date_time.rs
//     evidence={"module":553,"record_offset":57556,"parent_record_offset":57532,"ancestor_record_offset":51900,"inlinee":9174,"source_line":143,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] time::primitive_date_time::PrimitiveDateTime::assume_offset
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"time","version":"0.3.36","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/time-0.3.36/src/primitive_date_time.rs
//     evidence={"module":553,"record_offset":57532,"parent_record_offset":57508,"ancestor_record_offset":51900,"inlinee":34560,"source_line":477,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::time_of_day::remap_hour_to_fmod_hour
// 记录形式: compiled_procedure; instances=1
// 源码行: 317
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":553,"record_offset":59824,"rva":11377200,"va":5380086320}
// 编译实例 RVAs: 0xad9a30
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <alloc::vec::Vec<_, _>>::insert_mut::assert_failed
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t36533657","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380086320,"raw_hex":"e899752d02","size":5,"soft_fail":false,"target_va":5416620867,"va":5380087205,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-4407700","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380086320,"raw_hex":"e86cbebcff","size":5,"soft_fail":false,"target_va":5375678688,"va":5380086383,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t-4408393","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380086320,"raw_hex":"e8b7bbbcff","size":5,"soft_fail":false,"target_va":5375678704,"va":5380087092,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=2
//     target_aliases=1699 candidates; complete list: time_of_day.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11364181","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380086320,"raw_hex":"e8ab9852ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5380086368,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb30
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: time_of_day.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t36970438","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380086320,"raw_hex":"e8c61f3402","size":5,"soft_fail":false,"target_va":5417057072,"va":5380086629,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t36533598","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380086320,"raw_hex":"e85e752d02","size":5,"soft_fail":false,"target_va":5416620769,"va":5380087166,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t36279538","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380086320,"raw_hex":"e8f2942902","size":5,"soft_fail":false,"target_va":5416366080,"va":5380086537,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::fit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3194487","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380086320,"raw_hex":"e877be3000","size":5,"soft_fail":false,"target_va":5383281536,"va":5380087044,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::boxed::box_assume_init_into_vec_unsafe
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":553,"record_offset":60172,"parent_record_offset":59824,"ancestor_record_offset":59824,"inlinee":4459,"source_line":262,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_direct] alloc::boxed::impl$0::new_uninit
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":553,"record_offset":59992,"parent_record_offset":59824,"ancestor_record_offset":59824,"inlinee":4443,"source_line":311,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::insert
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":553,"record_offset":60520,"parent_record_offset":59824,"ancestor_record_offset":59824,"inlinee":11069,"source_line":2246,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":553,"record_offset":61940,"parent_record_offset":59824,"ancestor_record_offset":59824,"inlinee":6656,"source_line":84,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_direct] core::ops::range::Range::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":553,"record_offset":62860,"parent_record_offset":59824,"ancestor_record_offset":59824,"inlinee":11437,"source_line":123,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":553,"record_offset":63004,"parent_record_offset":59824,"ancestor_record_offset":59824,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":553,"record_offset":61772,"parent_record_offset":59824,"ancestor_record_offset":59824,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":553,"record_offset":60076,"parent_record_offset":60052,"ancestor_record_offset":59824,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":553,"record_offset":60100,"parent_record_offset":60076,"ancestor_record_offset":59824,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":553,"record_offset":63192,"parent_record_offset":63168,"ancestor_record_offset":59824,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":553,"record_offset":63216,"parent_record_offset":63192,"ancestor_record_offset":59824,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":553,"record_offset":60124,"parent_record_offset":60100,"ancestor_record_offset":59824,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":553,"record_offset":63240,"parent_record_offset":63216,"ancestor_record_offset":59824,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":553,"record_offset":60052,"parent_record_offset":60020,"ancestor_record_offset":59824,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":553,"record_offset":63168,"parent_record_offset":63076,"ancestor_record_offset":59824,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":553,"record_offset":60020,"parent_record_offset":59992,"ancestor_record_offset":59824,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":553,"record_offset":60608,"parent_record_offset":60584,"ancestor_record_offset":59824,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":553,"record_offset":63104,"parent_record_offset":63076,"ancestor_record_offset":59824,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":553,"record_offset":63076,"parent_record_offset":63052,"ancestor_record_offset":59824,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":553,"record_offset":60656,"parent_record_offset":60632,"ancestor_record_offset":59824,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":553,"record_offset":60632,"parent_record_offset":60608,"ancestor_record_offset":59824,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":553,"record_offset":63052,"parent_record_offset":63028,"ancestor_record_offset":59824,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":553,"record_offset":60196,"parent_record_offset":60172,"ancestor_record_offset":59824,"inlinee":4460,"source_line":480,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":553,"record_offset":60584,"parent_record_offset":60544,"ancestor_record_offset":59824,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":553,"record_offset":60220,"parent_record_offset":60196,"ancestor_record_offset":59824,"inlinee":4461,"source_line":1181,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::insert_mut
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":553,"record_offset":60544,"parent_record_offset":60520,"ancestor_record_offset":59824,"inlinee":10769,"source_line":2277,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":553,"record_offset":60868,"parent_record_offset":60816,"ancestor_record_offset":59824,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":553,"record_offset":60752,"parent_record_offset":60544,"ancestor_record_offset":59824,"inlinee":4802,"source_line":2161,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":553,"record_offset":62924,"parent_record_offset":62892,"ancestor_record_offset":59824,"inlinee":6833,"source_line":2145,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":553,"record_offset":62956,"parent_record_offset":62924,"ancestor_record_offset":59824,"inlinee":8914,"source_line":1918,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":553,"record_offset":62188,"parent_record_offset":62164,"ancestor_record_offset":59824,"inlinee":7042,"source_line":81,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":553,"record_offset":62236,"parent_record_offset":62212,"ancestor_record_offset":59824,"inlinee":7430,"source_line":82,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$3::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":553,"record_offset":61976,"parent_record_offset":61940,"ancestor_record_offset":59824,"inlinee":9633,"source_line":164,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":553,"record_offset":62260,"parent_record_offset":62236,"ancestor_record_offset":59824,"inlinee":4993,"source_line":4225,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":553,"record_offset":63136,"parent_record_offset":63104,"ancestor_record_offset":59824,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":553,"record_offset":62892,"parent_record_offset":62860,"ancestor_record_offset":59824,"inlinee":6826,"source_line":868,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":553,"record_offset":62212,"parent_record_offset":62188,"ancestor_record_offset":59824,"inlinee":7428,"source_line":1647,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":553,"record_offset":60696,"parent_record_offset":60544,"ancestor_record_offset":59824,"inlinee":4801,"source_line":623,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":553,"record_offset":63028,"parent_record_offset":63004,"ancestor_record_offset":59824,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":553,"record_offset":61016,"parent_record_offset":60816,"ancestor_record_offset":59824,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":553,"record_offset":62132,"parent_record_offset":62064,"ancestor_record_offset":59824,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":553,"record_offset":62100,"parent_record_offset":62064,"ancestor_record_offset":59824,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":553,"record_offset":60724,"parent_record_offset":60544,"ancestor_record_offset":59824,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":553,"record_offset":61796,"parent_record_offset":61772,"ancestor_record_offset":59824,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":553,"record_offset":62064,"parent_record_offset":62028,"ancestor_record_offset":59824,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$31::collect_from_iter_no_buf
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":553,"record_offset":62164,"parent_record_offset":62028,"ancestor_record_offset":59824,"inlinee":7466,"source_line":374,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$7::next
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":553,"record_offset":62028,"parent_record_offset":61976,"ancestor_record_offset":59824,"inlinee":9808,"source_line":193,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::time_of_day::suncalc_pos_to_vec
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 378
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8702
// 首条位置: {"module":553,"inlinee":34562}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::f64::impl$0::cos
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f64.rs
//     evidence={"module":553,"record_offset":57628,"parent_record_offset":57592,"ancestor_record_offset":57592,"inlinee":34563,"source_line":723,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_direct] std::f64::impl$0::sin
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f64.rs
//     evidence={"module":553,"record_offset":57684,"parent_record_offset":57592,"ancestor_record_offset":57592,"inlinee":33029,"source_line":699,"archive":"_pdb/C/modules/00553.jsonl.gz"}
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
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TodSequence>,bevy_ecs::change_detection::ResMut<country_core::resources::weather::Weather>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<bevy_input::button_input::ButtonInput<enum2$<bevy_input::keyboard::KeyCode> > >,bevy_ecs::change_detection::ResMut<country_core::render::directional_light::DirectionalLight>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::render::atmosphere::DerivedAtmosphere>,bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::session::NewSessionStartedCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::TimeOfDayScreen>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TodSequence>", "bevy_ecs::change_detection::ResMut<country_core::resources::weather::Weather>", "bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::change_detection::Res<bevy_input::button_input::ButtonInput<enum2$<bevy_input::keyboard::KeyCode> > >", "bevy_ecs::change_detection::ResMut<country_core::render::directional_light::DirectionalLight>", "bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>", "bevy_ecs::change_detection::ResMut<country_core::render::atmosphere::DerivedAtmosphere>", "bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>", "bevy_ecs::message::message_reader::MessageReader<country_core::resources::session::NewSessionStartedCmd>", "bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::load_game::LoadingGameComplete>", "bevy_ecs::change_detection::ResMut<country_core::resources::gui::TimeOfDayScreen>", "bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TodSequence>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::weather::Weather>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_input::button_input::ButtonInput<enum2$<bevy_input::keyboard::KeyCode> > >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::render::directional_light::DirectionalLight>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::render::atmosphere::DerivedAtmosphere>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::resources::session::NewSessionStartedCmd>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::load_game::LoadingGameComplete>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::gui::TimeOfDayScreen>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>

// ------------------------------------------------------------------
// 函数: country_core::systems::time_of_day::remap_hour_to_fmod_hour
// 信息 ID: 97617d3458c1f2dd09a15d9d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::ops::function::FnOnce::call_once<tiny_glade::gui::gui_debug::do_gui_debug::closure$0::closure$0::closure$12::closure_env$1,tuple$<ref_mut$<egui::ui::Ui> > > @ 0x6859e8 -> 0xad9a30 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::time_of_day::time_of_day @ 0xad8e60 -> 0xad9a30 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::time_of_day::suncalc_pos_to_vec
// 信息 ID: 2783b2d6b9ba429e586ac316
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::time_of_day::time_of_day; module=553, inline_record=57592; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::time_of_day::time_of_day
// 信息 ID: 3041107bf584a4a8de5cc266
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 239440af9b64978c03cf3f10: void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TodSequence>,bevy_ecs::change_detection::ResMut<country_core::resources::weather::Weather>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<bevy_input::button_input::ButtonInput<enum2$<bevy_input::keyboard::KeyCode> > >,bevy_ecs::change_detection::ResMut<country_core::render::directional_light::DirectionalLight>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::render::atmosphere::DerivedAtmosphere>,bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::session::NewSessionStartedCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::TimeOfDayScreen>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TodSequence>,bevy_ecs::change_detection::ResMut<country_core::resources::weather::Weather>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<bevy_input::button_input::ButtonInput<enum2$<bevy_input::keyboard::KeyCode> > >,bevy_ecs::change_detection::ResMut<country_core::render::directional_light::DirectionalLight>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::render::atmosphere::DerivedAtmosphere>,bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::session::NewSessionStartedCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::TimeOfDayScreen>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TodSequence>,bevy_ecs::change_detection::ResMut<country_core::resources::weather::Weather>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<bevy_input::button_input::ButtonInput<enum2$<bevy_input::keyboard::KeyCode> > >,bevy_ecs::change_detection::ResMut<country_core::render::directional_light::DirectionalLight>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::render::atmosphere::DerivedAtmosphere>,bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::session::NewSessionStartedCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::TimeOfDayScreen>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>)> > @ 0x291f1b -> 0xad8640 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TodSequence>,bevy_ecs::change_detection::ResMut<country_core::resources::weather::Weather>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<bevy_input::button_input::ButtonInput<enum2$<bevy_input::keyboard::KeyCode> > >,bevy_ecs::change_detection::ResMut<country_core::render::directional_light::DirectionalLight>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::render::atmosphere::DerivedAtmosphere>,bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::session::NewSessionStartedCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::TimeOfDayScreen>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TodSequence>,bevy_ecs::change_detection::ResMut<country_core::resources::weather::Weather>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<bevy_input::button_input::ButtonInput<enum2$<bevy_input::keyboard::KeyCode> > >,bevy_ecs::change_detection::ResMut<country_core::render::directional_light::DirectionalLight>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::render::atmosphere::DerivedAtmosphere>,bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::session::NewSessionStartedCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::TimeOfDayScreen>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>)> @ 0x55699f -> 0xad8640 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===

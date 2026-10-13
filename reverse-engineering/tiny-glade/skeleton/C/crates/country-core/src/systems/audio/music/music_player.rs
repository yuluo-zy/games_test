// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/country-core/src/systems/audio/music/music_player.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 4258b6fa0530041ce3c245e94ba1539d4eae3b5c0ca8f047d5ca57d12abbcce5
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: music_player.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 546, 553, 1762
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use anyhow; // namespace candidate
//   package identities: [{"name":"anyhow","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.100"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use fastrand; // namespace candidate
//   package identities: [{"name":"fastrand","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.8.0"},{"name":"fastrand","source":"registry+https://github.com/rust-lang/crates.io-index","version":"2.3.0"}]
// use libfmod; // namespace candidate
//   package identities: [{"name":"libfmod","source":"registry+https://github.com/rust-lang/crates.io-index","version":"2.222.6"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.27"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::audio::music::music_player::impl$2::fmt
// 记录形式: compiled_procedure; instances=1
// 源码行: 17
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1762,"record_offset":556,"rva":28020112,"va":5396729232}
// 编译实例 RVAs: 0x1ab8d90
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <core::fmt::Formatter>::debug_struct_field1_finish
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-11908166","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396729232,"raw_hex":"e8ba4b4aff","size":5,"soft_fail":false,"target_va":5384821184,"va":5396729345,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <core::fmt::Formatter>::debug_struct_field2_finish
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-11908106","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396729232,"raw_hex":"e8f64b4aff","size":5,"soft_fail":false,"target_va":5384821360,"va":5396729461,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] alias_set_at_0x140f5e510
//     owner=unresolved_namespace; count=2
//     target_aliases=4 candidates; complete list: music_player.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t-11905202","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5396729232,"raw_hex":"e94e574aff","size":5,"soft_fail":false,"target_va":5384824080,"va":5396729277,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::audio::music::music_player::MusicPlayer::update
// 记录形式: compiled_procedure; instances=1
// 源码行: 51
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":546,"record_offset":128968,"rva":11178192,"va":5379887312}
// 编译实例 RVAs: 0xaa90d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4721448","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379887312,"raw_hex":"e8280b4800","size":5,"soft_fail":false,"target_va":5384609408,"va":5379887955,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a7cbd0
//     owner=unresolved_namespace; count=13
//     target_aliases=2 candidates; complete list: music_player.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-181642","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379887312,"raw_hex":"e8763afdff","size":5,"soft_fail":false,"target_va":5379705808,"va":5379887445,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bd80
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: music_player.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t37169104","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379887312,"raw_hex":"e8d0273702","size":5,"soft_fail":false,"target_va":5417057664,"va":5379888555,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=12
//     evidence={"assembly":"\tcallq\t36733480","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379887312,"raw_hex":"e828823002","size":5,"soft_fail":false,"target_va":5416621632,"va":5379888147,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::audio::music::music_player::MusicPlayer::switch_track
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-3840","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379887312,"raw_hex":"e800f1ffff","size":5,"soft_fail":false,"target_va":5379883712,"va":5379887547,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] fastrand::f32
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"fastrand","version":"1.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"fastrand","version":"2.3.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t3494572","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379887312,"raw_hex":"e8ac523500","size":5,"soft_fail":false,"target_va":5383383216,"va":5379888639,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] libfmod::EventInstance::get_paused
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"libfmod","version":"2.222.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t16760561","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379887312,"raw_hex":"e8f1beff00","size":5,"soft_fail":false,"target_va":5396649328,"va":5379888762,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] libfmod::EventInstance::get_timeline_position
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"libfmod","version":"2.222.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t16765572","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379887312,"raw_hex":"e884d2ff00","size":5,"soft_fail":false,"target_va":5396652992,"va":5379887415,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] libfmod::EventInstance::set_paused
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"libfmod","version":"2.222.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t16761271","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379887312,"raw_hex":"e8b7c1ff00","size":5,"soft_fail":false,"target_va":5396650080,"va":5379888804,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] libfmod::EventInstance::start
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"libfmod","version":"2.222.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t16766688","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379887312,"raw_hex":"e8e0d6ff00","size":5,"soft_fail":false,"target_va":5396654304,"va":5379887611,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] libfmod::EventInstance::stop
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"libfmod","version":"2.222.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t16766204","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379887312,"raw_hex":"e8fcd4ff00","size":5,"soft_fail":false,"target_va":5396653968,"va":5379887759,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":546,"record_offset":130012,"parent_record_offset":128968,"ancestor_record_offset":128968,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":546,"record_offset":129260,"parent_record_offset":128968,"ancestor_record_offset":128968,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::audio_resources::AudioInstance::is_paused
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/audio_resources.rs
//     evidence={"module":546,"record_offset":131192,"parent_record_offset":128968,"ancestor_record_offset":128968,"inlinee":33515,"source_line":572,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::audio_resources::AudioInstance::resume
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/audio_resources.rs
//     evidence={"module":546,"record_offset":131304,"parent_record_offset":128968,"ancestor_record_offset":128968,"inlinee":33514,"source_line":581,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::audio_resources::AudioInstance::start
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/audio_resources.rs
//     evidence={"module":546,"record_offset":129576,"parent_record_offset":128968,"ancestor_record_offset":128968,"inlinee":33513,"source_line":505,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::audio_resources::AudioInstance::stop
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/audio_resources.rs
//     evidence={"module":546,"record_offset":129808,"parent_record_offset":128968,"ancestor_record_offset":128968,"inlinee":33511,"source_line":532,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::audio_resources::AudioInstance::stop_immediately
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/audio_resources.rs
//     evidence={"module":546,"record_offset":130404,"parent_record_offset":128968,"ancestor_record_offset":128968,"inlinee":33507,"source_line":544,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::audio::music::music_player::MusicPlayer::current_track
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/audio/music/music_player.rs
//     evidence={"module":546,"record_offset":129168,"parent_record_offset":128968,"ancestor_record_offset":128968,"inlinee":34005,"source_line":232,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::audio::music::music_player::MusicPlayer::probabilistic_switch_track
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/audio/music/music_player.rs
//     evidence={"module":546,"record_offset":130896,"parent_record_offset":128968,"ancestor_record_offset":128968,"inlinee":34008,"source_line":252,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":546,"record_offset":130068,"parent_record_offset":128968,"ancestor_record_offset":128968,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":546,"record_offset":129928,"parent_record_offset":128968,"ancestor_record_offset":128968,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::ceil
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":546,"record_offset":130840,"parent_record_offset":128968,"ancestor_record_offset":128968,"inlinee":11750,"source_line":72,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::max
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":546,"record_offset":129452,"parent_record_offset":129284,"ancestor_record_offset":128968,"inlinee":5501,"source_line":1033,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::ceil
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":546,"record_offset":130864,"parent_record_offset":130840,"ancestor_record_offset":128968,"inlinee":11751,"source_line":1749,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::powi
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":546,"record_offset":130948,"parent_record_offset":130924,"ancestor_record_offset":128968,"inlinee":17053,"source_line":2010,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":546,"record_offset":129332,"parent_record_offset":129304,"ancestor_record_offset":128968,"inlinee":5449,"source_line":1541,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":546,"record_offset":129616,"parent_record_offset":129576,"ancestor_record_offset":128968,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":546,"record_offset":130036,"parent_record_offset":130012,"ancestor_record_offset":128968,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":546,"record_offset":129408,"parent_record_offset":129380,"ancestor_record_offset":128968,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":546,"record_offset":129380,"parent_record_offset":129360,"ancestor_record_offset":128968,"inlinee":5287,"source_line":708,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":546,"record_offset":129644,"parent_record_offset":129576,"ancestor_record_offset":128968,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":546,"record_offset":129192,"parent_record_offset":129168,"ancestor_record_offset":128968,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":546,"record_offset":129220,"parent_record_offset":129192,"ancestor_record_offset":128968,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":546,"record_offset":129952,"parent_record_offset":129928,"ancestor_record_offset":128968,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":546,"record_offset":129976,"parent_record_offset":129952,"ancestor_record_offset":128968,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::audio_resources::AudioInstance::get_timeline_position
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/audio_resources.rs
//     evidence={"module":546,"record_offset":129304,"parent_record_offset":129284,"ancestor_record_offset":128968,"inlinee":33509,"source_line":603,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::audio_resources::impl$9::get_timeline_position::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/audio_resources.rs
//     evidence={"module":546,"record_offset":129360,"parent_record_offset":129332,"ancestor_record_offset":128968,"inlinee":33406,"source_line":605,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::audio::music::music_player::impl$0::update::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/audio/music/music_player.rs
//     evidence={"module":546,"record_offset":129284,"parent_record_offset":129260,"ancestor_record_offset":128968,"inlinee":34007,"source_line":54,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":546,"record_offset":130116,"parent_record_offset":130092,"ancestor_record_offset":128968,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":546,"record_offset":130092,"parent_record_offset":130068,"ancestor_record_offset":128968,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":546,"record_offset":130140,"parent_record_offset":130116,"ancestor_record_offset":128968,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::powi
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":546,"record_offset":130924,"parent_record_offset":130896,"ancestor_record_offset":128968,"inlinee":17052,"source_line":323,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::audio::music::music_player::impl$0::update::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 54
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x84D7
// 首条位置: {"module":546,"inlinee":34007}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::Ord::max
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":546,"record_offset":129452,"parent_record_offset":129284,"ancestor_record_offset":129284,"inlinee":5501,"source_line":1033,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::audio_resources::AudioInstance::get_timeline_position
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/audio_resources.rs
//     evidence={"module":546,"record_offset":129304,"parent_record_offset":129284,"ancestor_record_offset":129284,"inlinee":33509,"source_line":603,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":546,"record_offset":129332,"parent_record_offset":129304,"ancestor_record_offset":129284,"inlinee":5449,"source_line":1541,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":546,"record_offset":129408,"parent_record_offset":129380,"ancestor_record_offset":129284,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":546,"record_offset":129380,"parent_record_offset":129360,"ancestor_record_offset":129284,"inlinee":5287,"source_line":708,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::audio_resources::impl$9::get_timeline_position::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/audio_resources.rs
//     evidence={"module":546,"record_offset":129360,"parent_record_offset":129332,"ancestor_record_offset":129284,"inlinee":33406,"source_line":605,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::audio::music::music_player::MusicPlayer::trigger_photomode_minus_fpv_snapshot
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 162
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8503
// 首条位置: {"module":553,"inlinee":34051}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::resources::audio_resources::AudioInstance::stop_immediately
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/audio_resources.rs
//     evidence={"module":553,"record_offset":76016,"parent_record_offset":75984,"ancestor_record_offset":75984,"inlinee":33507,"source_line":544,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":553,"record_offset":76048,"parent_record_offset":76016,"ancestor_record_offset":75984,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":553,"record_offset":76076,"parent_record_offset":76016,"ancestor_record_offset":75984,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::audio::music::music_player::MusicPlayer::trigger_fpv_snapshot
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 167
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8501
// 首条位置: {"module":553,"inlinee":34049}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::resources::audio_resources::AudioInstance::stop_immediately
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/audio_resources.rs
//     evidence={"module":553,"record_offset":76592,"parent_record_offset":76560,"ancestor_record_offset":76560,"inlinee":33507,"source_line":544,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":553,"record_offset":76624,"parent_record_offset":76592,"ancestor_record_offset":76560,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":553,"record_offset":76652,"parent_record_offset":76592,"ancestor_record_offset":76560,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/00553.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::audio::music::music_player::MusicPlayer::stop_photomode_snapshots
// 记录形式: compiled_procedure; instances=1
// 源码行: 172
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":546,"record_offset":125372,"rva":11175552,"va":5379884672}
// 编译实例 RVAs: 0xaa8680
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140a7cbd0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: music_player.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-179179","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379884672,"raw_hex":"e81544fdff","size":5,"soft_fail":false,"target_va":5379705808,"va":5379884982,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t36736765","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379884672,"raw_hex":"e8fd8e3002","size":5,"soft_fail":false,"target_va":5416621632,"va":5379884862,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] libfmod::EventInstance::stop
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"libfmod","version":"2.222.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t16769246","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379884672,"raw_hex":"e8dee0ff00","size":5,"soft_fail":false,"target_va":5396653968,"va":5379884717,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::resources::audio_resources::AudioInstance::stop
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/audio_resources.rs
//     evidence={"module":546,"record_offset":125544,"parent_record_offset":125372,"ancestor_record_offset":125372,"inlinee":33511,"source_line":532,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":546,"record_offset":125584,"parent_record_offset":125544,"ancestor_record_offset":125372,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":546,"record_offset":125612,"parent_record_offset":125544,"ancestor_record_offset":125372,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::audio::music::music_player::MusicPlayer::init
// 记录形式: compiled_procedure; instances=1
// 源码行: 177
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":546,"record_offset":125772,"rva":11175936,"va":5379885056}
// 编译实例 RVAs: 0xaa8800
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t-4206730","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379885056,"raw_hex":"e876cfbfff","size":5,"soft_fail":false,"target_va":5375678688,"va":5379885413,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=4
//     target_aliases=1699 candidates; complete list: music_player.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11163211","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379885056,"raw_hex":"e8b5a955ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5379885398,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400b45a0
//     owner=unresolved_namespace; count=3
//     target_aliases=14 candidates; complete list: music_player.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10438796","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379885056,"raw_hex":"e874b760ff","size":5,"soft_fail":false,"target_va":5369447840,"va":5379886631,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409aa2e0
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: music_player.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1043180","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379885056,"raw_hex":"e81415f0ff","size":5,"soft_fail":false,"target_va":5378843360,"va":5379886535,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t36734339","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379885056,"raw_hex":"e883853002","size":5,"soft_fail":false,"target_va":5416620794,"va":5379886450,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4737492","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379885056,"raw_hex":"e8d4494800","size":5,"soft_fail":false,"target_va":5384624080,"va":5379886583,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t36736240","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379885056,"raw_hex":"e8f08c3002","size":5,"soft_fail":false,"target_va":5416621632,"va":5379885387,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::audio_resources::AudioCoreInner::instantiate_audio
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-200353","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379885056,"raw_hex":"e85ff1fcff","size":5,"soft_fail":false,"target_va":5379684800,"va":5379885148,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":546,"record_offset":125932,"parent_record_offset":125772,"ancestor_record_offset":125772,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":546,"record_offset":126560,"parent_record_offset":125772,"ancestor_record_offset":125772,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::audio_resources::AudioCore::instantiate_audio
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/audio_resources.rs
//     evidence={"module":546,"record_offset":125960,"parent_record_offset":125772,"ancestor_record_offset":125772,"inlinee":29635,"source_line":321,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":546,"record_offset":126348,"parent_record_offset":126324,"ancestor_record_offset":125772,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":546,"record_offset":126372,"parent_record_offset":126348,"ancestor_record_offset":125772,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":546,"record_offset":126396,"parent_record_offset":126372,"ancestor_record_offset":125772,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":546,"record_offset":126324,"parent_record_offset":126296,"ancestor_record_offset":125772,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":546,"record_offset":126236,"parent_record_offset":126208,"ancestor_record_offset":125772,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":546,"record_offset":126296,"parent_record_offset":126264,"ancestor_record_offset":125772,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":546,"record_offset":126264,"parent_record_offset":126236,"ancestor_record_offset":125772,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":546,"record_offset":126116,"parent_record_offset":126088,"ancestor_record_offset":125772,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":546,"record_offset":126144,"parent_record_offset":126116,"ancestor_record_offset":125772,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":546,"record_offset":126172,"parent_record_offset":126144,"ancestor_record_offset":125772,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":546,"record_offset":126088,"parent_record_offset":126060,"ancestor_record_offset":125772,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":546,"record_offset":126060,"parent_record_offset":125960,"ancestor_record_offset":125772,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$34::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":546,"record_offset":126032,"parent_record_offset":125960,"ancestor_record_offset":125772,"inlinee":4944,"source_line":2421,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":546,"record_offset":126208,"parent_record_offset":126172,"ancestor_record_offset":125772,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Ok
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/lib.rs
//     evidence={"module":546,"record_offset":126528,"parent_record_offset":125960,"ancestor_record_offset":125772,"inlinee":29637,"source_line":653,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":546,"record_offset":126004,"parent_record_offset":125960,"ancestor_record_offset":125772,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":546,"record_offset":126452,"parent_record_offset":126172,"ancestor_record_offset":125772,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":546,"record_offset":126476,"parent_record_offset":126452,"ancestor_record_offset":125772,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":546,"record_offset":126604,"parent_record_offset":126560,"ancestor_record_offset":125772,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::audio::music::music_player::MusicPlayer::load_tracks
// 记录形式: compiled_procedure; instances=1
// 源码行: 200
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":546,"record_offset":119016,"rva":11173168,"va":5379882288}
// 编译实例 RVAs: 0xaa7d30
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <alloc::string::String as core::clone::Clone>::clone
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4912944","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379882288,"raw_hex":"e830f74a00","size":5,"soft_fail":false,"target_va":5384795984,"va":5379883035,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-4204010","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379882288,"raw_hex":"e816dabfff","size":5,"soft_fail":false,"target_va":5375678688,"va":5379882693,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=6
//     evidence={"assembly":"\tcallq\t-4203743","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379882288,"raw_hex":"e821dbbfff","size":5,"soft_fail":false,"target_va":5375678704,"va":5379882442,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: music_player.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11160487","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379882288,"raw_hex":"e859b455ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5379882674,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400b45a0
//     owner=unresolved_namespace; count=3
//     target_aliases=14 candidates; complete list: music_player.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10435486","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379882288,"raw_hex":"e862c460ff","size":5,"soft_fail":false,"target_va":5369447840,"va":5379883321,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409ab250
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: music_player.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1035106","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379882288,"raw_hex":"e89e34f0ff","size":5,"soft_fail":false,"target_va":5378847312,"va":5379882413,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bcf5d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: music_player.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1209934","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379882288,"raw_hex":"e84e761200","size":5,"soft_fail":false,"target_va":5381092816,"va":5379882877,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f390
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: music_player.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t36467636","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379882288,"raw_hex":"e8b4732c02","size":5,"soft_fail":false,"target_va":5416350608,"va":5379882967,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: music_player.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t37174332","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379882288,"raw_hex":"e83c3c3702","size":5,"soft_fail":false,"target_va":5417057056,"va":5379882719,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t36482643","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379882288,"raw_hex":"e853ae2c02","size":5,"soft_fail":false,"target_va":5416365712,"va":5379883064,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t36737601","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379882288,"raw_hex":"e841923002","size":5,"soft_fail":false,"target_va":5416620794,"va":5379883188,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4740395","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379882288,"raw_hex":"e82b554800","size":5,"soft_fail":false,"target_va":5384624080,"va":5379883680,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,enum2$<libfmod::Error> > > >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t7694","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379882288,"raw_hex":"e80e1e0000","size":5,"soft_fail":false,"target_va":5379890592,"va":5379882893,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t36738391","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379882288,"raw_hex":"e857953002","size":5,"soft_fail":false,"target_va":5416621632,"va":5379883236,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::audio_resources::AudioCoreInner::instantiate_audio
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-197832","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379882288,"raw_hex":"e838fbfcff","size":5,"soft_fail":false,"target_va":5379684800,"va":5379882627,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":546,"record_offset":119204,"parent_record_offset":119016,"ancestor_record_offset":119016,"inlinee":7435,"source_line":2968,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":546,"record_offset":121488,"parent_record_offset":119016,"ancestor_record_offset":119016,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":546,"record_offset":122228,"parent_record_offset":119016,"ancestor_record_offset":119016,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::is_ok
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":546,"record_offset":121460,"parent_record_offset":119016,"ancestor_record_offset":119016,"inlinee":5676,"source_line":593,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":546,"record_offset":121380,"parent_record_offset":119016,"ancestor_record_offset":119016,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":546,"record_offset":120644,"parent_record_offset":119016,"ancestor_record_offset":119016,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$1::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":546,"record_offset":120532,"parent_record_offset":119016,"ancestor_record_offset":119016,"inlinee":6163,"source_line":25,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::audio_resources::AudioCore::instantiate_audio
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/audio_resources.rs
//     evidence={"module":546,"record_offset":120716,"parent_record_offset":119016,"ancestor_record_offset":119016,"inlinee":29635,"source_line":321,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":546,"record_offset":121168,"parent_record_offset":121144,"ancestor_record_offset":119016,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":546,"record_offset":121192,"parent_record_offset":121168,"ancestor_record_offset":119016,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":546,"record_offset":119616,"parent_record_offset":119592,"ancestor_record_offset":119016,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":546,"record_offset":119640,"parent_record_offset":119616,"ancestor_record_offset":119016,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":546,"record_offset":121216,"parent_record_offset":121192,"ancestor_record_offset":119016,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":546,"record_offset":119664,"parent_record_offset":119640,"ancestor_record_offset":119016,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":546,"record_offset":121144,"parent_record_offset":120996,"ancestor_record_offset":119016,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":546,"record_offset":119592,"parent_record_offset":119536,"ancestor_record_offset":119016,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":546,"record_offset":119288,"parent_record_offset":119264,"ancestor_record_offset":119016,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":546,"record_offset":120936,"parent_record_offset":120908,"ancestor_record_offset":119016,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":546,"record_offset":119564,"parent_record_offset":119536,"ancestor_record_offset":119016,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":546,"record_offset":119536,"parent_record_offset":119512,"ancestor_record_offset":119016,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":546,"record_offset":119336,"parent_record_offset":119312,"ancestor_record_offset":119016,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":546,"record_offset":119312,"parent_record_offset":119288,"ancestor_record_offset":119016,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":546,"record_offset":120996,"parent_record_offset":120964,"ancestor_record_offset":119016,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":546,"record_offset":120964,"parent_record_offset":120936,"ancestor_record_offset":119016,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":546,"record_offset":119512,"parent_record_offset":119488,"ancestor_record_offset":119016,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":546,"record_offset":121032,"parent_record_offset":120996,"ancestor_record_offset":119016,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":546,"record_offset":120816,"parent_record_offset":120788,"ancestor_record_offset":119016,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":546,"record_offset":120844,"parent_record_offset":120816,"ancestor_record_offset":119016,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":546,"record_offset":120872,"parent_record_offset":120844,"ancestor_record_offset":119016,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":546,"record_offset":120788,"parent_record_offset":120760,"ancestor_record_offset":119016,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":546,"record_offset":120760,"parent_record_offset":120716,"ancestor_record_offset":119016,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":546,"record_offset":119264,"parent_record_offset":119240,"ancestor_record_offset":119016,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":546,"record_offset":119240,"parent_record_offset":119204,"ancestor_record_offset":119016,"inlinee":4484,"source_line":1834,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":546,"record_offset":121516,"parent_record_offset":121488,"ancestor_record_offset":119016,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":546,"record_offset":120908,"parent_record_offset":120872,"ancestor_record_offset":119016,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Ok
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/lib.rs
//     evidence={"module":546,"record_offset":121348,"parent_record_offset":120716,"ancestor_record_offset":119016,"inlinee":29637,"source_line":653,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":546,"record_offset":121056,"parent_record_offset":121032,"ancestor_record_offset":119016,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":546,"record_offset":121104,"parent_record_offset":121080,"ancestor_record_offset":119016,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":546,"record_offset":121080,"parent_record_offset":121056,"ancestor_record_offset":119016,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":546,"record_offset":121272,"parent_record_offset":120872,"ancestor_record_offset":119016,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":546,"record_offset":121296,"parent_record_offset":121272,"ancestor_record_offset":119016,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":546,"record_offset":119380,"parent_record_offset":119204,"ancestor_record_offset":119016,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":546,"record_offset":120604,"parent_record_offset":120580,"ancestor_record_offset":119016,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":546,"record_offset":120676,"parent_record_offset":120644,"ancestor_record_offset":119016,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":546,"record_offset":121680,"parent_record_offset":121516,"ancestor_record_offset":119016,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":546,"record_offset":120556,"parent_record_offset":120532,"ancestor_record_offset":119016,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":546,"record_offset":120580,"parent_record_offset":120556,"ancestor_record_offset":119016,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::audio::music::music_player::MusicPlayer::switch_track
// 记录形式: compiled_procedure; instances=1
// 源码行: 223
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":546,"record_offset":122832,"rva":11174592,"va":5379883712}
// 编译实例 RVAs: 0xaa82c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-4205121","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379883712,"raw_hex":"e8bfd5bfff","size":5,"soft_fail":false,"target_va":5375678688,"va":5379883804,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-4205411","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379883712,"raw_hex":"e89dd4bfff","size":5,"soft_fail":false,"target_va":5375678704,"va":5379884110,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: music_player.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11161604","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379883712,"raw_hex":"e8fcaf55ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5379883791,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1416f89e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: music_player.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12912080","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379883712,"raw_hex":"e8d005c500","size":5,"soft_fail":false,"target_va":5392796128,"va":5379884043,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t36736827","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379883712,"raw_hex":"e83b8f3002","size":5,"soft_fail":false,"target_va":5416620794,"va":5379883962,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":546,"record_offset":123004,"parent_record_offset":122832,"ancestor_record_offset":122832,"inlinee":5735,"source_line":3021,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::pop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":546,"record_offset":123032,"parent_record_offset":122832,"ancestor_record_offset":122832,"inlinee":5450,"source_line":2790,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":546,"record_offset":124604,"parent_record_offset":122832,"ancestor_record_offset":122832,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::audio::music::music_player::generate_track_playlist
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/audio/music/music_player.rs
//     evidence={"module":546,"record_offset":123200,"parent_record_offset":122832,"ancestor_record_offset":122832,"inlinee":34003,"source_line":294,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":546,"record_offset":123676,"parent_record_offset":123652,"ancestor_record_offset":122832,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":546,"record_offset":123700,"parent_record_offset":123676,"ancestor_record_offset":122832,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":546,"record_offset":124496,"parent_record_offset":124472,"ancestor_record_offset":122832,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":546,"record_offset":124520,"parent_record_offset":124496,"ancestor_record_offset":122832,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":546,"record_offset":123724,"parent_record_offset":123700,"ancestor_record_offset":122832,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":546,"record_offset":124544,"parent_record_offset":124520,"ancestor_record_offset":122832,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":546,"record_offset":123652,"parent_record_offset":123508,"ancestor_record_offset":122832,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":546,"record_offset":124472,"parent_record_offset":124448,"ancestor_record_offset":122832,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":546,"record_offset":123108,"parent_record_offset":123072,"ancestor_record_offset":122832,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":546,"record_offset":123448,"parent_record_offset":123420,"ancestor_record_offset":122832,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":546,"record_offset":124740,"parent_record_offset":124700,"ancestor_record_offset":122832,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":546,"record_offset":124448,"parent_record_offset":124424,"ancestor_record_offset":122832,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":546,"record_offset":123156,"parent_record_offset":123132,"ancestor_record_offset":122832,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":546,"record_offset":123132,"parent_record_offset":123108,"ancestor_record_offset":122832,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":546,"record_offset":123508,"parent_record_offset":123476,"ancestor_record_offset":122832,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":546,"record_offset":123476,"parent_record_offset":123448,"ancestor_record_offset":122832,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":546,"record_offset":124424,"parent_record_offset":124400,"ancestor_record_offset":122832,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":546,"record_offset":123540,"parent_record_offset":123508,"ancestor_record_offset":122832,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":546,"record_offset":123072,"parent_record_offset":123032,"ancestor_record_offset":122832,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":546,"record_offset":123816,"parent_record_offset":123784,"ancestor_record_offset":122832,"inlinee":5079,"source_line":3988,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":546,"record_offset":123392,"parent_record_offset":123352,"ancestor_record_offset":122832,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":546,"record_offset":123420,"parent_record_offset":123392,"ancestor_record_offset":122832,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":546,"record_offset":123288,"parent_record_offset":123256,"ancestor_record_offset":122832,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$21::extend_trusted::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":546,"record_offset":124096,"parent_record_offset":124068,"ancestor_record_offset":122832,"inlinee":5113,"source_line":4001,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":546,"record_offset":124160,"parent_record_offset":124096,"ancestor_record_offset":122832,"inlinee":5115,"source_line":18,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":546,"record_offset":123784,"parent_record_offset":123352,"ancestor_record_offset":122832,"inlinee":5078,"source_line":26,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":546,"record_offset":123320,"parent_record_offset":123288,"ancestor_record_offset":122832,"inlinee":6133,"source_line":32,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":546,"record_offset":123352,"parent_record_offset":123320,"ancestor_record_offset":122832,"inlinee":6135,"source_line":50,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":546,"record_offset":123564,"parent_record_offset":123540,"ancestor_record_offset":122832,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":546,"record_offset":123996,"parent_record_offset":123956,"ancestor_record_offset":122832,"inlinee":5179,"source_line":211,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":546,"record_offset":123956,"parent_record_offset":123920,"ancestor_record_offset":122832,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":546,"record_offset":123920,"parent_record_offset":123880,"ancestor_record_offset":122832,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":546,"record_offset":123256,"parent_record_offset":123200,"ancestor_record_offset":122832,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":546,"record_offset":123880,"parent_record_offset":123848,"ancestor_record_offset":122832,"inlinee":6141,"source_line":2640,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":546,"record_offset":123848,"parent_record_offset":123816,"ancestor_record_offset":122832,"inlinee":5042,"source_line":835,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":546,"record_offset":124068,"parent_record_offset":123880,"ancestor_record_offset":122832,"inlinee":5057,"source_line":842,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":546,"record_offset":123588,"parent_record_offset":123564,"ancestor_record_offset":122832,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":546,"record_offset":123612,"parent_record_offset":123588,"ancestor_record_offset":122832,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":546,"record_offset":124024,"parent_record_offset":123996,"ancestor_record_offset":122832,"inlinee":5180,"source_line":832,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":546,"record_offset":124780,"parent_record_offset":124740,"ancestor_record_offset":122832,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":546,"record_offset":124312,"parent_record_offset":124284,"ancestor_record_offset":122832,"inlinee":4801,"source_line":623,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":546,"record_offset":124340,"parent_record_offset":124284,"ancestor_record_offset":122832,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":546,"record_offset":124376,"parent_record_offset":123200,"ancestor_record_offset":122832,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::swap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":546,"record_offset":124284,"parent_record_offset":124256,"ancestor_record_offset":122832,"inlinee":6602,"source_line":1299,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":546,"record_offset":124128,"parent_record_offset":124096,"ancestor_record_offset":122832,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::swap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":546,"record_offset":124256,"parent_record_offset":123200,"ancestor_record_offset":122832,"inlinee":6601,"source_line":905,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] fastrand::shuffle
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fastrand","version":"1.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fastrand-1.8.0/src/lib.rs
//     evidence={"module":546,"record_offset":124228,"parent_record_offset":123200,"ancestor_record_offset":122832,"inlinee":11875,"source_line":642,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::audio::music::music_player::MusicPlayer::current_track
// 记录形式: inline_instance, inlinee_declaration; instances=8
// 源码行: 232
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x84D5
// 首条位置: {"module":546,"inlinee":34005}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::slice::impl$0::get
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":546,"record_offset":128684,"parent_record_offset":128656,"ancestor_record_offset":128656,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":546,"record_offset":128712,"parent_record_offset":128684,"ancestor_record_offset":128656,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::audio::music::music_player::MusicPlayer::stop_immediately
// 记录形式: compiled_procedure; instances=1
// 源码行: 237
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":546,"record_offset":124944,"rva":11175120,"va":5379884240}
// 编译实例 RVAs: 0xaa84d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140a7cbd0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: music_player.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-178801","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379884240,"raw_hex":"e88f45fdff","size":5,"soft_fail":false,"target_va":5379705808,"va":5379884604,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t36737188","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379884240,"raw_hex":"e8a4903002","size":5,"soft_fail":false,"target_va":5416621632,"va":5379884439,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] libfmod::EventInstance::stop
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"libfmod","version":"2.222.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t16769602","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379884240,"raw_hex":"e842e2ff00","size":5,"soft_fail":false,"target_va":5396653968,"va":5379884361,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":546,"record_offset":125112,"parent_record_offset":124944,"ancestor_record_offset":124944,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::audio_resources::AudioInstance::stop_immediately
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/audio_resources.rs
//     evidence={"module":546,"record_offset":125176,"parent_record_offset":124944,"ancestor_record_offset":124944,"inlinee":33507,"source_line":544,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":546,"record_offset":125292,"parent_record_offset":125256,"ancestor_record_offset":124944,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":546,"record_offset":125140,"parent_record_offset":125112,"ancestor_record_offset":124944,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":546,"record_offset":125212,"parent_record_offset":125176,"ancestor_record_offset":124944,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::audio::music::music_player::MusicPlayer::pause
// 记录形式: compiled_procedure; instances=1
// 源码行: 245
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":546,"record_offset":128496,"rva":11177776,"va":5379886896}
// 编译实例 RVAs: 0xaa8f30
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140a7cbd0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: music_player.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-181435","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379886896,"raw_hex":"e8453bfdff","size":5,"soft_fail":false,"target_va":5379705808,"va":5379887238,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t36734501","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379886896,"raw_hex":"e825863002","size":5,"soft_fail":false,"target_va":5416621632,"va":5379887126,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] libfmod::EventInstance::set_paused
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"libfmod","version":"2.222.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t16763098","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379886896,"raw_hex":"e8dac8ff00","size":5,"soft_fail":false,"target_va":5396650080,"va":5379886977,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::resources::audio_resources::AudioInstance::pause
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/audio_resources.rs
//     evidence={"module":546,"record_offset":128752,"parent_record_offset":128496,"ancestor_record_offset":128496,"inlinee":33512,"source_line":562,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::audio::music::music_player::MusicPlayer::current_track
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/audio/music/music_player.rs
//     evidence={"module":546,"record_offset":128656,"parent_record_offset":128496,"ancestor_record_offset":128496,"inlinee":34005,"source_line":232,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":546,"record_offset":128784,"parent_record_offset":128752,"ancestor_record_offset":128496,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":546,"record_offset":128812,"parent_record_offset":128752,"ancestor_record_offset":128496,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":546,"record_offset":128684,"parent_record_offset":128656,"ancestor_record_offset":128496,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":546,"record_offset":128712,"parent_record_offset":128684,"ancestor_record_offset":128496,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::audio::music::music_player::MusicPlayer::probabilistic_switch_track
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 252
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x84D8
// 首条位置: {"module":546,"inlinee":34008}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::f32::impl$0::powi
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":546,"record_offset":130924,"parent_record_offset":130896,"ancestor_record_offset":130896,"inlinee":17052,"source_line":323,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::powi
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":546,"record_offset":130948,"parent_record_offset":130924,"ancestor_record_offset":130896,"inlinee":17053,"source_line":2010,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::audio::music::music_player::get_timeline_beat_properties
// 记录形式: compiled_procedure; instances=1
// 源码行: 269
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":546,"record_offset":131660,"rva":11181312,"va":5379890432}
// 编译实例 RVAs: 0xaa9d00
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::Ord::max
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":546,"record_offset":131892,"parent_record_offset":131660,"ancestor_record_offset":131660,"inlinee":5501,"source_line":1033,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::const_ptr::impl$0::is_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":546,"record_offset":131828,"parent_record_offset":131660,"ancestor_record_offset":131660,"inlinee":19030,"source_line":22,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::is_null::runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":546,"record_offset":131856,"parent_record_offset":131828,"ancestor_record_offset":131660,"inlinee":19032,"source_line":2434,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::audio::music::music_player::generate_track_playlist
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 294
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x84D3
// 首条位置: {"module":546,"inlinee":34003}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":546,"record_offset":123256,"parent_record_offset":123200,"ancestor_record_offset":123200,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":546,"record_offset":124376,"parent_record_offset":123200,"ancestor_record_offset":123200,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::swap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":546,"record_offset":124256,"parent_record_offset":123200,"ancestor_record_offset":123200,"inlinee":6601,"source_line":905,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_direct] fastrand::shuffle
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fastrand","version":"1.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fastrand-1.8.0/src/lib.rs
//     evidence={"module":546,"record_offset":124228,"parent_record_offset":123200,"ancestor_record_offset":123200,"inlinee":11875,"source_line":642,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":546,"record_offset":123676,"parent_record_offset":123652,"ancestor_record_offset":123200,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":546,"record_offset":123700,"parent_record_offset":123676,"ancestor_record_offset":123200,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":546,"record_offset":124496,"parent_record_offset":124472,"ancestor_record_offset":123200,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":546,"record_offset":124520,"parent_record_offset":124496,"ancestor_record_offset":123200,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":546,"record_offset":123724,"parent_record_offset":123700,"ancestor_record_offset":123200,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":546,"record_offset":124544,"parent_record_offset":124520,"ancestor_record_offset":123200,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":546,"record_offset":123652,"parent_record_offset":123508,"ancestor_record_offset":123200,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":546,"record_offset":124472,"parent_record_offset":124448,"ancestor_record_offset":123200,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":546,"record_offset":123448,"parent_record_offset":123420,"ancestor_record_offset":123200,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":546,"record_offset":124448,"parent_record_offset":124424,"ancestor_record_offset":123200,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":546,"record_offset":123508,"parent_record_offset":123476,"ancestor_record_offset":123200,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":546,"record_offset":123476,"parent_record_offset":123448,"ancestor_record_offset":123200,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":546,"record_offset":124424,"parent_record_offset":124400,"ancestor_record_offset":123200,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":546,"record_offset":123540,"parent_record_offset":123508,"ancestor_record_offset":123200,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":546,"record_offset":123816,"parent_record_offset":123784,"ancestor_record_offset":123200,"inlinee":5079,"source_line":3988,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":546,"record_offset":123392,"parent_record_offset":123352,"ancestor_record_offset":123200,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":546,"record_offset":123420,"parent_record_offset":123392,"ancestor_record_offset":123200,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":546,"record_offset":123288,"parent_record_offset":123256,"ancestor_record_offset":123200,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$21::extend_trusted::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":546,"record_offset":124096,"parent_record_offset":124068,"ancestor_record_offset":123200,"inlinee":5113,"source_line":4001,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":546,"record_offset":124160,"parent_record_offset":124096,"ancestor_record_offset":123200,"inlinee":5115,"source_line":18,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":546,"record_offset":123784,"parent_record_offset":123352,"ancestor_record_offset":123200,"inlinee":5078,"source_line":26,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":546,"record_offset":123320,"parent_record_offset":123288,"ancestor_record_offset":123200,"inlinee":6133,"source_line":32,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":546,"record_offset":123352,"parent_record_offset":123320,"ancestor_record_offset":123200,"inlinee":6135,"source_line":50,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":546,"record_offset":123564,"parent_record_offset":123540,"ancestor_record_offset":123200,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":546,"record_offset":123996,"parent_record_offset":123956,"ancestor_record_offset":123200,"inlinee":5179,"source_line":211,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":546,"record_offset":123956,"parent_record_offset":123920,"ancestor_record_offset":123200,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":546,"record_offset":123920,"parent_record_offset":123880,"ancestor_record_offset":123200,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":546,"record_offset":123880,"parent_record_offset":123848,"ancestor_record_offset":123200,"inlinee":6141,"source_line":2640,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":546,"record_offset":123848,"parent_record_offset":123816,"ancestor_record_offset":123200,"inlinee":5042,"source_line":835,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":546,"record_offset":124068,"parent_record_offset":123880,"ancestor_record_offset":123200,"inlinee":5057,"source_line":842,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":546,"record_offset":123588,"parent_record_offset":123564,"ancestor_record_offset":123200,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":546,"record_offset":123612,"parent_record_offset":123588,"ancestor_record_offset":123200,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":546,"record_offset":124024,"parent_record_offset":123996,"ancestor_record_offset":123200,"inlinee":5180,"source_line":832,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":546,"record_offset":124312,"parent_record_offset":124284,"ancestor_record_offset":123200,"inlinee":4801,"source_line":623,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":546,"record_offset":124340,"parent_record_offset":124284,"ancestor_record_offset":123200,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":546,"record_offset":124400,"parent_record_offset":124376,"ancestor_record_offset":123200,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::swap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":546,"record_offset":124284,"parent_record_offset":124256,"ancestor_record_offset":123200,"inlinee":6602,"source_line":1299,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":546,"record_offset":124128,"parent_record_offset":124096,"ancestor_record_offset":123200,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00546.jsonl.gz"}
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
// 完整结构及原始类型记录: music_player.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::audio::music::music_player::MusicPlayer
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 12; 过程引用数: 7
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: music_player.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: music_player.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: country_core::systems::audio::music::music_player::MusicPlayer::current_track
// 信息 ID: f555ac677c83e6463b7e8d08
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 6 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::audio::music::music_player::MusicPlayer::pause; module=546, inline_record=128656; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::music::music_player::MusicPlayer::update; module=546, inline_record=129168; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::music::music_player::MusicPlayer::update; module=546, inline_record=129488; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::music::music_player::MusicPlayer::update; module=546, inline_record=130640; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::music::music_player::MusicPlayer::update; module=546, inline_record=131104; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<country_core::systems::audio::music::music_debug_gui::music_debug_gui::closure_env$0,tuple$<ref_mut$<egui::ui::Ui> > >; module=1762, inline_record=77360; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::audio::music::music_player::MusicPlayer::init
// 信息 ID: e98d93eb2355f23c486a0b5c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::audio::music::music_system::music_audio_init_system @ 0xadbc8b -> 0xaa8800 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::audio::music::music_player::MusicPlayer::load_tracks
// 信息 ID: 97999fabb42233b967eb2607
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::startup::load_glade_audio::load_glade_audio @ 0x1b26fe2 -> 0xaa7d30 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::audio::music::music_player::MusicPlayer::pause
// 信息 ID: de39049b71be456b1273e013
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::audio::music::music_system::music_audio_system @ 0xadb051 -> 0xaa8f30 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::audio::music::music_player::MusicPlayer::probabilistic_switch_track
// 信息 ID: 5e1a861a90032bf4a45b43d1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::audio::music::music_player::MusicPlayer::update; module=546, inline_record=130896; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::audio::music::music_player::MusicPlayer::stop_immediately
// 信息 ID: 6a50ca04ddd68a6cb7c7d053
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::startup::load_glade_audio::load_glade_audio @ 0x1b26dad -> 0xaa84d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::audio::music::music_player::MusicPlayer::stop_photomode_snapshots
// 信息 ID: d4ef2360d6a3261c52ea73fd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::audio::music::music_system::music_audio_system @ 0xadb0c5 -> 0xaa8680 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::music::music_system::music_audio_system @ 0xadb05e -> 0xaa8680 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::audio::music::music_player::MusicPlayer::switch_track
// 信息 ID: eb5418559be780013702991d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::audio::music::music_player::MusicPlayer::update @ 0xaa91bb -> 0xaa82c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::audio::music::music_player::MusicPlayer::trigger_fpv_snapshot
// 信息 ID: 5b926e6fefe93a3be6513c4a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::audio::music::music_system::music_audio_system; module=553, inline_record=76560; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::audio::music::music_player::MusicPlayer::trigger_photomode_minus_fpv_snapshot
// 信息 ID: 57e2eab36f4e763dd66e1f1c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::audio::music::music_system::music_audio_system; module=553, inline_record=75984; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::audio::music::music_player::MusicPlayer::update
// 信息 ID: 8ae42bb801a7c238eb3a410c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::audio::music::music_system::music_audio_system @ 0xadaf8f -> 0xaa90d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::audio::music::music_player::generate_track_playlist
// 信息 ID: 1ffdf1b614054e718ec304ee
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::audio::music::music_player::MusicPlayer::switch_track; module=546, inline_record=123200; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::audio::music::music_player::get_timeline_beat_properties
// 信息 ID: 4d035b6cf05e9571c113e569
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::audio::music::music_player::impl$0::update::closure$0
// 信息 ID: 0fe207a73da1ebdb5f5630af
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::audio::music::music_player::impl$0::update", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::audio::music::music_player::MusicPlayer::update; module=546, inline_record=129284; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::audio::music::music_player::impl$2::fmt
// 信息 ID: 449102ebd24f60f1f92042e3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===

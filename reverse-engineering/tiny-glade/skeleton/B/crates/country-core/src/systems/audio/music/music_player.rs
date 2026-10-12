// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/systems/audio/music/music_player.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 4258b6fa0530041ce3c245e94ba1539d4eae3b5c0ca8f047d5ca57d12abbcce5
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: music_player.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 476, 491, 553
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use anyhow; // namespace candidate
//   package identities: [{"name":"anyhow","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.86"}]
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
// 首条位置: {"module":476,"record_offset":79340,"rva":8650432,"va":5377359552}
// 编译实例 RVAs: 0x83fec0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::fmt::Formatter::debug_struct_field1_finish
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5532652","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377359552,"raw_hex":"e8ec6b5400","size":5,"soft_fail":false,"target_va":5382892528,"va":5377359871,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::fmt::Formatter::debug_struct_field2_finish
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5532927","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377359552,"raw_hex":"e8ff6c5400","size":5,"soft_fail":false,"target_va":5382892704,"va":5377359772,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] alias_set_at_0x140d86b90
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: music_player.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t5532830","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5377359552,"raw_hex":"e99e6c5400","size":5,"soft_fail":false,"target_va":5382892432,"va":5377359597,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
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
// 首条位置: {"module":491,"record_offset":16088,"rva":9131264,"va":5377840384}
// 编译实例 RVAs: 0x8b5500
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4803343","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377840384,"raw_hex":"e80f4b4900","size":5,"soft_fail":false,"target_va":5382644528,"va":5377841180,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067dd80
//     owner=unresolved_namespace; count=1
//     target_aliases=69 candidates; complete list: music_player.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2325031","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377840384,"raw_hex":"e8d985dcff","size":5,"soft_fail":false,"target_va":5375516032,"va":5377841058,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a526b0
//     owner=unresolved_namespace; count=15
//     target_aliases=2 candidates; complete list: music_player.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1691941","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377840384,"raw_hex":"e825d11900","size":5,"soft_fail":false,"target_va":5379532464,"va":5377840518,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923e60
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: music_player.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t34005763","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377840384,"raw_hex":"e803e30602","size":5,"soft_fail":false,"target_va":5411847776,"va":5377842008,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=14
//     evidence={"assembly":"\tcallq\t33701854","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377840384,"raw_hex":"e8de3f0202","size":5,"soft_fail":false,"target_va":5411542880,"va":5377841021,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::audio::music::music_player::MusicPlayer::switch_track
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t6787","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377840384,"raw_hex":"e8831a0000","size":5,"soft_fail":false,"target_va":5377847504,"va":5377840712,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] fastrand::f32
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"fastrand","version":"1.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"fastrand","version":"2.3.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4167695","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377840384,"raw_hex":"e80f983f00","size":5,"soft_fail":false,"target_va":5382009792,"va":5377842092,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] libfmod::EventInstance::get_paused
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"libfmod","version":"2.222.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t12063666","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377840384,"raw_hex":"e8b213b800","size":5,"soft_fail":false,"target_va":5389905888,"va":5377842217,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] libfmod::EventInstance::get_timeline_position
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"libfmod","version":"2.222.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t12066883","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377840384,"raw_hex":"e84320b800","size":5,"soft_fail":false,"target_va":5389907376,"va":5377840488,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] libfmod::EventInstance::set_paused
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"libfmod","version":"2.222.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t12064002","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377840384,"raw_hex":"e80215b800","size":5,"soft_fail":false,"target_va":5389906272,"va":5377842265,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] libfmod::EventInstance::start
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"libfmod","version":"2.222.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t12065862","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377840384,"raw_hex":"e8461cb800","size":5,"soft_fail":false,"target_va":5389906640,"va":5377840773,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] libfmod::EventInstance::stop
//     owner=package_namespace_candidate; count=8
//     packages=[{"name":"libfmod","version":"2.222.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t12066367","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377840384,"raw_hex":"e83f1eb800","size":5,"soft_fail":false,"target_va":5389907008,"va":5377840636,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":491,"record_offset":17240,"parent_record_offset":16088,"ancestor_record_offset":16088,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":491,"record_offset":16380,"parent_record_offset":16088,"ancestor_record_offset":16088,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::audio_resources::AudioInstance::is_paused
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/audio_resources.rs
//     evidence={"module":491,"record_offset":18496,"parent_record_offset":16088,"ancestor_record_offset":16088,"inlinee":27236,"source_line":572,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::audio_resources::AudioInstance::resume
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/audio_resources.rs
//     evidence={"module":491,"record_offset":18612,"parent_record_offset":16088,"ancestor_record_offset":16088,"inlinee":27237,"source_line":581,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::audio_resources::AudioInstance::start
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/audio_resources.rs
//     evidence={"module":491,"record_offset":16820,"parent_record_offset":16088,"ancestor_record_offset":16088,"inlinee":27234,"source_line":505,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::audio_resources::AudioInstance::stop
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/audio_resources.rs
//     evidence={"module":491,"record_offset":16616,"parent_record_offset":16088,"ancestor_record_offset":16088,"inlinee":27233,"source_line":532,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::audio_resources::AudioInstance::stop_immediately
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/audio_resources.rs
//     evidence={"module":491,"record_offset":17448,"parent_record_offset":16088,"ancestor_record_offset":16088,"inlinee":26124,"source_line":544,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::audio::music::music_player::MusicPlayer::current_track
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/audio/music/music_player.rs
//     evidence={"module":491,"record_offset":16288,"parent_record_offset":16088,"ancestor_record_offset":16088,"inlinee":26126,"source_line":232,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::audio::music::music_player::MusicPlayer::probabilistic_switch_track
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/audio/music/music_player.rs
//     evidence={"module":491,"record_offset":18232,"parent_record_offset":16088,"ancestor_record_offset":16088,"inlinee":27235,"source_line":252,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":491,"record_offset":17272,"parent_record_offset":16088,"ancestor_record_offset":16088,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":491,"record_offset":17156,"parent_record_offset":16088,"ancestor_record_offset":16088,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::ceil
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":491,"record_offset":18204,"parent_record_offset":16088,"ancestor_record_offset":16088,"inlinee":13632,"source_line":70,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":491,"record_offset":16580,"parent_record_offset":16408,"ancestor_record_offset":16088,"inlinee":6234,"source_line":998,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":491,"record_offset":16460,"parent_record_offset":16432,"ancestor_record_offset":16088,"inlinee":4518,"source_line":1447,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":491,"record_offset":16656,"parent_record_offset":16616,"ancestor_record_offset":16088,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":491,"record_offset":17328,"parent_record_offset":17296,"ancestor_record_offset":16088,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":491,"record_offset":17356,"parent_record_offset":17296,"ancestor_record_offset":16088,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":491,"record_offset":16536,"parent_record_offset":16508,"ancestor_record_offset":16088,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":491,"record_offset":16508,"parent_record_offset":16488,"ancestor_record_offset":16088,"inlinee":4624,"source_line":658,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":491,"record_offset":16684,"parent_record_offset":16616,"ancestor_record_offset":16088,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":491,"record_offset":16312,"parent_record_offset":16288,"ancestor_record_offset":16088,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":491,"record_offset":16340,"parent_record_offset":16312,"ancestor_record_offset":16088,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":491,"record_offset":17180,"parent_record_offset":17156,"ancestor_record_offset":16088,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":491,"record_offset":17204,"parent_record_offset":17180,"ancestor_record_offset":16088,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::audio_resources::AudioInstance::get_timeline_position
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/audio_resources.rs
//     evidence={"module":491,"record_offset":16432,"parent_record_offset":16408,"ancestor_record_offset":16088,"inlinee":27230,"source_line":603,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::audio_resources::impl$9::get_timeline_position::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/audio_resources.rs
//     evidence={"module":491,"record_offset":16488,"parent_record_offset":16460,"ancestor_record_offset":16088,"inlinee":27232,"source_line":605,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::audio::music::music_player::impl$0::update::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/audio/music/music_player.rs
//     evidence={"module":491,"record_offset":16408,"parent_record_offset":16380,"ancestor_record_offset":16088,"inlinee":27229,"source_line":54,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":491,"record_offset":17384,"parent_record_offset":17296,"ancestor_record_offset":16088,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":491,"record_offset":17296,"parent_record_offset":17272,"ancestor_record_offset":16088,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":491,"record_offset":17408,"parent_record_offset":17384,"ancestor_record_offset":16088,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::powi
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":491,"record_offset":18260,"parent_record_offset":18232,"ancestor_record_offset":16088,"inlinee":14226,"source_line":314,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::audio::music::music_player::impl$0::update::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 54
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6A5D
// 首条位置: {"module":491,"inlinee":27229}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::Ord::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":491,"record_offset":16580,"parent_record_offset":16408,"ancestor_record_offset":16408,"inlinee":6234,"source_line":998,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::audio_resources::AudioInstance::get_timeline_position
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/audio_resources.rs
//     evidence={"module":491,"record_offset":16432,"parent_record_offset":16408,"ancestor_record_offset":16408,"inlinee":27230,"source_line":603,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":491,"record_offset":16460,"parent_record_offset":16432,"ancestor_record_offset":16408,"inlinee":4518,"source_line":1447,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":491,"record_offset":16536,"parent_record_offset":16508,"ancestor_record_offset":16408,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":491,"record_offset":16508,"parent_record_offset":16488,"ancestor_record_offset":16408,"inlinee":4624,"source_line":658,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::audio_resources::impl$9::get_timeline_position::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/audio_resources.rs
//     evidence={"module":491,"record_offset":16488,"parent_record_offset":16460,"ancestor_record_offset":16408,"inlinee":27232,"source_line":605,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::audio::music::music_player::MusicPlayer::trigger_photomode_minus_fpv_snapshot
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 162
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6A80
// 首条位置: {"module":553,"inlinee":27264}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::resources::audio_resources::AudioInstance::stop_immediately
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/audio_resources.rs
//     evidence={"module":553,"record_offset":42252,"parent_record_offset":42220,"ancestor_record_offset":42220,"inlinee":26124,"source_line":544,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":553,"record_offset":42284,"parent_record_offset":42252,"ancestor_record_offset":42220,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":553,"record_offset":42312,"parent_record_offset":42252,"ancestor_record_offset":42220,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::audio::music::music_player::MusicPlayer::trigger_fpv_snapshot
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 167
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6A81
// 首条位置: {"module":553,"inlinee":27265}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::resources::audio_resources::AudioInstance::stop_immediately
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/audio_resources.rs
//     evidence={"module":553,"record_offset":43032,"parent_record_offset":43000,"ancestor_record_offset":43000,"inlinee":26124,"source_line":544,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":553,"record_offset":43064,"parent_record_offset":43032,"ancestor_record_offset":43000,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":553,"record_offset":43092,"parent_record_offset":43032,"ancestor_record_offset":43000,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00553.jsonl.gz"}
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
// 首条位置: {"module":491,"record_offset":18908,"rva":9134672,"va":5377843792}
// 编译实例 RVAs: 0x8b6250
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140a526b0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: music_player.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1688373","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377843792,"raw_hex":"e835c31900","size":5,"soft_fail":false,"target_va":5379532464,"va":5377844086,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t33698893","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377843792,"raw_hex":"e84d340202","size":5,"soft_fail":false,"target_va":5411542880,"va":5377843982,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] libfmod::EventInstance::stop
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"libfmod","version":"2.222.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t12063166","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377843792,"raw_hex":"e8be11b800","size":5,"soft_fail":false,"target_va":5389907008,"va":5377843837,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::resources::audio_resources::AudioInstance::stop
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/audio_resources.rs
//     evidence={"module":491,"record_offset":19080,"parent_record_offset":18908,"ancestor_record_offset":18908,"inlinee":27233,"source_line":532,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":491,"record_offset":19116,"parent_record_offset":19080,"ancestor_record_offset":18908,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":491,"record_offset":19144,"parent_record_offset":19080,"ancestor_record_offset":18908,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00491.jsonl.gz"}
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
// 首条位置: {"module":491,"record_offset":19300,"rva":9135040,"va":5377844160}
// 编译实例 RVAs: 0x8b63c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14014a680
//     owner=unresolved_namespace; count=3
//     target_aliases=24 candidates; complete list: music_player.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7783308","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377844160,"raw_hex":"e8743c89ff","size":5,"soft_fail":false,"target_va":5370062464,"va":5377845767,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: music_player.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2763032","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377844160,"raw_hex":"e8e8d6d5ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5377844515,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140899740
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: music_player.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-119404","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377844160,"raw_hex":"e8942dfeff","size":5,"soft_fail":false,"target_va":5377726272,"va":5377845671,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t33695393","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377844160,"raw_hex":"e8a1260202","size":5,"soft_fail":false,"target_va":5411540963,"va":5377845565,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4809300","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377844160,"raw_hex":"e854624900","size":5,"soft_fail":false,"target_va":5382655024,"va":5377845719,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t33698387","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377844160,"raw_hex":"e853320202","size":5,"soft_fail":false,"target_va":5411542880,"va":5377844488,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::audio_resources::AudioCoreInner::instantiate_audio
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1695775","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377844160,"raw_hex":"e81fe01900","size":5,"soft_fail":false,"target_va":5379540032,"va":5377844252,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":491,"record_offset":19460,"parent_record_offset":19300,"ancestor_record_offset":19300,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":491,"record_offset":20092,"parent_record_offset":19300,"ancestor_record_offset":19300,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::audio_resources::AudioCore::instantiate_audio
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/audio_resources.rs
//     evidence={"module":491,"record_offset":19488,"parent_record_offset":19300,"ancestor_record_offset":19300,"inlinee":27239,"source_line":321,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":491,"record_offset":19904,"parent_record_offset":19880,"ancestor_record_offset":19300,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":491,"record_offset":19928,"parent_record_offset":19904,"ancestor_record_offset":19300,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":491,"record_offset":19880,"parent_record_offset":19852,"ancestor_record_offset":19300,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":491,"record_offset":19792,"parent_record_offset":19764,"ancestor_record_offset":19300,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":491,"record_offset":19852,"parent_record_offset":19820,"ancestor_record_offset":19300,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":491,"record_offset":19820,"parent_record_offset":19792,"ancestor_record_offset":19300,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":491,"record_offset":19728,"parent_record_offset":19700,"ancestor_record_offset":19300,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":491,"record_offset":19700,"parent_record_offset":19672,"ancestor_record_offset":19300,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":491,"record_offset":19644,"parent_record_offset":19616,"ancestor_record_offset":19300,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":491,"record_offset":19672,"parent_record_offset":19644,"ancestor_record_offset":19300,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":491,"record_offset":19616,"parent_record_offset":19588,"ancestor_record_offset":19300,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":491,"record_offset":19588,"parent_record_offset":19488,"ancestor_record_offset":19300,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":491,"record_offset":19560,"parent_record_offset":19488,"ancestor_record_offset":19300,"inlinee":5345,"source_line":2205,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":491,"record_offset":19764,"parent_record_offset":19728,"ancestor_record_offset":19300,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Ok
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/lib.rs
//     evidence={"module":491,"record_offset":20060,"parent_record_offset":19488,"ancestor_record_offset":19300,"inlinee":27241,"source_line":647,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":491,"record_offset":20004,"parent_record_offset":19980,"ancestor_record_offset":19300,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":491,"record_offset":19532,"parent_record_offset":19488,"ancestor_record_offset":19300,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":491,"record_offset":19980,"parent_record_offset":19728,"ancestor_record_offset":19300,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":491,"record_offset":20136,"parent_record_offset":20092,"ancestor_record_offset":19300,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00491.jsonl.gz"}
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
// 首条位置: {"module":491,"record_offset":22040,"rva":9136912,"va":5377846032}
// 编译实例 RVAs: 0x8b6b10
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$alloc..string..String$u20$as$u20$core..clone..Clone$GT$::clone
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t5001079","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377846032,"raw_hex":"e8774f4c00","size":5,"soft_fail":false,"target_va":5382847888,"va":5377846804,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14014a680
//     owner=unresolved_namespace; count=3
//     target_aliases=24 candidates; complete list: music_player.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7784654","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377846032,"raw_hex":"e8323789ff","size":5,"soft_fail":false,"target_va":5370062464,"va":5377847113,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: music_player.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2764988","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377846032,"raw_hex":"e844cfd5ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5377846471,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=6
//     target_aliases=1 candidates; complete list: music_player.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2764689","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377846032,"raw_hex":"e86fd0d5ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5377846188,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14089a280
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: music_player.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-117010","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377846032,"raw_hex":"e8ee36feff","size":5,"soft_fail":false,"target_va":5377729152,"va":5377846157,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14157c850
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: music_player.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13392592","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377846032,"raw_hex":"e8d05acc00","size":5,"soft_fail":false,"target_va":5391239248,"va":5377846651,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: music_player.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t34000651","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377846032,"raw_hex":"e80bcf0602","size":5,"soft_fail":false,"target_va":5411847168,"va":5377846512,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-3490791","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377846032,"raw_hex":"e819bccaff","size":5,"soft_fail":false,"target_va":5374355952,"va":5377846738,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33693938","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377846032,"raw_hex":"e8f2200202","size":5,"soft_fail":false,"target_va":5411540963,"va":5377847020,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4807547","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377846032,"raw_hex":"e87b5b4900","size":5,"soft_fail":false,"target_va":5382655024,"va":5377847472,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,enum2$<libfmod::Error> > > >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-10188","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377846032,"raw_hex":"e834d8ffff","size":5,"soft_fail":false,"target_va":5377836480,"va":5377846663,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33695878","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377846032,"raw_hex":"e886280202","size":5,"soft_fail":false,"target_va":5411542880,"va":5377846997,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::audio_resources::AudioCoreInner::instantiate_audio
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1693623","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377846032,"raw_hex":"e8b7d71900","size":5,"soft_fail":false,"target_va":5379540032,"va":5377846404,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":491,"record_offset":22228,"parent_record_offset":22040,"ancestor_record_offset":22040,"inlinee":4586,"source_line":2652,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":491,"record_offset":24396,"parent_record_offset":22040,"ancestor_record_offset":22040,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":491,"record_offset":25016,"parent_record_offset":22040,"ancestor_record_offset":22040,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::is_ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":491,"record_offset":24368,"parent_record_offset":22040,"ancestor_record_offset":22040,"inlinee":6884,"source_line":564,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":491,"record_offset":24288,"parent_record_offset":22040,"ancestor_record_offset":22040,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":491,"record_offset":23500,"parent_record_offset":22040,"ancestor_record_offset":22040,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$1::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":491,"record_offset":23388,"parent_record_offset":22040,"ancestor_record_offset":22040,"inlinee":10157,"source_line":25,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::audio_resources::AudioCore::instantiate_audio
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/audio_resources.rs
//     evidence={"module":491,"record_offset":23592,"parent_record_offset":22040,"ancestor_record_offset":22040,"inlinee":27239,"source_line":321,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":491,"record_offset":24100,"parent_record_offset":24076,"ancestor_record_offset":22040,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":491,"record_offset":24124,"parent_record_offset":24100,"ancestor_record_offset":22040,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":491,"record_offset":22640,"parent_record_offset":22616,"ancestor_record_offset":22040,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":491,"record_offset":24076,"parent_record_offset":23900,"ancestor_record_offset":22040,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":491,"record_offset":22616,"parent_record_offset":22560,"ancestor_record_offset":22040,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":491,"record_offset":22312,"parent_record_offset":22288,"ancestor_record_offset":22040,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":491,"record_offset":23840,"parent_record_offset":23812,"ancestor_record_offset":22040,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":491,"record_offset":22588,"parent_record_offset":22560,"ancestor_record_offset":22040,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":491,"record_offset":22560,"parent_record_offset":22536,"ancestor_record_offset":22040,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":491,"record_offset":22360,"parent_record_offset":22336,"ancestor_record_offset":22040,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":491,"record_offset":22336,"parent_record_offset":22312,"ancestor_record_offset":22040,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":491,"record_offset":23900,"parent_record_offset":23868,"ancestor_record_offset":22040,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":491,"record_offset":23868,"parent_record_offset":23840,"ancestor_record_offset":22040,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":491,"record_offset":22536,"parent_record_offset":22512,"ancestor_record_offset":22040,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":491,"record_offset":23936,"parent_record_offset":23900,"ancestor_record_offset":22040,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":491,"record_offset":23776,"parent_record_offset":23748,"ancestor_record_offset":22040,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":491,"record_offset":23748,"parent_record_offset":23720,"ancestor_record_offset":22040,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":491,"record_offset":23692,"parent_record_offset":23664,"ancestor_record_offset":22040,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":491,"record_offset":23720,"parent_record_offset":23692,"ancestor_record_offset":22040,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":491,"record_offset":23664,"parent_record_offset":23636,"ancestor_record_offset":22040,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":491,"record_offset":23636,"parent_record_offset":23592,"ancestor_record_offset":22040,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":491,"record_offset":22288,"parent_record_offset":22264,"ancestor_record_offset":22040,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":491,"record_offset":22264,"parent_record_offset":22228,"ancestor_record_offset":22040,"inlinee":5037,"source_line":1593,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":491,"record_offset":23812,"parent_record_offset":23776,"ancestor_record_offset":22040,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Ok
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/lib.rs
//     evidence={"module":491,"record_offset":24256,"parent_record_offset":23592,"ancestor_record_offset":22040,"inlinee":27241,"source_line":647,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":491,"record_offset":23960,"parent_record_offset":23936,"ancestor_record_offset":22040,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":491,"record_offset":23984,"parent_record_offset":23960,"ancestor_record_offset":22040,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":491,"record_offset":24200,"parent_record_offset":24176,"ancestor_record_offset":22040,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":491,"record_offset":24032,"parent_record_offset":24008,"ancestor_record_offset":22040,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":491,"record_offset":24008,"parent_record_offset":23984,"ancestor_record_offset":22040,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":491,"record_offset":24176,"parent_record_offset":23776,"ancestor_record_offset":22040,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=22
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":491,"record_offset":22404,"parent_record_offset":22228,"ancestor_record_offset":22040,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":491,"record_offset":23460,"parent_record_offset":23436,"ancestor_record_offset":22040,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":491,"record_offset":23552,"parent_record_offset":23500,"ancestor_record_offset":22040,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":491,"record_offset":24560,"parent_record_offset":24396,"ancestor_record_offset":22040,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":491,"record_offset":23412,"parent_record_offset":23388,"ancestor_record_offset":22040,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":491,"record_offset":23436,"parent_record_offset":23412,"ancestor_record_offset":22040,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/00491.jsonl.gz"}
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
// 首条位置: {"module":491,"record_offset":25508,"rva":9138384,"va":5377847504}
// 编译实例 RVAs: 0x8b70d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: music_player.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2766115","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377847504,"raw_hex":"e8ddcad5ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5377847598,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: music_player.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2766409","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377847504,"raw_hex":"e8b7c9d5ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5377847908,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33693197","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377847504,"raw_hex":"e80d1e0202","size":5,"soft_fail":false,"target_va":5411540963,"va":5377847761,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::LocalKey$LT$T$GT$::with
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t10960761","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377847504,"raw_hex":"e8793fa700","size":5,"soft_fail":false,"target_va":5388808608,"va":5377847842,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":491,"record_offset":25680,"parent_record_offset":25508,"ancestor_record_offset":25508,"inlinee":4613,"source_line":2705,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::pop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":491,"record_offset":25708,"parent_record_offset":25508,"ancestor_record_offset":25508,"inlinee":6530,"source_line":2503,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":491,"record_offset":27224,"parent_record_offset":25508,"ancestor_record_offset":25508,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::audio::music::music_player::generate_track_playlist
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/audio/music/music_player.rs
//     evidence={"module":491,"record_offset":25876,"parent_record_offset":25508,"ancestor_record_offset":25508,"inlinee":27243,"source_line":294,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":491,"record_offset":26380,"parent_record_offset":26356,"ancestor_record_offset":25508,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":491,"record_offset":26404,"parent_record_offset":26380,"ancestor_record_offset":25508,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":491,"record_offset":27172,"parent_record_offset":27148,"ancestor_record_offset":25508,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":491,"record_offset":26356,"parent_record_offset":26184,"ancestor_record_offset":25508,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":491,"record_offset":27148,"parent_record_offset":27124,"ancestor_record_offset":25508,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":491,"record_offset":25784,"parent_record_offset":25748,"ancestor_record_offset":25508,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":491,"record_offset":26124,"parent_record_offset":26096,"ancestor_record_offset":25508,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":491,"record_offset":27360,"parent_record_offset":27320,"ancestor_record_offset":25508,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":491,"record_offset":27124,"parent_record_offset":27100,"ancestor_record_offset":25508,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":491,"record_offset":25832,"parent_record_offset":25808,"ancestor_record_offset":25508,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":491,"record_offset":25808,"parent_record_offset":25784,"ancestor_record_offset":25508,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":491,"record_offset":26184,"parent_record_offset":26152,"ancestor_record_offset":25508,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":491,"record_offset":26152,"parent_record_offset":26124,"ancestor_record_offset":25508,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":491,"record_offset":27100,"parent_record_offset":27076,"ancestor_record_offset":25508,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":491,"record_offset":26216,"parent_record_offset":26184,"ancestor_record_offset":25508,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":491,"record_offset":25748,"parent_record_offset":25708,"ancestor_record_offset":25508,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":491,"record_offset":26492,"parent_record_offset":26460,"ancestor_record_offset":25508,"inlinee":4678,"source_line":3552,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":491,"record_offset":26068,"parent_record_offset":26028,"ancestor_record_offset":25508,"inlinee":4632,"source_line":482,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":491,"record_offset":26096,"parent_record_offset":26068,"ancestor_record_offset":25508,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":491,"record_offset":25964,"parent_record_offset":25932,"ancestor_record_offset":25508,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$20::extend_trusted::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":491,"record_offset":26772,"parent_record_offset":26744,"ancestor_record_offset":25508,"inlinee":4708,"source_line":3565,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":491,"record_offset":26836,"parent_record_offset":26772,"ancestor_record_offset":25508,"inlinee":4710,"source_line":18,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":491,"record_offset":26460,"parent_record_offset":26028,"ancestor_record_offset":25508,"inlinee":4677,"source_line":28,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":491,"record_offset":25996,"parent_record_offset":25964,"ancestor_record_offset":25508,"inlinee":4673,"source_line":33,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":491,"record_offset":26028,"parent_record_offset":25996,"ancestor_record_offset":25508,"inlinee":4675,"source_line":52,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":491,"record_offset":26240,"parent_record_offset":26216,"ancestor_record_offset":25508,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":491,"record_offset":26264,"parent_record_offset":26240,"ancestor_record_offset":25508,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":491,"record_offset":26988,"parent_record_offset":26960,"ancestor_record_offset":25508,"inlinee":5303,"source_line":4520,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":491,"record_offset":27016,"parent_record_offset":26960,"ancestor_record_offset":25508,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":491,"record_offset":26672,"parent_record_offset":26632,"ancestor_record_offset":25508,"inlinee":4764,"source_line":203,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":491,"record_offset":26632,"parent_record_offset":26596,"ancestor_record_offset":25508,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":491,"record_offset":26596,"parent_record_offset":26556,"ancestor_record_offset":25508,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":491,"record_offset":25932,"parent_record_offset":25876,"ancestor_record_offset":25508,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":491,"record_offset":26556,"parent_record_offset":26524,"ancestor_record_offset":25508,"inlinee":5298,"source_line":2539,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":491,"record_offset":26524,"parent_record_offset":26492,"ancestor_record_offset":25508,"inlinee":4679,"source_line":790,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":491,"record_offset":26744,"parent_record_offset":26556,"ancestor_record_offset":25508,"inlinee":4706,"source_line":797,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":491,"record_offset":26288,"parent_record_offset":26264,"ancestor_record_offset":25508,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":491,"record_offset":26312,"parent_record_offset":26288,"ancestor_record_offset":25508,"inlinee":4619,"source_line":2579,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":491,"record_offset":26700,"parent_record_offset":26672,"ancestor_record_offset":25508,"inlinee":4562,"source_line":560,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":491,"record_offset":27400,"parent_record_offset":27360,"ancestor_record_offset":25508,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":491,"record_offset":27052,"parent_record_offset":25876,"ancestor_record_offset":25508,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":491,"record_offset":26960,"parent_record_offset":26932,"ancestor_record_offset":25508,"inlinee":7390,"source_line":1009,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":491,"record_offset":26804,"parent_record_offset":26772,"ancestor_record_offset":25508,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":491,"record_offset":26932,"parent_record_offset":25876,"ancestor_record_offset":25508,"inlinee":7389,"source_line":917,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] fastrand::shuffle
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fastrand","version":"1.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fastrand-1.8.0/src/lib.rs
//     evidence={"module":491,"record_offset":26904,"parent_record_offset":25876,"ancestor_record_offset":25508,"inlinee":13724,"source_line":642,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::audio::music::music_player::MusicPlayer::current_track
// 记录形式: inline_instance, inlinee_declaration; instances=8
// 源码行: 232
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x660E
// 首条位置: {"module":476,"inlinee":26126}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::slice::impl$0::get
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":476,"record_offset":35872,"parent_record_offset":35844,"ancestor_record_offset":35844,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":476,"record_offset":35896,"parent_record_offset":35872,"ancestor_record_offset":35844,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/00476.jsonl.gz"}
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
// 首条位置: {"module":491,"record_offset":27508,"rva":9138912,"va":5377848032}
// 编译实例 RVAs: 0x8b72e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140a526b0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: music_player.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1684063","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377848032,"raw_hex":"e85fb21900","size":5,"soft_fail":false,"target_va":5379532464,"va":5377848396,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t33694644","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377848032,"raw_hex":"e8b4230202","size":5,"soft_fail":false,"target_va":5411542880,"va":5377848231,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] libfmod::EventInstance::stop
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"libfmod","version":"2.222.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t12058850","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377848032,"raw_hex":"e8e200b800","size":5,"soft_fail":false,"target_va":5389907008,"va":5377848153,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":491,"record_offset":27676,"parent_record_offset":27508,"ancestor_record_offset":27508,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::audio_resources::AudioInstance::stop_immediately
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/audio_resources.rs
//     evidence={"module":491,"record_offset":27752,"parent_record_offset":27508,"ancestor_record_offset":27508,"inlinee":26124,"source_line":544,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":491,"record_offset":27868,"parent_record_offset":27832,"ancestor_record_offset":27508,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":491,"record_offset":27716,"parent_record_offset":27676,"ancestor_record_offset":27508,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":491,"record_offset":27788,"parent_record_offset":27752,"ancestor_record_offset":27508,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00491.jsonl.gz"}
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
// 首条位置: {"module":491,"record_offset":27948,"rva":9139344,"va":5377848464}
// 编译实例 RVAs: 0x8b7490
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140a526b0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: music_player.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1683669","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377848464,"raw_hex":"e8d5b01900","size":5,"soft_fail":false,"target_va":5379532464,"va":5377848790,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t33694190","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377848464,"raw_hex":"e8ee210202","size":5,"soft_fail":false,"target_va":5411542880,"va":5377848685,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] libfmod::EventInstance::set_paused
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"libfmod","version":"2.222.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t12057721","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377848464,"raw_hex":"e879fcb700","size":5,"soft_fail":false,"target_va":5389906272,"va":5377848546,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::resources::audio_resources::AudioInstance::pause
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/audio_resources.rs
//     evidence={"module":491,"record_offset":28200,"parent_record_offset":27948,"ancestor_record_offset":27948,"inlinee":27244,"source_line":562,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::audio::music::music_player::MusicPlayer::current_track
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/audio/music/music_player.rs
//     evidence={"module":491,"record_offset":28108,"parent_record_offset":27948,"ancestor_record_offset":27948,"inlinee":26126,"source_line":232,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":491,"record_offset":28232,"parent_record_offset":28200,"ancestor_record_offset":27948,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":491,"record_offset":28260,"parent_record_offset":28200,"ancestor_record_offset":27948,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":491,"record_offset":28132,"parent_record_offset":28108,"ancestor_record_offset":27948,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":491,"record_offset":28160,"parent_record_offset":28132,"ancestor_record_offset":27948,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::audio::music::music_player::MusicPlayer::probabilistic_switch_track
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 252
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6A63
// 首条位置: {"module":491,"inlinee":27235}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::f32::impl$0::powi
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":491,"record_offset":18260,"parent_record_offset":18232,"ancestor_record_offset":18232,"inlinee":14226,"source_line":314,"archive":"_pdb/B/modules/00491.jsonl.gz"}
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
// 首条位置: {"module":491,"record_offset":28412,"rva":9139744,"va":5377848864}
// 编译实例 RVAs: 0x8b7620
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::Ord::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":491,"record_offset":28644,"parent_record_offset":28412,"ancestor_record_offset":28412,"inlinee":6234,"source_line":998,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::const_ptr::impl$0::is_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":491,"record_offset":28580,"parent_record_offset":28412,"ancestor_record_offset":28412,"inlinee":27245,"source_line":39,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::is_null::runtime
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":491,"record_offset":28608,"parent_record_offset":28580,"ancestor_record_offset":28412,"inlinee":27247,"source_line":3874,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::audio::music::music_player::generate_track_playlist
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 294
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6A6B
// 首条位置: {"module":491,"inlinee":27243}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":491,"record_offset":25932,"parent_record_offset":25876,"ancestor_record_offset":25876,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":491,"record_offset":27052,"parent_record_offset":25876,"ancestor_record_offset":25876,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":491,"record_offset":26932,"parent_record_offset":25876,"ancestor_record_offset":25876,"inlinee":7389,"source_line":917,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_direct] fastrand::shuffle
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fastrand","version":"1.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fastrand-1.8.0/src/lib.rs
//     evidence={"module":491,"record_offset":26904,"parent_record_offset":25876,"ancestor_record_offset":25876,"inlinee":13724,"source_line":642,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":491,"record_offset":26380,"parent_record_offset":26356,"ancestor_record_offset":25876,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":491,"record_offset":26404,"parent_record_offset":26380,"ancestor_record_offset":25876,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":491,"record_offset":27172,"parent_record_offset":27148,"ancestor_record_offset":25876,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":491,"record_offset":26356,"parent_record_offset":26184,"ancestor_record_offset":25876,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":491,"record_offset":27148,"parent_record_offset":27124,"ancestor_record_offset":25876,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":491,"record_offset":26124,"parent_record_offset":26096,"ancestor_record_offset":25876,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":491,"record_offset":27124,"parent_record_offset":27100,"ancestor_record_offset":25876,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":491,"record_offset":26184,"parent_record_offset":26152,"ancestor_record_offset":25876,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":491,"record_offset":26152,"parent_record_offset":26124,"ancestor_record_offset":25876,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":491,"record_offset":27100,"parent_record_offset":27076,"ancestor_record_offset":25876,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":491,"record_offset":26216,"parent_record_offset":26184,"ancestor_record_offset":25876,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":491,"record_offset":26492,"parent_record_offset":26460,"ancestor_record_offset":25876,"inlinee":4678,"source_line":3552,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":491,"record_offset":26068,"parent_record_offset":26028,"ancestor_record_offset":25876,"inlinee":4632,"source_line":482,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":491,"record_offset":26096,"parent_record_offset":26068,"ancestor_record_offset":25876,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":491,"record_offset":25964,"parent_record_offset":25932,"ancestor_record_offset":25876,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$20::extend_trusted::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":491,"record_offset":26772,"parent_record_offset":26744,"ancestor_record_offset":25876,"inlinee":4708,"source_line":3565,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":491,"record_offset":26836,"parent_record_offset":26772,"ancestor_record_offset":25876,"inlinee":4710,"source_line":18,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":491,"record_offset":26460,"parent_record_offset":26028,"ancestor_record_offset":25876,"inlinee":4677,"source_line":28,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":491,"record_offset":25996,"parent_record_offset":25964,"ancestor_record_offset":25876,"inlinee":4673,"source_line":33,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":491,"record_offset":26028,"parent_record_offset":25996,"ancestor_record_offset":25876,"inlinee":4675,"source_line":52,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":491,"record_offset":26240,"parent_record_offset":26216,"ancestor_record_offset":25876,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":491,"record_offset":26264,"parent_record_offset":26240,"ancestor_record_offset":25876,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":491,"record_offset":26988,"parent_record_offset":26960,"ancestor_record_offset":25876,"inlinee":5303,"source_line":4520,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":491,"record_offset":27016,"parent_record_offset":26960,"ancestor_record_offset":25876,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":491,"record_offset":26672,"parent_record_offset":26632,"ancestor_record_offset":25876,"inlinee":4764,"source_line":203,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":491,"record_offset":26632,"parent_record_offset":26596,"ancestor_record_offset":25876,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":491,"record_offset":26596,"parent_record_offset":26556,"ancestor_record_offset":25876,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":491,"record_offset":26556,"parent_record_offset":26524,"ancestor_record_offset":25876,"inlinee":5298,"source_line":2539,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":491,"record_offset":26524,"parent_record_offset":26492,"ancestor_record_offset":25876,"inlinee":4679,"source_line":790,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":491,"record_offset":26744,"parent_record_offset":26556,"ancestor_record_offset":25876,"inlinee":4706,"source_line":797,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":491,"record_offset":26288,"parent_record_offset":26264,"ancestor_record_offset":25876,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":491,"record_offset":26312,"parent_record_offset":26288,"ancestor_record_offset":25876,"inlinee":4619,"source_line":2579,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":491,"record_offset":26700,"parent_record_offset":26672,"ancestor_record_offset":25876,"inlinee":4562,"source_line":560,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":491,"record_offset":27076,"parent_record_offset":27052,"ancestor_record_offset":25876,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":491,"record_offset":26960,"parent_record_offset":26932,"ancestor_record_offset":25876,"inlinee":7390,"source_line":1009,"archive":"_pdb/B/modules/00491.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":491,"record_offset":26804,"parent_record_offset":26772,"ancestor_record_offset":25876,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00491.jsonl.gz"}
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
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 12; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] bool (*)(ref$<country_core::resources::walls::decorator::DecoratorId>,ref$<country_core::resources::walls::decorator::DecoratorId>)
// 候选参数类型: ["ref$<country_core::resources::walls::decorator::DecoratorId>", "ref$<country_core::resources::walls::decorator::DecoratorId>"]
// [文件级接口候选，具体函数归属 unknown] bool (*)(ref$<u64>,ref$<u64>)
// 候选参数类型: ["ref$<u64>", "ref$<u64>"]
// [文件级接口候选，具体函数归属 unknown] bool (*)(ref$<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId> >,ref$<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId> >)
// 候选参数类型: ["ref$<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId> >", "ref$<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId> >"]
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
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::systems::audio::music::music_player::MusicPlayer::current_track
// 信息 ID: 9a69722d312dc53f9e2d2a3a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 6 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<country_core::systems::audio::music::music_debug_gui::music_debug_gui::closure_env$0,tuple$<ref_mut$<egui::ui::Ui> > >; module=476, inline_record=35844; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::music::music_player::MusicPlayer::update; module=491, inline_record=16288; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::music::music_player::MusicPlayer::update; module=491, inline_record=16732; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::music::music_player::MusicPlayer::update; module=491, inline_record=17680; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::music::music_player::MusicPlayer::update; module=491, inline_record=18408; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::music::music_player::MusicPlayer::pause; module=491, inline_record=28108; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::audio::music::music_player::MusicPlayer::init
// 信息 ID: cbccc2ae1d51292aa502b00e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::audio::music::music_system::music_audio_init_system @ 0xa7243b -> 0x8b63c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::audio::music::music_player::MusicPlayer::load_tracks
// 信息 ID: f119a5715d580035fae29e14
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::startup::load_glade_audio::load_glade_audio @ 0x8be5d2 -> 0x8b6b10 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::audio::music::music_player::MusicPlayer::pause
// 信息 ID: ea2d2663c703b99e11d9300c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::audio::music::music_system::music_audio_system @ 0xa72d8c -> 0x8b7490 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::audio::music::music_player::MusicPlayer::probabilistic_switch_track
// 信息 ID: 033aa7dec75ed6b6faafdda8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::audio::music::music_player::MusicPlayer::update; module=491, inline_record=18232; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::audio::music::music_player::MusicPlayer::stop_immediately
// 信息 ID: 44499e2a4d8a5ad5cf411e06
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::startup::load_glade_audio::load_glade_audio @ 0x8be39d -> 0x8b72e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::audio::music::music_player::MusicPlayer::stop_photomode_snapshots
// 信息 ID: 29c2f5cf00c4626a2fd6428e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::audio::music::music_system::music_audio_system @ 0xa72e12 -> 0x8b6250 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::music::music_system::music_audio_system @ 0xa72d98 -> 0x8b6250 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::audio::music::music_player::MusicPlayer::switch_track
// 信息 ID: c1945a19031f793cfa3212fb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::audio::music::music_player::MusicPlayer::update @ 0x8b5648 -> 0x8b70d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::audio::music::music_player::MusicPlayer::trigger_fpv_snapshot
// 信息 ID: 67639a4770e31e60af38544f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::audio::music::music_system::music_audio_system; module=553, inline_record=43000; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::audio::music::music_player::MusicPlayer::trigger_photomode_minus_fpv_snapshot
// 信息 ID: ae753fceb753667f92e842e5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::audio::music::music_system::music_audio_system; module=553, inline_record=42220; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::audio::music::music_player::MusicPlayer::update
// 信息 ID: 0837e95c5a9cf434c071105b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::audio::music::music_system::music_audio_system @ 0xa72cca -> 0x8b5500 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::audio::music::music_player::generate_track_playlist
// 信息 ID: 0f56ab58cf7001c166361071
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::audio::music::music_player::MusicPlayer::switch_track; module=491, inline_record=25876; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::audio::music::music_player::get_timeline_beat_properties
// 信息 ID: ab1e199874762a6969dad9fd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::audio::music::music_player::impl$0::update::closure$0
// 信息 ID: 7104b65ccaa8a8cf71c5e0ad
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::audio::music::music_player::impl$0::update", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::audio::music::music_player::MusicPlayer::update; module=491, inline_record=16408; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::audio::music::music_player::impl$2::fmt
// 信息 ID: 97fc8b3ce42bcdbca84d6046
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===

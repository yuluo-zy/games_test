// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/country-core/src/startup/load_audio.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: ddab2f451476896bfcd552e5a39726cdd0c35b4f030eb9e68b5856b51f2ceabd
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: load_audio.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 538
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.27"}]
// use slotmap; // namespace candidate
//   package identities: [{"name":"slotmap","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.1.1"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_audio::load_audio
// 记录形式: compiled_procedure; instances=1
// 源码行: 9
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":538,"record_offset":9928,"rva":10854208,"va":5379563328}
// 编译实例 RVAs: 0xa59f40
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::time::Instant>::elapsed
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t5085034","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379563328,"raw_hex":"e86a974d00","size":5,"soft_fail":false,"target_va":5384648448,"va":5379563409,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::time::Instant>::now
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t5084683","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379563328,"raw_hex":"e80b964d00","size":5,"soft_fail":false,"target_va":5384648032,"va":5379563344,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t5045804","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379563328,"raw_hex":"e82cfe4c00","size":5,"soft_fail":false,"target_va":5384609408,"va":5379563599,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-3884936","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379563328,"raw_hex":"e878b8c4ff","size":5,"soft_fail":false,"target_va":5375678688,"va":5379563619,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: load_audio.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10841417","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379563328,"raw_hex":"e8b7925aff","size":5,"soft_fail":false,"target_va":5368722192,"va":5379563604,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141629aa0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_audio.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12384750","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379563328,"raw_hex":"e8eef9bc00","size":5,"soft_fail":false,"target_va":5391948448,"va":5379563693,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14162a9d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_audio.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12388541","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379563328,"raw_hex":"e8bd08bd00","size":5,"soft_fail":false,"target_va":5391952336,"va":5379563790,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6fd10
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: load_audio.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t36789163","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379563328,"raw_hex":"e8ab5b3102","size":5,"soft_fail":false,"target_va":5416353040,"va":5379563872,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t37056863","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379563328,"raw_hex":"e85f713502","size":5,"soft_fail":false,"target_va":5416620794,"va":5379563926,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::audio_resources::AudioSamplesLoadedPromise::recv
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t122763","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379563328,"raw_hex":"e88bdf0100","size":5,"soft_fail":false,"target_va":5379686144,"va":5379563376,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":538,"record_offset":10088,"parent_record_offset":9928,"ancestor_record_offset":9928,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::commands::Commands::insert_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":538,"record_offset":10960,"parent_record_offset":9928,"ancestor_record_offset":9928,"inlinee":11204,"source_line":840,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::commands::Commands::remove_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":538,"record_offset":11252,"parent_record_offset":9928,"ancestor_record_offset":9928,"inlinee":11390,"source_line":861,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":538,"record_offset":10228,"parent_record_offset":9928,"ancestor_record_offset":9928,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::audio_trigger_library::impl$24::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/audio_trigger_library.rs
//     evidence={"module":538,"record_offset":10396,"parent_record_offset":9928,"ancestor_record_offset":9928,"inlinee":32971,"source_line":42,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::audio::game_loading_audio::impl$0::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/audio/game_loading_audio.rs
//     evidence={"module":538,"record_offset":11224,"parent_record_offset":9928,"ancestor_record_offset":9928,"inlinee":32975,"source_line":11,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":538,"record_offset":10284,"parent_record_offset":9928,"ancestor_record_offset":9928,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":538,"record_offset":10144,"parent_record_offset":9928,"ancestor_record_offset":9928,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":538,"record_offset":10744,"parent_record_offset":10720,"ancestor_record_offset":9928,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":538,"record_offset":10768,"parent_record_offset":10744,"ancestor_record_offset":9928,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":538,"record_offset":10792,"parent_record_offset":10768,"ancestor_record_offset":9928,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":538,"record_offset":10720,"parent_record_offset":10692,"ancestor_record_offset":9928,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":538,"record_offset":11076,"parent_record_offset":11048,"ancestor_record_offset":9928,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":538,"record_offset":10632,"parent_record_offset":10604,"ancestor_record_offset":9928,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":538,"record_offset":11160,"parent_record_offset":11136,"ancestor_record_offset":9928,"inlinee":4788,"source_line":616,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":538,"record_offset":11136,"parent_record_offset":11104,"ancestor_record_offset":9928,"inlinee":5068,"source_line":764,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":538,"record_offset":11104,"parent_record_offset":11076,"ancestor_record_offset":9928,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":538,"record_offset":10692,"parent_record_offset":10660,"ancestor_record_offset":9928,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":538,"record_offset":10660,"parent_record_offset":10632,"ancestor_record_offset":9928,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":538,"record_offset":10852,"parent_record_offset":10536,"ancestor_record_offset":9928,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":538,"record_offset":10876,"parent_record_offset":10852,"ancestor_record_offset":9928,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":538,"record_offset":11048,"parent_record_offset":11016,"ancestor_record_offset":9928,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":538,"record_offset":10576,"parent_record_offset":10536,"ancestor_record_offset":9928,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":538,"record_offset":10604,"parent_record_offset":10576,"ancestor_record_offset":9928,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":538,"record_offset":10112,"parent_record_offset":10088,"ancestor_record_offset":9928,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":538,"record_offset":10988,"parent_record_offset":10960,"ancestor_record_offset":9928,"inlinee":8582,"source_line":588,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue_internal
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":538,"record_offset":11016,"parent_record_offset":10988,"ancestor_record_offset":9928,"inlinee":8583,"source_line":652,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::array::impl$62::default
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\mod.rs
//     evidence={"module":538,"record_offset":10424,"parent_record_offset":10396,"ancestor_record_offset":9928,"inlinee":23039,"source_line":493,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":538,"record_offset":10252,"parent_record_offset":10228,"ancestor_record_offset":9928,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":538,"record_offset":10900,"parent_record_offset":10876,"ancestor_record_offset":9928,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":538,"record_offset":10168,"parent_record_offset":10144,"ancestor_record_offset":9928,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":538,"record_offset":10192,"parent_record_offset":10168,"ancestor_record_offset":9928,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::audio_trigger_library::impl$2::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/audio_trigger_library.rs
//     evidence={"module":538,"record_offset":10452,"parent_record_offset":10424,"ancestor_record_offset":9928,"inlinee":32973,"source_line":15,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":538,"record_offset":10332,"parent_record_offset":10308,"ancestor_record_offset":9928,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":538,"record_offset":10308,"parent_record_offset":10284,"ancestor_record_offset":9928,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":538,"record_offset":10356,"parent_record_offset":10332,"ancestor_record_offset":9928,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] slotmap::basic::SlotMap::with_capacity_and_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"slotmap","version":"1.1.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/slotmap-1.1.1/src/basic.rs
//     evidence={"module":538,"record_offset":10536,"parent_record_offset":10508,"ancestor_record_offset":9928,"inlinee":31183,"source_line":199,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] slotmap::basic::SlotMap::with_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"slotmap","version":"1.1.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/slotmap-1.1.1/src/basic.rs
//     evidence={"module":538,"record_offset":10508,"parent_record_offset":10480,"ancestor_record_offset":9928,"inlinee":31182,"source_line":177,"archive":"_pdb/C/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] slotmap::basic::impl$7::default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"slotmap","version":"1.1.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/slotmap-1.1.1/src/basic.rs
//     evidence={"module":538,"record_offset":10480,"parent_record_offset":10452,"ancestor_record_offset":9928,"inlinee":31181,"source_line":990,"archive":"_pdb/C/modules/00538.jsonl.gz"}
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
// 完整结构及原始类型记录: load_audio.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: load_audio.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: load_audio.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::audio_resources::AudioSamplesLoadedPromise>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::audio_resources::AudioSamplesLoadedPromise>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::audio_resources::AudioSamplesLoadedPromise>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands

// ------------------------------------------------------------------
// 函数: country_core::startup::load_audio::load_audio
// 信息 ID: 7bb8f4ae3de55771a909046a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 4db70a03d7954a41c7ffedde: void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::audio_resources::AudioSamplesLoadedPromise>,bevy_ecs::system::commands::Commands)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::audio_resources::AudioSamplesLoadedPromise>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::audio_resources::AudioSamplesLoadedPromise>,bevy_ecs::system::commands::Commands)> > @ 0x1913d2e -> 0xa59f40 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::audio_resources::AudioSamplesLoadedPromise>,bevy_ecs::system::commands::Commands),tuple$<>,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::audio_resources::AudioSamplesLoadedPromise>,bevy_ecs::system::commands::Commands)> @ 0x1a26ca1 -> 0xa59f40 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===

// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/systems/steam/startup_steam.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 2fd0e40c342a966a587bed28279909f5f02fdbf19a1e35afe8b9d662fcd55b91
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: startup_steam.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 427, 1367
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: []
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"PDB_registry_path_only","version":"0.4.21"}]
// use steamworks; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::steam::startup_steam::impl$0::check_get_action_set_handle
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 18
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x60B6
// 首条位置: {"module":427,"inlinee":24758}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::steam::startup_steam::impl$0::check_get_analog_action_handle
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 28
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x60B9
// 首条位置: {"module":427,"inlinee":24761}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::steam::startup_steam::impl$0::check_get_digital_action_handle
// 记录形式: inline_instance, inlinee_declaration; instances=8
// 源码行: 35
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x60BA
// 首条位置: {"module":427,"inlinee":24762}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::steam::startup_steam::startup_steam
// 记录形式: compiled_procedure; instances=1
// 源码行: 42
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":427,"record_offset":61628,"rva":6015344,"va":5374724464}
// 编译实例 RVAs: 0x5bc970
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140696b10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: startup_steam.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t893245","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374724464,"raw_hex":"e83da10d00","size":5,"soft_fail":false,"target_va":5375617808,"va":5374724558,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140696c50
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: startup_steam.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t893528","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374724464,"raw_hex":"e858a20d00","size":5,"soft_fail":false,"target_va":5375618128,"va":5374724595,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140696d90
//     owner=unresolved_namespace; count=7
//     target_aliases=2 candidates; complete list: startup_steam.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t893774","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374724464,"raw_hex":"e84ea30d00","size":5,"soft_fail":false,"target_va":5375618448,"va":5374724669,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ea0c50
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: startup_steam.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9322360","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374724464,"raw_hex":"e8783f8e00","size":5,"soft_fail":false,"target_va":5384047696,"va":5374725331,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144800
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: startup_steam.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t28868194","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374724464,"raw_hex":"e8627eb801","size":5,"soft_fail":false,"target_va":5403592704,"va":5374724505,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144810
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: startup_steam.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t28868179","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374724464,"raw_hex":"e8537eb801","size":5,"soft_fail":false,"target_va":5403592720,"va":5374724536,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t28194027","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374724464,"raw_hex":"e8eb34ae01","size":5,"soft_fail":false,"target_va":5402919072,"va":5374725040,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::world::command_queue::CommandQueue::get_raw
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2521359","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374724464,"raw_hex":"e80f792600","size":5,"soft_fail":false,"target_va":5377246528,"va":5374725164,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=9
//     evidence={"assembly":"\tcallq\t28253771","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374724464,"raw_hex":"e84b1eaf01","size":5,"soft_fail":false,"target_va":5402979152,"va":5374725376,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] log::__private_api::log_impl
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t2628782","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374724464,"raw_hex":"e8ae1c2800","size":5,"soft_fail":false,"target_va":5377353776,"va":5374724989,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::system::commands::Commands::insert_resource
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":427,"record_offset":62616,"parent_record_offset":61628,"ancestor_record_offset":61628,"inlinee":12145,"source_line":598,"archive":"_pdb/A/modules/00427.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":427,"record_offset":62532,"parent_record_offset":61628,"ancestor_record_offset":61628,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/00427.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":427,"record_offset":62560,"parent_record_offset":61628,"ancestor_record_offset":61628,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/00427.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":427,"record_offset":61988,"parent_record_offset":61628,"ancestor_record_offset":61628,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00427.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":427,"record_offset":63288,"parent_record_offset":61628,"ancestor_record_offset":61628,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00427.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::steam::startup_steam::impl$0::check_get_action_set_handle
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/steam/startup_steam.rs
//     evidence={"module":427,"record_offset":61956,"parent_record_offset":61628,"ancestor_record_offset":61628,"inlinee":24758,"source_line":18,"archive":"_pdb/A/modules/00427.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":427,"record_offset":62588,"parent_record_offset":61628,"ancestor_record_offset":61628,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/00427.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":427,"record_offset":62448,"parent_record_offset":61628,"ancestor_record_offset":61628,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/00427.jsonl.gz"}
//   [pdb_inline_direct] steamworks::Client::input
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/steamworks-rs-fe4b32a430eda6d8/ef87239/src/lib.rs
//     evidence={"module":427,"record_offset":61808,"parent_record_offset":61628,"ancestor_record_offset":61628,"inlinee":22160,"source_line":379,"archive":"_pdb/A/modules/00427.jsonl.gz"}
//   [pdb_inline_direct] steamworks::input::Input::init
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/steamworks-rs-fe4b32a430eda6d8/ef87239/src/input.rs
//     evidence={"module":427,"record_offset":61928,"parent_record_offset":61628,"ancestor_record_offset":61628,"inlinee":24756,"source_line":33,"archive":"_pdb/A/modules/00427.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":427,"record_offset":62800,"parent_record_offset":62772,"ancestor_record_offset":61628,"inlinee":4277,"source_line":270,"archive":"_pdb/A/modules/00427.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::needs_to_grow
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":427,"record_offset":62772,"parent_record_offset":62740,"ancestor_record_offset":61628,"inlinee":4276,"source_line":414,"archive":"_pdb/A/modules/00427.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":427,"record_offset":62740,"parent_record_offset":62712,"ancestor_record_offset":61628,"inlinee":4275,"source_line":318,"archive":"_pdb/A/modules/00427.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":427,"record_offset":63372,"parent_record_offset":63336,"ancestor_record_offset":61628,"inlinee":4296,"source_line":1790,"archive":"_pdb/A/modules/00427.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":427,"record_offset":61836,"parent_record_offset":61808,"ancestor_record_offset":61628,"inlinee":5316,"source_line":2063,"archive":"_pdb/A/modules/00427.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":427,"record_offset":63336,"parent_record_offset":63312,"ancestor_record_offset":61628,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/00427.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":427,"record_offset":62852,"parent_record_offset":62684,"ancestor_record_offset":61628,"inlinee":4292,"source_line":2202,"archive":"_pdb/A/modules/00427.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":427,"record_offset":62712,"parent_record_offset":62684,"ancestor_record_offset":61628,"inlinee":4274,"source_line":944,"archive":"_pdb/A/modules/00427.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::push
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":427,"record_offset":62644,"parent_record_offset":62616,"ancestor_record_offset":61628,"inlinee":12140,"source_line":501,"archive":"_pdb/A/modules/00427.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::command_queue::CommandQueue::push
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/command_queue.rs
//     evidence={"module":427,"record_offset":62884,"parent_record_offset":62644,"ancestor_record_offset":61628,"inlinee":12149,"source_line":78,"archive":"_pdb/A/modules/00427.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::command_queue::RawCommandQueue::push
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/command_queue.rs
//     evidence={"module":427,"record_offset":62684,"parent_record_offset":62644,"ancestor_record_offset":61628,"inlinee":12142,"source_line":157,"archive":"_pdb/A/modules/00427.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":427,"record_offset":63312,"parent_record_offset":63288,"ancestor_record_offset":61628,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00427.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":427,"record_offset":63404,"parent_record_offset":63372,"ancestor_record_offset":61628,"inlinee":4298,"source_line":398,"archive":"_pdb/A/modules/00427.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":427,"record_offset":61864,"parent_record_offset":61836,"ancestor_record_offset":61628,"inlinee":5317,"source_line":2665,"archive":"_pdb/A/modules/00427.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":427,"record_offset":62472,"parent_record_offset":62448,"ancestor_record_offset":61628,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00427.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":427,"record_offset":61888,"parent_record_offset":61864,"ancestor_record_offset":61628,"inlinee":4353,"source_line":3336,"archive":"_pdb/A/modules/00427.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":427,"record_offset":62496,"parent_record_offset":62472,"ancestor_record_offset":61628,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00427.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":427,"record_offset":63432,"parent_record_offset":63372,"ancestor_record_offset":61628,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/00427.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":427,"record_offset":63464,"parent_record_offset":63336,"ancestor_record_offset":61628,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/00427.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::steam::startup_steam::impl$0::check_get_analog_action_handle
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/steam/startup_steam.rs
//     evidence={"module":427,"record_offset":62116,"parent_record_offset":62020,"ancestor_record_offset":61628,"inlinee":24761,"source_line":28,"archive":"_pdb/A/modules/00427.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::steam::startup_steam::impl$0::check_get_digital_action_handle
//     owner=private_source_ownership_candidate; count=7
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/steam/startup_steam.rs
//     evidence={"module":427,"record_offset":62188,"parent_record_offset":62020,"ancestor_record_offset":61628,"inlinee":24762,"source_line":35,"archive":"_pdb/A/modules/00427.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::steam::startup_steam::startup_steam::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/steam/startup_steam.rs
//     evidence={"module":427,"record_offset":62020,"parent_record_offset":61988,"ancestor_record_offset":61628,"inlinee":24760,"source_line":48,"archive":"_pdb/A/modules/00427.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::steam::startup_steam::startup_steam::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 48
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x60B8
// 首条位置: {"module":427,"inlinee":24760}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::systems::steam::startup_steam::impl$0::check_get_analog_action_handle
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/steam/startup_steam.rs
//     evidence={"module":427,"record_offset":62116,"parent_record_offset":62020,"ancestor_record_offset":62020,"inlinee":24761,"source_line":28,"archive":"_pdb/A/modules/00427.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::steam::startup_steam::impl$0::check_get_digital_action_handle
//     owner=private_source_ownership_candidate; count=7
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/steam/startup_steam.rs
//     evidence={"module":427,"record_offset":62188,"parent_record_offset":62020,"ancestor_record_offset":62020,"inlinee":24762,"source_line":35,"archive":"_pdb/A/modules/00427.jsonl.gz"}
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
// 完整结构及原始类型记录: startup_steam.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: startup_steam.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: startup_steam.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 5; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::timelapse_replay::TimelapseState> >,bevy_ecs::event::EventReader<country_core::systems::timelapse_replay::TimelapseReplayFinished>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::timelapse_replay::TimelapseState> >", "bevy_ecs::event::EventReader<country_core::systems::timelapse_replay::TimelapseReplayFinished>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::timelapse_replay::TimelapseState> >
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::timelapse_replay::TimelapseReplayFinished>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::systems::wall::shapes::circle::signifiers::edit_line::SignifierEditCircleShapeLine>,ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> >,ref$<country_core::systems::ui::signifiers::SignifierAnimation> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::circle::ui_edit_circle::AnimatedCircleSignifierResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::systems::wall::shapes::circle::signifiers::edit_line::SignifierEditCircleShapeLine>,ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> >,ref$<country_core::systems::ui::signifiers::SignifierAnimation> >,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::circle::ui_edit_circle::AnimatedCircleSignifierResource>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::systems::wall::shapes::circle::signifiers::edit_line::SignifierEditCircleShapeLine>,ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> >,ref$<country_core::systems::ui::signifiers::SignifierAnimation> >,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::circle::ui_edit_circle::AnimatedCircleSignifierResource>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::event::EventReader<country_core::systems::timelapse_replay::InterruptTimelapse>,bevy_ecs::event::EventWriter<country_core::systems::timelapse_replay::TimelapseReplayFinished>,bevy_ecs::change_detection::ResMut<country_core::systems::timelapse_replay::ReplaySpeedMultiplierOverride>,bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::timelapse_replay::TimelapseState> >,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>)
// 候选参数类型: ["bevy_ecs::event::EventReader<country_core::systems::timelapse_replay::InterruptTimelapse>", "bevy_ecs::event::EventWriter<country_core::systems::timelapse_replay::TimelapseReplayFinished>", "bevy_ecs::change_detection::ResMut<country_core::systems::timelapse_replay::ReplaySpeedMultiplierOverride>", "bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::timelapse_replay::TimelapseState> >", "bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>", "bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>"]
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::timelapse_replay::InterruptTimelapse>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::timelapse_replay::TimelapseReplayFinished>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::timelapse_replay::ReplaySpeedMultiplierOverride>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::timelapse_replay::TimelapseState> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>
// [文件级接口候选，具体函数归属 unknown] bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >)
// 候选参数类型: ["enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands

// ------------------------------------------------------------------
// 函数: country_core::systems::steam::startup_steam::impl$0::check_get_action_set_handle
// 信息 ID: f13f95df78b064c7fb830535
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::steam::startup_steam::startup_steam; module=427, inline_record=61956; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::steam::startup_steam::impl$0::check_get_analog_action_handle
// 信息 ID: d82f7f94c5fdd43e31cccebe
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::steam::startup_steam::startup_steam; module=427, inline_record=62116; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::steam::startup_steam::startup_steam; module=427, inline_record=62152; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::steam::startup_steam::impl$0::check_get_digital_action_handle
// 信息 ID: 622b0bf470f5081f1ee6ac10
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 7 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::steam::startup_steam::startup_steam; module=427, inline_record=62188; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::steam::startup_steam::startup_steam; module=427, inline_record=62224; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::steam::startup_steam::startup_steam; module=427, inline_record=62260; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::steam::startup_steam::startup_steam; module=427, inline_record=62296; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::steam::startup_steam::startup_steam; module=427, inline_record=62332; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::steam::startup_steam::startup_steam; module=427, inline_record=62368; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::steam::startup_steam::startup_steam; module=427, inline_record=62404; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::steam::startup_steam::startup_steam
// 信息 ID: c1de14f06ca7c2fa9c768f09
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 19a238a0b080cc5fdb23afff: void (*)(bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource>,bevy_ecs::system::commands::Commands)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource>,bevy_ecs::system::commands::Commands)> > @ 0xe62588 -> 0x5bc970 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource>,bevy_ecs::system::commands::Commands)> @ 0xf1dd1d -> 0x5bc970 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::steam::startup_steam::startup_steam::closure$0
// 信息 ID: 265fbb9c878949231364e306
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::steam::startup_steam::startup_steam", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::steam::startup_steam::startup_steam; module=427, inline_record=62020; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===

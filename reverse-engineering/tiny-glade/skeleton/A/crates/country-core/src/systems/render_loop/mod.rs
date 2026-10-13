// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/systems/render_loop/mod.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 0114acfbe0da3686a4648204571ed98cd84be157372eca77912a7200c1e98d9c
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: mod.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 14, 256, 449, 451, 473, 485, 490, 1385
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use bevy_ecs; // namespace candidate
//   package identities: []
// use bytemuck; // namespace candidate
//   package identities: [{"name":"bytemuck","source":"PDB_registry_path_only","version":"1.16.0"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: []
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"PDB_registry_path_only","version":"0.14.5"}]
// use nonmax; // namespace candidate
//   package identities: [{"name":"nonmax","source":"PDB_registry_path_only","version":"0.5.5"}]
// use rhapsody; // namespace candidate
//   package identities: []
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::impl$1::default
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 43
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x15B5
// 首条位置: {"module":14,"inlinee":5557}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":14,"record_offset":8348,"parent_record_offset":8320,"ancestor_record_offset":8320,"inlinee":5535,"source_line":1322,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":14,"record_offset":8636,"parent_record_offset":8608,"ancestor_record_offset":8320,"inlinee":5551,"source_line":518,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":14,"record_offset":8688,"parent_record_offset":8664,"ancestor_record_offset":8320,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":14,"record_offset":8664,"parent_record_offset":8608,"ancestor_record_offset":8320,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":14,"record_offset":8712,"parent_record_offset":8688,"ancestor_record_offset":8320,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":14,"record_offset":8564,"parent_record_offset":8540,"ancestor_record_offset":8320,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":14,"record_offset":8736,"parent_record_offset":8712,"ancestor_record_offset":8320,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":14,"record_offset":8404,"parent_record_offset":8376,"ancestor_record_offset":8320,"inlinee":5539,"source_line":55,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__getit
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":14,"record_offset":8488,"parent_record_offset":8460,"ancestor_record_offset":8320,"inlinee":5544,"source_line":85,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":14,"record_offset":8608,"parent_record_offset":8460,"ancestor_record_offset":8320,"inlinee":5550,"source_line":71,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":14,"record_offset":8376,"parent_record_offset":8348,"ancestor_record_offset":8320,"inlinee":5537,"source_line":151,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":14,"record_offset":8512,"parent_record_offset":8488,"ancestor_record_offset":8320,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":14,"record_offset":8540,"parent_record_offset":8512,"ancestor_record_offset":8320,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":14,"record_offset":8460,"parent_record_offset":8432,"ancestor_record_offset":8320,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":14,"record_offset":8432,"parent_record_offset":8404,"ancestor_record_offset":8320,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::RenderContext::finish
// 记录形式: compiled_procedure; instances=1
// 源码行: 58
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":473,"record_offset":13988,"rva":6696496,"va":5375405616}
// 编译实例 RVAs: 0x662e30
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140365cd0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3133921","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375405616,"raw_hex":"e81f2ed0ff","size":5,"soft_fail":false,"target_va":5372271824,"va":5375405740,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072e930
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t834159","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375405616,"raw_hex":"e86fba0c00","size":5,"soft_fail":false,"target_va":5376239920,"va":5375405756,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t28187694","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375405616,"raw_hex":"e82e1cae01","size":5,"soft_fail":false,"target_va":5403593360,"va":5375405661,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":473,"record_offset":14276,"parent_record_offset":13988,"ancestor_record_offset":13988,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::handle::GpuOwned::mark_unused
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":473,"record_offset":14136,"parent_record_offset":13988,"ancestor_record_offset":13988,"inlinee":26891,"source_line":447,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":473,"record_offset":14184,"parent_record_offset":14160,"ancestor_record_offset":13988,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":473,"record_offset":14160,"parent_record_offset":14136,"ancestor_record_offset":13988,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::take
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\manually_drop.rs
//     evidence={"module":473,"record_offset":14324,"parent_record_offset":14296,"ancestor_record_offset":13988,"inlinee":7425,"source_line":110,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":473,"record_offset":14208,"parent_record_offset":14184,"ancestor_record_offset":13988,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":473,"record_offset":14412,"parent_record_offset":14388,"ancestor_record_offset":13988,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":473,"record_offset":14348,"parent_record_offset":14324,"ancestor_record_offset":13988,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":473,"record_offset":14232,"parent_record_offset":14208,"ancestor_record_offset":13988,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$25::drop
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":473,"record_offset":14296,"parent_record_offset":14276,"ancestor_record_offset":13988,"inlinee":7424,"source_line":550,"archive":"_pdb/A/modules/00473.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::clear_buffer<country_core::render::nani::buffers::NaniNextVisibilityBitmaskSsbo>
// 记录形式: compiled_procedure; instances=1
// 源码行: 66
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":490,"record_offset":61452,"rva":7008880,"va":5375718000}
// 编译实例 RVAs: 0x6af270
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140002d20
//     owner=unresolved_namespace; count=1
//     target_aliases=1889 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6997713","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375718000,"raw_hex":"e82f3995ff","size":5,"soft_fail":false,"target_va":5368720672,"va":5375718380,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14064bc90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-407417","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375718000,"raw_hex":"e887c8f9ff","size":5,"soft_fail":false,"target_va":5375310992,"va":5375718404,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14064bf20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-406824","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375718000,"raw_hex":"e8d8caf9ff","size":5,"soft_fail":false,"target_va":5375311648,"va":5375718467,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406dc3c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t184092","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375718000,"raw_hex":"e81ccf0200","size":5,"soft_fail":false,"target_va":5375902656,"va":5375718559,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072f520
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t524663","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375718000,"raw_hex":"e877010800","size":5,"soft_fail":false,"target_va":5376242976,"va":5375718308,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d3d7c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6874144","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375718000,"raw_hex":"e820e46800","size":5,"soft_fail":false,"target_va":5382592448,"va":5375718299,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27260379","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375718000,"raw_hex":"e8dbf59f01","size":5,"soft_fail":false,"target_va":5402978880,"va":5375718496,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$50::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":490,"record_offset":63372,"parent_record_offset":61452,"ancestor_record_offset":61452,"inlinee":7127,"source_line":326,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":490,"record_offset":61668,"parent_record_offset":61452,"ancestor_record_offset":61452,"inlinee":7123,"source_line":416,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":490,"record_offset":63628,"parent_record_offset":61452,"ancestor_record_offset":61452,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::pass_resource_access::ssbo_to_rg_handle
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/pass_resource_access.rs
//     evidence={"module":490,"record_offset":63428,"parent_record_offset":61452,"ancestor_record_offset":61452,"inlinee":22846,"source_line":206,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::render_loop::clear_rg_buffer
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/mod.rs
//     evidence={"module":490,"record_offset":63456,"parent_record_offset":61452,"ancestor_record_offset":61452,"inlinee":25798,"source_line":75,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$49::set_changed
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":490,"record_offset":63396,"parent_record_offset":63372,"ancestor_record_offset":61452,"inlinee":7129,"source_line":309,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::Components::get_resource_id
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/component.rs
//     evidence={"module":490,"record_offset":61696,"parent_record_offset":61668,"ancestor_record_offset":61452,"inlinee":5560,"source_line":751,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":490,"record_offset":63208,"parent_record_offset":63184,"ancestor_record_offset":61452,"inlinee":4387,"source_line":394,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":490,"record_offset":63184,"parent_record_offset":63160,"ancestor_record_offset":61452,"inlinee":4386,"source_line":363,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":490,"record_offset":63132,"parent_record_offset":63100,"ancestor_record_offset":61452,"inlinee":4380,"source_line":92,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_with_ticks
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":490,"record_offset":63076,"parent_record_offset":62684,"ancestor_record_offset":61452,"inlinee":4377,"source_line":112,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":490,"record_offset":63100,"parent_record_offset":63076,"ancestor_record_offset":61452,"inlinee":4378,"source_line":69,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":490,"record_offset":62716,"parent_record_offset":62684,"ancestor_record_offset":61452,"inlinee":4359,"source_line":285,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_with_ticks::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":490,"record_offset":63160,"parent_record_offset":63100,"ancestor_record_offset":61452,"inlinee":4385,"source_line":113,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":490,"record_offset":62764,"parent_record_offset":62740,"ancestor_record_offset":61452,"inlinee":4363,"source_line":56,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":490,"record_offset":62740,"parent_record_offset":62716,"ancestor_record_offset":61452,"inlinee":4361,"source_line":369,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$10::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":490,"record_offset":62856,"parent_record_offset":62792,"ancestor_record_offset":61452,"inlinee":4368,"source_line":58,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":490,"record_offset":62944,"parent_record_offset":62920,"ancestor_record_offset":61452,"inlinee":4370,"source_line":370,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::read_change_tick
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/mod.rs
//     evidence={"module":490,"record_offset":63276,"parent_record_offset":63252,"ancestor_record_offset":61452,"inlinee":4389,"source_line":2074,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::change_tick
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":490,"record_offset":63252,"parent_record_offset":62684,"ancestor_record_offset":61452,"inlinee":4388,"source_line":252,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut_by_id
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":490,"record_offset":62684,"parent_record_offset":61668,"ancestor_record_offset":61452,"inlinee":4383,"source_line":440,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$7::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\any.rs
//     evidence={"module":490,"record_offset":62416,"parent_record_offset":62392,"ancestor_record_offset":61452,"inlinee":4165,"source_line":616,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$25::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":490,"record_offset":62464,"parent_record_offset":62440,"ancestor_record_offset":61452,"inlinee":4169,"source_line":1469,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":490,"record_offset":62392,"parent_record_offset":62368,"ancestor_record_offset":61452,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":490,"record_offset":63532,"parent_record_offset":63504,"ancestor_record_offset":61452,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":490,"record_offset":61884,"parent_record_offset":61860,"ancestor_record_offset":61452,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":490,"record_offset":61968,"parent_record_offset":61944,"ancestor_record_offset":61452,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":490,"record_offset":61908,"parent_record_offset":61884,"ancestor_record_offset":61452,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":490,"record_offset":62080,"parent_record_offset":62056,"ancestor_record_offset":61452,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":490,"record_offset":62880,"parent_record_offset":62856,"ancestor_record_offset":61452,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":490,"record_offset":62652,"parent_record_offset":61696,"ancestor_record_offset":61452,"inlinee":5564,"source_line":1863,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":490,"record_offset":62920,"parent_record_offset":62740,"ancestor_record_offset":61452,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":490,"record_offset":63024,"parent_record_offset":63000,"ancestor_record_offset":61452,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":490,"record_offset":62304,"parent_record_offset":62280,"ancestor_record_offset":61452,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":490,"record_offset":62224,"parent_record_offset":62200,"ancestor_record_offset":61452,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":490,"record_offset":62792,"parent_record_offset":62764,"ancestor_record_offset":61452,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":490,"record_offset":62828,"parent_record_offset":62792,"ancestor_record_offset":61452,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":490,"record_offset":63000,"parent_record_offset":62968,"ancestor_record_offset":61452,"inlinee":4373,"source_line":232,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":490,"record_offset":63300,"parent_record_offset":63276,"ancestor_record_offset":61452,"inlinee":4390,"source_line":2413,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":490,"record_offset":63324,"parent_record_offset":63300,"ancestor_record_offset":61452,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] core::tuple::impl$11::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\tuple.rs
//     evidence={"module":490,"record_offset":62440,"parent_record_offset":62416,"ancestor_record_offset":61452,"inlinee":4167,"source_line":31,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":490,"record_offset":62368,"parent_record_offset":62344,"ancestor_record_offset":61452,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":490,"record_offset":61720,"parent_record_offset":61696,"ancestor_record_offset":61452,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":490,"record_offset":61744,"parent_record_offset":61720,"ancestor_record_offset":61452,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":490,"record_offset":62344,"parent_record_offset":62148,"ancestor_record_offset":61452,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::as_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":490,"record_offset":62256,"parent_record_offset":62172,"ancestor_record_offset":61452,"inlinee":4850,"source_line":513,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::from_base_index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":490,"record_offset":62200,"parent_record_offset":62172,"ancestor_record_offset":61452,"inlinee":4848,"source_line":355,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":490,"record_offset":62600,"parent_record_offset":61820,"ancestor_record_offset":61452,"inlinee":4857,"source_line":175,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::bucket
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":490,"record_offset":62172,"parent_record_offset":62148,"ancestor_record_offset":61452,"inlinee":4846,"source_line":995,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::find
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":490,"record_offset":61796,"parent_record_offset":61772,"ancestor_record_offset":61452,"inlinee":4826,"source_line":1454,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":490,"record_offset":61772,"parent_record_offset":61744,"ancestor_record_offset":61452,"inlinee":5563,"source_line":1477,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":490,"record_offset":61820,"parent_record_offset":61796,"ancestor_record_offset":61452,"inlinee":4828,"source_line":2166,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":490,"record_offset":62028,"parent_record_offset":62000,"ancestor_record_offset":61452,"inlinee":4839,"source_line":49,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":490,"record_offset":62056,"parent_record_offset":62028,"ancestor_record_offset":61452,"inlinee":4840,"source_line":74,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::remove_lowest_bit
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":490,"record_offset":62116,"parent_record_offset":62000,"ancestor_record_offset":61452,"inlinee":4843,"source_line":37,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::impl$3::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":490,"record_offset":62000,"parent_record_offset":61820,"ancestor_record_offset":61452,"inlinee":4837,"source_line":128,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":490,"record_offset":62148,"parent_record_offset":61820,"ancestor_record_offset":61452,"inlinee":4845,"source_line":1464,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::load
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":490,"record_offset":61860,"parent_record_offset":61820,"ancestor_record_offset":61452,"inlinee":4831,"source_line":52,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_byte
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":490,"record_offset":61944,"parent_record_offset":61820,"ancestor_record_offset":61452,"inlinee":4834,"source_line":79,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":490,"record_offset":62516,"parent_record_offset":61820,"ancestor_record_offset":61452,"inlinee":4855,"source_line":97,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":490,"record_offset":62968,"parent_record_offset":62944,"ancestor_record_offset":61452,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::pass_builder::PassBuilder::write_impl
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/pass_builder.rs
//     evidence={"module":490,"record_offset":63592,"parent_record_offset":63564,"ancestor_record_offset":61452,"inlinee":7414,"source_line":52,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::pass_builder::impl$2::write
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/pass_builder.rs
//     evidence={"module":490,"record_offset":63564,"parent_record_offset":63456,"ancestor_record_offset":61452,"inlinee":7412,"source_line":166,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::recorder::RenderGraphRecorder::add_pass
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/recorder.rs
//     evidence={"module":490,"record_offset":63504,"parent_record_offset":63456,"ancestor_record_offset":61452,"inlinee":9083,"source_line":27,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::clear_buffer<country_core::render::nani::buffers::NaniPrevVisibilityBitmaskSsbo>
// 记录形式: compiled_procedure; instances=1
// 源码行: 66
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":490,"record_offset":59244,"rva":7008304,"va":5375717424}
// 编译实例 RVAs: 0x6af030
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140002d20
//     owner=unresolved_namespace; count=1
//     target_aliases=1889 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6997137","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375717424,"raw_hex":"e86f3b95ff","size":5,"soft_fail":false,"target_va":5368720672,"va":5375717804,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14064bc90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-406841","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375717424,"raw_hex":"e8c7caf9ff","size":5,"soft_fail":false,"target_va":5375310992,"va":5375717828,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14064bf20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-406248","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375717424,"raw_hex":"e818cdf9ff","size":5,"soft_fail":false,"target_va":5375311648,"va":5375717891,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406dc3c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t184668","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375717424,"raw_hex":"e85cd10200","size":5,"soft_fail":false,"target_va":5375902656,"va":5375717983,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072f520
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t525239","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375717424,"raw_hex":"e8b7030800","size":5,"soft_fail":false,"target_va":5376242976,"va":5375717732,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d33f60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6835712","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375717424,"raw_hex":"e8004e6800","size":5,"soft_fail":false,"target_va":5382553440,"va":5375717723,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27260955","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375717424,"raw_hex":"e81bf89f01","size":5,"soft_fail":false,"target_va":5402978880,"va":5375717920,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$50::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":490,"record_offset":61164,"parent_record_offset":59244,"ancestor_record_offset":59244,"inlinee":7127,"source_line":326,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":490,"record_offset":59460,"parent_record_offset":59244,"ancestor_record_offset":59244,"inlinee":7123,"source_line":416,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":490,"record_offset":61420,"parent_record_offset":59244,"ancestor_record_offset":59244,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::pass_resource_access::ssbo_to_rg_handle
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/pass_resource_access.rs
//     evidence={"module":490,"record_offset":61220,"parent_record_offset":59244,"ancestor_record_offset":59244,"inlinee":22846,"source_line":206,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::render_loop::clear_rg_buffer
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/mod.rs
//     evidence={"module":490,"record_offset":61248,"parent_record_offset":59244,"ancestor_record_offset":59244,"inlinee":25798,"source_line":75,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$49::set_changed
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":490,"record_offset":61188,"parent_record_offset":61164,"ancestor_record_offset":59244,"inlinee":7129,"source_line":309,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::Components::get_resource_id
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/component.rs
//     evidence={"module":490,"record_offset":59488,"parent_record_offset":59460,"ancestor_record_offset":59244,"inlinee":5560,"source_line":751,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":490,"record_offset":61000,"parent_record_offset":60976,"ancestor_record_offset":59244,"inlinee":4387,"source_line":394,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":490,"record_offset":60976,"parent_record_offset":60952,"ancestor_record_offset":59244,"inlinee":4386,"source_line":363,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":490,"record_offset":60924,"parent_record_offset":60892,"ancestor_record_offset":59244,"inlinee":4380,"source_line":92,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_with_ticks
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":490,"record_offset":60868,"parent_record_offset":60476,"ancestor_record_offset":59244,"inlinee":4377,"source_line":112,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":490,"record_offset":60892,"parent_record_offset":60868,"ancestor_record_offset":59244,"inlinee":4378,"source_line":69,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":490,"record_offset":60508,"parent_record_offset":60476,"ancestor_record_offset":59244,"inlinee":4359,"source_line":285,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_with_ticks::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":490,"record_offset":60952,"parent_record_offset":60892,"ancestor_record_offset":59244,"inlinee":4385,"source_line":113,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":490,"record_offset":60556,"parent_record_offset":60532,"ancestor_record_offset":59244,"inlinee":4363,"source_line":56,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":490,"record_offset":60532,"parent_record_offset":60508,"ancestor_record_offset":59244,"inlinee":4361,"source_line":369,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$10::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":490,"record_offset":60648,"parent_record_offset":60584,"ancestor_record_offset":59244,"inlinee":4368,"source_line":58,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":490,"record_offset":60736,"parent_record_offset":60712,"ancestor_record_offset":59244,"inlinee":4370,"source_line":370,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::read_change_tick
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/mod.rs
//     evidence={"module":490,"record_offset":61068,"parent_record_offset":61044,"ancestor_record_offset":59244,"inlinee":4389,"source_line":2074,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::change_tick
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":490,"record_offset":61044,"parent_record_offset":60476,"ancestor_record_offset":59244,"inlinee":4388,"source_line":252,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut_by_id
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":490,"record_offset":60476,"parent_record_offset":59460,"ancestor_record_offset":59244,"inlinee":4383,"source_line":440,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$7::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\any.rs
//     evidence={"module":490,"record_offset":60208,"parent_record_offset":60184,"ancestor_record_offset":59244,"inlinee":4165,"source_line":616,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$25::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":490,"record_offset":60256,"parent_record_offset":60232,"ancestor_record_offset":59244,"inlinee":4169,"source_line":1469,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":490,"record_offset":60184,"parent_record_offset":60160,"ancestor_record_offset":59244,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":490,"record_offset":61324,"parent_record_offset":61296,"ancestor_record_offset":59244,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":490,"record_offset":59676,"parent_record_offset":59652,"ancestor_record_offset":59244,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":490,"record_offset":59760,"parent_record_offset":59736,"ancestor_record_offset":59244,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":490,"record_offset":59700,"parent_record_offset":59676,"ancestor_record_offset":59244,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":490,"record_offset":59872,"parent_record_offset":59848,"ancestor_record_offset":59244,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":490,"record_offset":60672,"parent_record_offset":60648,"ancestor_record_offset":59244,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":490,"record_offset":60444,"parent_record_offset":59488,"ancestor_record_offset":59244,"inlinee":5564,"source_line":1863,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":490,"record_offset":60712,"parent_record_offset":60532,"ancestor_record_offset":59244,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":490,"record_offset":60816,"parent_record_offset":60792,"ancestor_record_offset":59244,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":490,"record_offset":60096,"parent_record_offset":60072,"ancestor_record_offset":59244,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":490,"record_offset":60016,"parent_record_offset":59992,"ancestor_record_offset":59244,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":490,"record_offset":60584,"parent_record_offset":60556,"ancestor_record_offset":59244,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":490,"record_offset":60620,"parent_record_offset":60584,"ancestor_record_offset":59244,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":490,"record_offset":60792,"parent_record_offset":60760,"ancestor_record_offset":59244,"inlinee":4373,"source_line":232,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":490,"record_offset":61092,"parent_record_offset":61068,"ancestor_record_offset":59244,"inlinee":4390,"source_line":2413,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":490,"record_offset":61116,"parent_record_offset":61092,"ancestor_record_offset":59244,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] core::tuple::impl$11::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\tuple.rs
//     evidence={"module":490,"record_offset":60232,"parent_record_offset":60208,"ancestor_record_offset":59244,"inlinee":4167,"source_line":31,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":490,"record_offset":60160,"parent_record_offset":60136,"ancestor_record_offset":59244,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":490,"record_offset":59512,"parent_record_offset":59488,"ancestor_record_offset":59244,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":490,"record_offset":59536,"parent_record_offset":59512,"ancestor_record_offset":59244,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":490,"record_offset":60136,"parent_record_offset":59940,"ancestor_record_offset":59244,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::as_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":490,"record_offset":60048,"parent_record_offset":59964,"ancestor_record_offset":59244,"inlinee":4850,"source_line":513,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::from_base_index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":490,"record_offset":59992,"parent_record_offset":59964,"ancestor_record_offset":59244,"inlinee":4848,"source_line":355,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":490,"record_offset":60392,"parent_record_offset":59612,"ancestor_record_offset":59244,"inlinee":4857,"source_line":175,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::bucket
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":490,"record_offset":59964,"parent_record_offset":59940,"ancestor_record_offset":59244,"inlinee":4846,"source_line":995,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::find
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":490,"record_offset":59588,"parent_record_offset":59564,"ancestor_record_offset":59244,"inlinee":4826,"source_line":1454,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":490,"record_offset":59564,"parent_record_offset":59536,"ancestor_record_offset":59244,"inlinee":5563,"source_line":1477,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":490,"record_offset":59612,"parent_record_offset":59588,"ancestor_record_offset":59244,"inlinee":4828,"source_line":2166,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":490,"record_offset":59820,"parent_record_offset":59792,"ancestor_record_offset":59244,"inlinee":4839,"source_line":49,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":490,"record_offset":59848,"parent_record_offset":59820,"ancestor_record_offset":59244,"inlinee":4840,"source_line":74,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::remove_lowest_bit
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":490,"record_offset":59908,"parent_record_offset":59792,"ancestor_record_offset":59244,"inlinee":4843,"source_line":37,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::impl$3::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":490,"record_offset":59792,"parent_record_offset":59612,"ancestor_record_offset":59244,"inlinee":4837,"source_line":128,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":490,"record_offset":59940,"parent_record_offset":59612,"ancestor_record_offset":59244,"inlinee":4845,"source_line":1464,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::load
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":490,"record_offset":59652,"parent_record_offset":59612,"ancestor_record_offset":59244,"inlinee":4831,"source_line":52,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_byte
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":490,"record_offset":59736,"parent_record_offset":59612,"ancestor_record_offset":59244,"inlinee":4834,"source_line":79,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":490,"record_offset":60308,"parent_record_offset":59612,"ancestor_record_offset":59244,"inlinee":4855,"source_line":97,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":490,"record_offset":60760,"parent_record_offset":60736,"ancestor_record_offset":59244,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::pass_builder::PassBuilder::write_impl
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/pass_builder.rs
//     evidence={"module":490,"record_offset":61384,"parent_record_offset":61356,"ancestor_record_offset":59244,"inlinee":7414,"source_line":52,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::pass_builder::impl$2::write
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/pass_builder.rs
//     evidence={"module":490,"record_offset":61356,"parent_record_offset":61248,"ancestor_record_offset":59244,"inlinee":7412,"source_line":166,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::recorder::RenderGraphRecorder::add_pass
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/recorder.rs
//     evidence={"module":490,"record_offset":61296,"parent_record_offset":61248,"ancestor_record_offset":59244,"inlinee":9083,"source_line":27,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::clear_rg_buffer
// 记录形式: inline_instance, inlinee_declaration; instances=10
// 源码行: 75
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x64C6
// 首条位置: {"module":449,"inlinee":25798}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] rhapsody::rg::pass_builder::impl$2::write
//     owner=private_source_ownership_candidate; count=6
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/pass_builder.rs
//     evidence={"module":449,"record_offset":14704,"parent_record_offset":14596,"ancestor_record_offset":14596,"inlinee":7412,"source_line":166,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::recorder::RenderGraphRecorder::add_pass
//     owner=private_source_ownership_candidate; count=6
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/recorder.rs
//     evidence={"module":449,"record_offset":14644,"parent_record_offset":14596,"ancestor_record_offset":14596,"inlinee":9083,"source_line":27,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":449,"record_offset":14672,"parent_record_offset":14644,"ancestor_record_offset":14596,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::pass_builder::PassBuilder::write_impl
//     owner=private_source_ownership_candidate; count=6
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/pass_builder.rs
//     evidence={"module":449,"record_offset":14732,"parent_record_offset":14704,"ancestor_record_offset":14596,"inlinee":7414,"source_line":52,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::clear_rg_buffer::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 81
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6B92
// 首条位置: {"module":490,"inlinee":27538}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::rg_buffer_copy_from_slice<u32>
// 记录形式: compiled_procedure; instances=1
// 源码行: 93
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":490,"record_offset":63660,"rva":7009456,"va":5375718576}
// 编译实例 RVAs: 0x6af4b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140002d20
//     owner=unresolved_namespace; count=1
//     target_aliases=1889 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6998326","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375718576,"raw_hex":"e8ca3695ff","size":5,"soft_fail":false,"target_va":5368720672,"va":5375718993,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14064bc90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-408035","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375718576,"raw_hex":"e81dc6f9ff","size":5,"soft_fail":false,"target_va":5375310992,"va":5375719022,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14064c600
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-405796","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375718576,"raw_hex":"e8dccef9ff","size":5,"soft_fail":false,"target_va":5375313408,"va":5375719199,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406dc3c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t183217","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375718576,"raw_hex":"e8b1cb0200","size":5,"soft_fail":false,"target_va":5375902656,"va":5375719434,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072e930
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t520576","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375718576,"raw_hex":"e880f10700","size":5,"soft_fail":false,"target_va":5376239920,"va":5375719339,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072f520
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t524053","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375718576,"raw_hex":"e815ff0700","size":5,"soft_fail":false,"target_va":5376242976,"va":5375718918,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t27874258","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375718576,"raw_hex":"e8d253a901","size":5,"soft_fail":false,"target_va":5403593360,"va":5375719097,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27259643","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375718576,"raw_hex":"e8fbf29f01","size":5,"soft_fail":false,"target_va":5402978880,"va":5375719232,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27259891","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375718576,"raw_hex":"e8f3f39f01","size":5,"soft_fail":false,"target_va":5402979152,"va":5375719256,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::_$LT$impl$u20$$u5b$T$u5d$$GT$::copy_from_slice::len_mismatch_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27261188","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375718576,"raw_hex":"e804f99f01","size":5,"soft_fail":false,"target_va":5402980464,"va":5375719271,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::dynamic_constants::DynamicConstants::allocate
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t543330","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375718576,"raw_hex":"e8624a0800","size":5,"soft_fail":false,"target_va":5376262400,"va":5375719065,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$50::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":490,"record_offset":65772,"parent_record_offset":63660,"ancestor_record_offset":63660,"inlinee":7127,"source_line":326,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":490,"record_offset":63948,"parent_record_offset":63660,"ancestor_record_offset":63660,"inlinee":7123,"source_line":416,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_direct] bytemuck::cast_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bytemuck","version":"1.16.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bytemuck-1.16.0/src/lib.rs
//     evidence={"module":490,"record_offset":63836,"parent_record_offset":63660,"ancestor_record_offset":63660,"inlinee":15086,"source_line":354,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":490,"record_offset":66172,"parent_record_offset":63660,"ancestor_record_offset":63660,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::dynamic_constants::DynamicConstants::push_slice
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/dynamic_constants.rs
//     evidence={"module":490,"record_offset":65828,"parent_record_offset":63660,"ancestor_record_offset":63660,"inlinee":23008,"source_line":150,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::pass_builder::impl$2::write
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/pass_builder.rs
//     evidence={"module":490,"record_offset":65712,"parent_record_offset":63660,"ancestor_record_offset":63660,"inlinee":7412,"source_line":166,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::recorder::RenderGraphRecorder::add_pass
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/recorder.rs
//     evidence={"module":490,"record_offset":65652,"parent_record_offset":63660,"ancestor_record_offset":63660,"inlinee":9083,"source_line":27,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$49::set_changed
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":490,"record_offset":65796,"parent_record_offset":65772,"ancestor_record_offset":63660,"inlinee":7129,"source_line":309,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::Components::get_resource_id
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/component.rs
//     evidence={"module":490,"record_offset":63976,"parent_record_offset":63948,"ancestor_record_offset":63660,"inlinee":5560,"source_line":751,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":490,"record_offset":65488,"parent_record_offset":65464,"ancestor_record_offset":63660,"inlinee":4387,"source_line":394,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":490,"record_offset":65464,"parent_record_offset":65440,"ancestor_record_offset":63660,"inlinee":4386,"source_line":363,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":490,"record_offset":65412,"parent_record_offset":65380,"ancestor_record_offset":63660,"inlinee":4380,"source_line":92,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_with_ticks
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":490,"record_offset":65356,"parent_record_offset":64964,"ancestor_record_offset":63660,"inlinee":4377,"source_line":112,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":490,"record_offset":65380,"parent_record_offset":65356,"ancestor_record_offset":63660,"inlinee":4378,"source_line":69,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":490,"record_offset":64996,"parent_record_offset":64964,"ancestor_record_offset":63660,"inlinee":4359,"source_line":285,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_with_ticks::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":490,"record_offset":65440,"parent_record_offset":65380,"ancestor_record_offset":63660,"inlinee":4385,"source_line":113,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":490,"record_offset":65044,"parent_record_offset":65020,"ancestor_record_offset":63660,"inlinee":4363,"source_line":56,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":490,"record_offset":65020,"parent_record_offset":64996,"ancestor_record_offset":63660,"inlinee":4361,"source_line":369,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$10::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":490,"record_offset":65136,"parent_record_offset":65072,"ancestor_record_offset":63660,"inlinee":4368,"source_line":58,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":490,"record_offset":65224,"parent_record_offset":65200,"ancestor_record_offset":63660,"inlinee":4370,"source_line":370,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::read_change_tick
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/mod.rs
//     evidence={"module":490,"record_offset":65556,"parent_record_offset":65532,"ancestor_record_offset":63660,"inlinee":4389,"source_line":2074,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::change_tick
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":490,"record_offset":65532,"parent_record_offset":64964,"ancestor_record_offset":63660,"inlinee":4388,"source_line":252,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut_by_id
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":490,"record_offset":64964,"parent_record_offset":63948,"ancestor_record_offset":63660,"inlinee":4383,"source_line":440,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] bytemuck::internal::cast_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bytemuck","version":"1.16.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bytemuck-1.16.0/src/internal.rs
//     evidence={"module":490,"record_offset":63860,"parent_record_offset":63836,"ancestor_record_offset":63660,"inlinee":15063,"source_line":253,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] bytemuck::internal::try_cast_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bytemuck","version":"1.16.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bytemuck-1.16.0/src/internal.rs
//     evidence={"module":490,"record_offset":63884,"parent_record_offset":63860,"ancestor_record_offset":63660,"inlinee":15064,"source_line":353,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$7::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\any.rs
//     evidence={"module":490,"record_offset":64696,"parent_record_offset":64672,"ancestor_record_offset":63660,"inlinee":4165,"source_line":616,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$25::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":490,"record_offset":64744,"parent_record_offset":64720,"ancestor_record_offset":63660,"inlinee":4169,"source_line":1469,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":490,"record_offset":64672,"parent_record_offset":64648,"ancestor_record_offset":63660,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":490,"record_offset":65680,"parent_record_offset":65652,"ancestor_record_offset":63660,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":490,"record_offset":64164,"parent_record_offset":64140,"ancestor_record_offset":63660,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":490,"record_offset":64248,"parent_record_offset":64224,"ancestor_record_offset":63660,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":490,"record_offset":64188,"parent_record_offset":64164,"ancestor_record_offset":63660,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::take
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\manually_drop.rs
//     evidence={"module":490,"record_offset":66016,"parent_record_offset":65988,"ancestor_record_offset":63660,"inlinee":7425,"source_line":110,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":490,"record_offset":63908,"parent_record_offset":63884,"ancestor_record_offset":63660,"inlinee":7725,"source_line":342,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":490,"record_offset":64360,"parent_record_offset":64336,"ancestor_record_offset":63660,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":490,"record_offset":65160,"parent_record_offset":65136,"ancestor_record_offset":63660,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":490,"record_offset":64932,"parent_record_offset":63976,"ancestor_record_offset":63660,"inlinee":5564,"source_line":1863,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":490,"record_offset":65200,"parent_record_offset":65020,"ancestor_record_offset":63660,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":490,"record_offset":65304,"parent_record_offset":65280,"ancestor_record_offset":63660,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":490,"record_offset":65940,"parent_record_offset":65828,"ancestor_record_offset":63660,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":490,"record_offset":64584,"parent_record_offset":64560,"ancestor_record_offset":63660,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":490,"record_offset":64504,"parent_record_offset":64480,"ancestor_record_offset":63660,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":490,"record_offset":66040,"parent_record_offset":66016,"ancestor_record_offset":63660,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::copy_from_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":490,"record_offset":65872,"parent_record_offset":65828,"ancestor_record_offset":63660,"inlinee":5235,"source_line":3626,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":490,"record_offset":65072,"parent_record_offset":65044,"ancestor_record_offset":63660,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":490,"record_offset":65108,"parent_record_offset":65072,"ancestor_record_offset":63660,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":490,"record_offset":65280,"parent_record_offset":65248,"ancestor_record_offset":63660,"inlinee":4373,"source_line":232,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":490,"record_offset":65580,"parent_record_offset":65556,"ancestor_record_offset":63660,"inlinee":4390,"source_line":2413,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":490,"record_offset":65604,"parent_record_offset":65580,"ancestor_record_offset":63660,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] core::tuple::impl$11::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\tuple.rs
//     evidence={"module":490,"record_offset":64720,"parent_record_offset":64696,"ancestor_record_offset":63660,"inlinee":4167,"source_line":31,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":490,"record_offset":64648,"parent_record_offset":64624,"ancestor_record_offset":63660,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":490,"record_offset":64000,"parent_record_offset":63976,"ancestor_record_offset":63660,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":490,"record_offset":64024,"parent_record_offset":64000,"ancestor_record_offset":63660,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":490,"record_offset":64624,"parent_record_offset":64428,"ancestor_record_offset":63660,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::as_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":490,"record_offset":64536,"parent_record_offset":64452,"ancestor_record_offset":63660,"inlinee":4850,"source_line":513,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::from_base_index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":490,"record_offset":64480,"parent_record_offset":64452,"ancestor_record_offset":63660,"inlinee":4848,"source_line":355,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":490,"record_offset":64880,"parent_record_offset":64100,"ancestor_record_offset":63660,"inlinee":4857,"source_line":175,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::bucket
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":490,"record_offset":64452,"parent_record_offset":64428,"ancestor_record_offset":63660,"inlinee":4846,"source_line":995,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::find
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":490,"record_offset":64076,"parent_record_offset":64052,"ancestor_record_offset":63660,"inlinee":4826,"source_line":1454,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":490,"record_offset":64052,"parent_record_offset":64024,"ancestor_record_offset":63660,"inlinee":5563,"source_line":1477,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":490,"record_offset":64100,"parent_record_offset":64076,"ancestor_record_offset":63660,"inlinee":4828,"source_line":2166,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":490,"record_offset":64308,"parent_record_offset":64280,"ancestor_record_offset":63660,"inlinee":4839,"source_line":49,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":490,"record_offset":64336,"parent_record_offset":64308,"ancestor_record_offset":63660,"inlinee":4840,"source_line":74,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::remove_lowest_bit
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":490,"record_offset":64396,"parent_record_offset":64280,"ancestor_record_offset":63660,"inlinee":4843,"source_line":37,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::impl$3::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":490,"record_offset":64280,"parent_record_offset":64100,"ancestor_record_offset":63660,"inlinee":4837,"source_line":128,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":490,"record_offset":64428,"parent_record_offset":64100,"ancestor_record_offset":63660,"inlinee":4845,"source_line":1464,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::load
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":490,"record_offset":64140,"parent_record_offset":64100,"ancestor_record_offset":63660,"inlinee":4831,"source_line":52,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_byte
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":490,"record_offset":64224,"parent_record_offset":64100,"ancestor_record_offset":63660,"inlinee":4834,"source_line":79,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":490,"record_offset":64796,"parent_record_offset":64100,"ancestor_record_offset":63660,"inlinee":4855,"source_line":97,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":490,"record_offset":65248,"parent_record_offset":65224,"ancestor_record_offset":63660,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$25::drop
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":490,"record_offset":65988,"parent_record_offset":65964,"ancestor_record_offset":63660,"inlinee":7424,"source_line":550,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::pass_builder::PassBuilder::write_impl
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/pass_builder.rs
//     evidence={"module":490,"record_offset":65740,"parent_record_offset":65712,"ancestor_record_offset":63660,"inlinee":7414,"source_line":52,"archive":"_pdb/A/modules/00490.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::rg_buffer_copy_from_slice::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 110
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xFCC0
// 首条位置: {"module":1385,"inlinee":64704}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1385,"record_offset":17940,"parent_record_offset":17556,"ancestor_record_offset":17556,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::handle::GpuOwned::track_unwrap
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1385,"record_offset":17644,"parent_record_offset":17556,"ancestor_record_offset":17556,"inlinee":62065,"source_line":424,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::resource::Ref::raw
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource.rs
//     evidence={"module":1385,"record_offset":17616,"parent_record_offset":17556,"ancestor_record_offset":17556,"inlinee":26479,"source_line":355,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::take
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\manually_drop.rs
//     evidence={"module":1385,"record_offset":17788,"parent_record_offset":17760,"ancestor_record_offset":17556,"inlinee":7425,"source_line":110,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1385,"record_offset":17736,"parent_record_offset":17644,"ancestor_record_offset":17556,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1385,"record_offset":17812,"parent_record_offset":17788,"ancestor_record_offset":17556,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$17::track
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1385,"record_offset":17704,"parent_record_offset":17680,"ancestor_record_offset":17556,"inlinee":7654,"source_line":250,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$22::track
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1385,"record_offset":17680,"parent_record_offset":17644,"ancestor_record_offset":17556,"inlinee":61716,"source_line":397,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$25::drop
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1385,"record_offset":17760,"parent_record_offset":17736,"ancestor_record_offset":17556,"inlinee":7424,"source_line":550,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::clear_rg_color_image
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 129
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x64C7
// 首条位置: {"module":449,"inlinee":25799}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] rhapsody::rg::pass_builder::impl$2::write
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/pass_builder.rs
//     evidence={"module":449,"record_offset":19968,"parent_record_offset":19860,"ancestor_record_offset":19860,"inlinee":7412,"source_line":166,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::recorder::RenderGraphRecorder::add_pass
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/recorder.rs
//     evidence={"module":449,"record_offset":19908,"parent_record_offset":19860,"ancestor_record_offset":19860,"inlinee":9083,"source_line":27,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":449,"record_offset":19936,"parent_record_offset":19908,"ancestor_record_offset":19860,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::pass_builder::PassBuilder::write_impl
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/pass_builder.rs
//     evidence={"module":449,"record_offset":19996,"parent_record_offset":19968,"ancestor_record_offset":19860,"inlinee":7414,"source_line":52,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::clear_rg_color_image::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 138
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6B8E
// 首条位置: {"module":490,"inlinee":27534}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] rhapsody::rg::resource_registry::ResourceRegistry::image
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource_registry.rs
//     evidence={"module":490,"record_offset":22948,"parent_record_offset":22920,"ancestor_record_offset":22920,"inlinee":27536,"source_line":64,"archive":"_pdb/A/modules/00490.jsonl.gz"}
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
// 完整结构及原始类型记录: mod.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::render_loop::RenderContext
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 2
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<country_core::systems::render_loop::RenderContext>", "constructor": "core::ptr::drop_in_place", "arguments": ["country_core::systems::render_loop::RenderContext"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: mod.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === RENDER PASSES V1 BEGIN ===
// 渲染 Pass: crates/country-core/src/systems/render_loop/mod.rs
// 范围: PDB/API 事实、Shader 声明、映射候选、未知分别标注。
// 完整记录: mod.rs.render-pass.json.gz
// 输入/输出: 具体参数、运行时资源句柄及完整返回资源 = unknown；可确认的 API 操作及 Shader 声明见下方。
// 资源依赖: 跨 Pass 实际 producer/consumer、资源版本、队列/屏障参数 = unknown。
// 调用顺序: 只记录同一物理父函数内唯一 PDB 目标名称的 callsite 地址排列；分支、循环和 GPU 执行先后 = unknown。
// 同地址别名目标候选 0 条：全部保留在附档，但是否调用本 Pass = unknown，不列为已确认 Pass 顺序边。
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::clear_buffer<country_core::render::nani::buffers::NaniNextVisibilityBitmaskSsbo>
// [资源接口动作；资源实例 unknown] {"role":"graph_resource_write_api","symbol":"rhapsody::rg::pass_builder::impl$2::write","relationship":"pdb_inline_transitive","evidence":{"module":490,"record_offset":63564,"parent_record_offset":63456,"ancestor_record_offset":61452,"inlinee":7412,"source_line":166,"archive":"_pdb/A/modules/00490.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::clear_buffer<country_core::render::nani::buffers::NaniPrevVisibilityBitmaskSsbo>
// [资源接口动作；资源实例 unknown] {"role":"graph_resource_write_api","symbol":"rhapsody::rg::pass_builder::impl$2::write","relationship":"pdb_inline_transitive","evidence":{"module":490,"record_offset":61356,"parent_record_offset":61248,"ancestor_record_offset":59244,"inlinee":7412,"source_line":166,"archive":"_pdb/A/modules/00490.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::clear_rg_buffer
// [资源接口动作；资源实例 unknown] {"role":"graph_resource_write_api","symbol":"rhapsody::rg::pass_builder::impl$2::write","relationship":"pdb_inline_direct","evidence":{"module":449,"record_offset":14704,"parent_record_offset":14596,"ancestor_record_offset":14596,"inlinee":7412,"source_line":166,"archive":"_pdb/A/modules/00449.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::rg_buffer_copy_from_slice<u32>
// [资源接口动作；资源实例 unknown] {"role":"graph_resource_write_api","symbol":"rhapsody::rg::pass_builder::impl$2::write","relationship":"pdb_inline_direct","evidence":{"module":490,"record_offset":65712,"parent_record_offset":63660,"ancestor_record_offset":63660,"inlinee":7412,"source_line":166,"archive":"_pdb/A/modules/00490.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::clear_rg_color_image
// [资源接口动作；资源实例 unknown] {"role":"graph_resource_write_api","symbol":"rhapsody::rg::pass_builder::impl$2::write","relationship":"pdb_inline_direct","evidence":{"module":449,"record_offset":19968,"parent_record_offset":19860,"ancestor_record_offset":19860,"inlinee":7412,"source_line":166,"archive":"_pdb/A/modules/00449.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// Shader 二进制文件映射 = unknown（没有足够名称/类型关联；不按效果名称强行绑定）。
// [父函数静态调用位置] {"caller_entry_rva":3133120,"caller_aliases":{"alias_group_rva":3133120,"count":1,"display_examples":["tiny_glade::render_loop::compile_frame_render_graph"],"complete_catalog":"_pdb/A/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":3145773,"position_among_mapped_render_targets":69,"mapped_render_target_count":70,"previous_callsite":3145617,"next_callsite":3147645,"previous_target_files":["crates/country-core/src/systems/render_loop/draw_passes/final_post.rs"],"next_target_files":["crates/country-core/src/systems/render_loop/painterly/mod.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":3152112,"caller_aliases":{"alias_group_rva":3152112,"count":1,"display_examples":["tiny_glade::render_loop::render"],"complete_catalog":"_pdb/A/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":3159954,"position_among_mapped_render_targets":9,"mapped_render_target_count":9,"previous_callsite":3159297,"next_callsite":null,"previous_target_files":["crates/country-core/src/systems/render_loop/draw_passes/ui_pass.rs"],"next_target_files":[],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":6331248,"caller_aliases":{"alias_group_rva":6331248,"count":1,"display_examples":["country_core::systems::render_loop::draw_passes::transparency_pass::ice_surface_pass"],"complete_catalog":"_pdb/A/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":6332056,"position_among_mapped_render_targets":1,"mapped_render_target_count":3,"previous_callsite":null,"next_callsite":6332086,"previous_target_files":[],"next_target_files":["crates/country-core/src/systems/render_loop/mod.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":6331248,"caller_aliases":{"alias_group_rva":6331248,"count":1,"display_examples":["country_core::systems::render_loop::draw_passes::transparency_pass::ice_surface_pass"],"complete_catalog":"_pdb/A/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":6332086,"position_among_mapped_render_targets":2,"mapped_render_target_count":3,"previous_callsite":6332056,"next_callsite":6332902,"previous_target_files":["crates/country-core/src/systems/render_loop/mod.rs"],"next_target_files":["crates/country-core/src/systems/render_loop/draw_passes/common_draw_params.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":6352640,"caller_aliases":{"alias_group_rva":6352640,"count":1,"display_examples":["country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::gpu_generate_draw_lists"],"complete_catalog":"_pdb/A/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":6353860,"position_among_mapped_render_targets":1,"mapped_render_target_count":3,"previous_callsite":null,"next_callsite":6353880,"previous_target_files":[],"next_target_files":["crates/country-core/src/systems/render_loop/mod.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":6352640,"caller_aliases":{"alias_group_rva":6352640,"count":1,"display_examples":["country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::gpu_generate_draw_lists"],"complete_catalog":"_pdb/A/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":6353880,"position_among_mapped_render_targets":2,"mapped_render_target_count":3,"previous_callsite":6353860,"next_callsite":6353897,"previous_target_files":["crates/country-core/src/systems/render_loop/mod.rs"],"next_target_files":["crates/country-core/src/systems/render_loop/compute_passes/gpu_generate_draw_lists.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// 全部直接引用方/目标别名、原指令字节及位置见同名 render-pass.json.gz；没有裁定同地址唯一符号。
// Shader 方程、启用条件、实际执行和完整渲染器实现 = unknown。
// === RENDER PASSES V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: mod.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 3; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::ResMut<country_core::systems::audio::roof_audio::RoofAudioState>,bevy_ecs::event::EventReader<country_core::resources::roofs::roof_sounds::PlayRoofSound>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::change_detection::ResMut<country_core::systems::audio::roof_audio::RoofAudioState>", "bevy_ecs::event::EventReader<country_core::resources::roofs::roof_sounds::PlayRoofSound>", "bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::roof_audio::RoofAudioState>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::resources::roofs::roof_sounds::PlayRoofSound>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >
// [文件级接口候选，具体函数归属 unknown] void (*)()
// 候选参数类型: []

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::RenderContext::finish
// 信息 ID: 58f4e92b668d32cc01f5d54d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::compile_frame_render_graph @ 0x30002d -> 0x662e30 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::render_loop::render @ 0x303792 -> 0x662e30 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::clear_buffer<country_core::render::nani::buffers::NaniNextVisibilityBitmaskSsbo>
// 信息 ID: f03024c84a30d193d66b2594
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::systems::render_loop::clear_buffer -> ["country_core::render::nani::buffers::NaniNextVisibilityBitmaskSsbo"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::gpu_generate_draw_lists @ 0x60f3d8 -> 0x6af270 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::clear_buffer<country_core::render::nani::buffers::NaniPrevVisibilityBitmaskSsbo>
// 信息 ID: 4e8d38fad31e02765cacf7e2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::systems::render_loop::clear_buffer -> ["country_core::render::nani::buffers::NaniPrevVisibilityBitmaskSsbo"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::gpu_generate_draw_lists @ 0x60f3c4 -> 0x6af030 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::clear_rg_buffer
// 信息 ID: 96c85c538ccd3e8822c30823
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 6 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::transparency_pass::water_surface_pass; module=449, inline_record=14596; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::generate_wall_draw_lists; module=451, inline_record=23204; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::generate_wall_draw_lists_pass2; module=451, inline_record=33516; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::grass::grass_compute; module=485, inline_record=16344; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::clear_buffer<country_core::render::nani::buffers::NaniPrevVisibilityBitmaskSsbo>; module=490, inline_record=61248; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::clear_buffer<country_core::render::nani::buffers::NaniNextVisibilityBitmaskSsbo>; module=490, inline_record=63456; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::clear_rg_buffer::closure$0
// 信息 ID: 4cf5bb0db7f31fa3f5c95d30
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::render_loop::clear_rg_buffer", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::render_loop::clear_rg_buffer::closure_env$0 <- core::ptr::drop_in_place<country_core::systems::render_loop::clear_rg_buffer::closure_env$0>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::render_loop::clear_rg_buffer::closure_env$0 <- rhapsody::rg::pass_builder::PassBuilder::render<country_core::render::pass_builder::pass_resources::PassResources,country_core::systems::render_loop::clear_rg_buffer::closure_env$0>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::render_loop::clear_rg_buffer::closure_env$0 <- rhapsody::rg::recorder::impl$1::render<country_core::systems::render_loop::clear_rg_buffer::closure_env$0,country_core::render::pass_builder::pass_resources::PassResources>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] rhapsody::rg::recorder::impl$1::render<country_core::systems::render_loop::clear_rg_buffer::closure_env$0,country_core::render::pass_builder::pass_resources::PassResources>; module=490, inline_record=23460; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::clear_rg_color_image
// 信息 ID: 41eb1ab34f3111a2dfc9c362
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::transparency_pass::ice_surface_pass; module=449, inline_record=19860; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::clear_rg_color_image::closure$0
// 信息 ID: 8c31d7cde4088fd9e22b14c3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::render_loop::clear_rg_color_image", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::render_loop::clear_rg_color_image::closure_env$0 <- rhapsody::rg::pass_builder::PassBuilder::render<country_core::render::pass_builder::pass_resources::PassResources,country_core::systems::render_loop::clear_rg_color_image::closure_env$0>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::render_loop::clear_rg_color_image::closure_env$0 <- rhapsody::rg::recorder::impl$1::render<country_core::systems::render_loop::clear_rg_color_image::closure_env$0,country_core::render::pass_builder::pass_resources::PassResources>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] rhapsody::rg::recorder::impl$1::render<country_core::systems::render_loop::clear_rg_color_image::closure_env$0,country_core::render::pass_builder::pass_resources::PassResources>; module=490, inline_record=22920; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::impl$1::default
// 信息 ID: 037df99492a65fd236478a75
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] std::sys_common::backtrace::__rust_begin_short_backtrace<tiny_glade::run_game::run_game_thread::closure_env$7,tuple$<> >; module=14, inline_record=8320; 不是独立调用边。
// [PDB 内联宿主] tiny_glade::run_game::run_game_thread; module=256, inline_record=115744; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::rg_buffer_copy_from_slice::closure$0
// 信息 ID: 54aaf39ac0d4e0603150293b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::render_loop::rg_buffer_copy_from_slice", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::render_loop::rg_buffer_copy_from_slice::closure_env$0 <- core::ptr::drop_in_place<country_core::systems::render_loop::rg_buffer_copy_from_slice::closure_env$0<u32> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::render_loop::rg_buffer_copy_from_slice::closure_env$0 <- rhapsody::rg::pass_builder::PassBuilder::render<country_core::render::pass_builder::pass_resources::PassResources,country_core::systems::render_loop::rg_buffer_copy_from_slice::closure_env$0<u32> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::render_loop::rg_buffer_copy_from_slice::closure_env$0 <- rhapsody::rg::recorder::impl$1::render<country_core::systems::render_loop::rg_buffer_copy_from_slice::closure_env$0<u32>,country_core::render::pass_builder::pass_resources::PassResources>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::render_loop::rg_buffer_copy_from_slice::closure_env$0 <- alloc::boxed::impl$47::call_once<tuple$<ref_mut$<rhapsody::rg::pass_api::RenderPassApi>,ref_mut$<country_core::render::pass_builder::pass_resources::PassResources> >,country_core::systems::render_loop::rg_buffer_copy_from_slice::closure_env$0<u32>,alloc::alloc::Global>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 4 条。
// [PDB 内联宿主] alloc::boxed::impl$47::call_once<tuple$<ref_mut$<rhapsody::rg::pass_api::RenderPassApi>,ref_mut$<country_core::render::pass_builder::pass_resources::PassResources> >,country_core::systems::render_loop::rg_buffer_copy_from_slice::closure_env$0<u32>,alloc::alloc::Global>; module=1385, inline_record=17556; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::rg_buffer_copy_from_slice<u32>
// 信息 ID: f53e28eb653e641296c8ab7c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::systems::render_loop::rg_buffer_copy_from_slice -> ["u32"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::render_loop::draw_passes::transparency_pass::ice_surface_pass @ 0x609e98 -> 0x6af4b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::transparency_pass::ice_surface_pass @ 0x609eb6 -> 0x6af4b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===

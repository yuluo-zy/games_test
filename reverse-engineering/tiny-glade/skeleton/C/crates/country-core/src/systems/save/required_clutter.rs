// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/country-core/src/systems/save/required_clutter.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: e2a078d11636ca6fc824216c109a56c27811bd9eacd869abb39de7bc00cba5ac
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: required_clutter.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 509, 529, 1643, 1673, 1682, 1685
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
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.1"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.1"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.27"}]
// use std; // namespace candidate
//   package identities: []
// use tg_mods; // namespace candidate
//   package identities: [{"name":"tg-mods","source":"local_or_path","version":"0.1.0"}]
// use ustr; // namespace candidate
//   package identities: [{"name":"ustr","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::required_clutter::warn_missing_mods_on_load
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":509,"record_offset":28720,"rva":9609008,"va":5378318128}
// 编译实例 RVAs: 0x929f30
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <alloc::string::String as core::clone::Clone>::clone
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t6476734","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378318128,"raw_hex":"e8bed36200","size":5,"soft_fail":false,"target_va":5384795984,"va":5378319245,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t6290815","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378318128,"raw_hex":"e87ffd5f00","size":5,"soft_fail":false,"target_va":5384609408,"va":5378318588,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=6
//     evidence={"assembly":"\tcallq\t-2640166","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378318128,"raw_hex":"e8dab6d7ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5378318865,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400b83b0
//     owner=unresolved_namespace; count=2
//     target_aliases=4 candidates; complete list: required_clutter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8855098","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378318128,"raw_hex":"e8c6e178ff","size":5,"soft_fail":false,"target_va":5369463728,"va":5378318821,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409238a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: required_clutter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-26553","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378318128,"raw_hex":"e84798ffff","size":5,"soft_fail":false,"target_va":5378291872,"va":5378318420,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409239d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: required_clutter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-26154","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378318128,"raw_hex":"e8d699ffff","size":5,"soft_fail":false,"target_va":5378292176,"va":5378318325,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140977010
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: required_clutter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t313739","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378318128,"raw_hex":"e88bc90400","size":5,"soft_fail":false,"target_va":5378633744,"va":5378320000,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14099d0f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: required_clutter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t469547","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378318128,"raw_hex":"e82b2a0700","size":5,"soft_fail":false,"target_va":5378789616,"va":5378320064,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b07700
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: required_clutter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1954277","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378318128,"raw_hex":"e8e5d11d00","size":5,"soft_fail":false,"target_va":5380273920,"va":5378319638,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b08220
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: required_clutter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1958069","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378318128,"raw_hex":"e8b5e01d00","size":5,"soft_fail":false,"target_va":5380276768,"va":5378318694,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c91ac0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: required_clutter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3569336","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378318128,"raw_hex":"e8b8763600","size":5,"soft_fail":false,"target_va":5381888704,"va":5378319363,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f550
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: required_clutter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t38032042","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378318128,"raw_hex":"e8aa524402","size":5,"soft_fail":false,"target_va":5416351056,"va":5378319009,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb40
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: required_clutter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t38738166","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378318128,"raw_hex":"e8f6184f02","size":5,"soft_fail":false,"target_va":5417057088,"va":5378318917,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::steam::missing_mods::collect_missing_mods
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t930946","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378318128,"raw_hex":"e882340e00","size":5,"soft_fail":false,"target_va":5379249680,"va":5378318729,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::steam::workshop_subscriptions::lookup_mod_on_workshop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2152816","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378318128,"raw_hex":"e870d92000","size":5,"soft_fail":false,"target_va":5380472096,"va":5378319275,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::collections::btree::set::BTreeSet::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\set.rs
//     evidence={"module":509,"record_offset":29620,"parent_record_offset":28720,"ancestor_record_offset":28720,"inlinee":29155,"source_line":1290,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_direct] alloc::collections::btree::set::BTreeSet::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\set.rs
//     evidence={"module":509,"record_offset":29648,"parent_record_offset":28720,"ancestor_record_offset":28720,"inlinee":29111,"source_line":1243,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::impl$32::clone
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":509,"record_offset":31864,"parent_record_offset":28720,"ancestor_record_offset":28720,"inlinee":5906,"source_line":2376,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":509,"record_offset":30268,"parent_record_offset":28720,"ancestor_record_offset":28720,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::message_writer::MessageWriter::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_writer.rs
//     evidence={"module":509,"record_offset":31500,"parent_record_offset":28720,"ancestor_record_offset":28720,"inlinee":11206,"source_line":69,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":509,"record_offset":29900,"parent_record_offset":28720,"ancestor_record_offset":28720,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":509,"record_offset":29788,"parent_record_offset":28720,"ancestor_record_offset":28720,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":509,"record_offset":29732,"parent_record_offset":28720,"ancestor_record_offset":28720,"inlinee":7311,"source_line":789,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":509,"record_offset":30068,"parent_record_offset":28720,"ancestor_record_offset":28720,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$179::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":509,"record_offset":32008,"parent_record_offset":28720,"ancestor_record_offset":28720,"inlinee":5064,"source_line":157,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::save::required_clutter::collect_missing_external_ids
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/save/required_clutter.rs
//     evidence={"module":509,"record_offset":28936,"parent_record_offset":28720,"ancestor_record_offset":28720,"inlinee":29153,"source_line":44,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::steam::missing_mods::MissingMods::has_issues
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/steam/missing_mods.rs
//     evidence={"module":509,"record_offset":31472,"parent_record_offset":28720,"ancestor_record_offset":28720,"inlinee":29157,"source_line":121,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":509,"record_offset":29956,"parent_record_offset":28720,"ancestor_record_offset":28720,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":509,"record_offset":29816,"parent_record_offset":28720,"ancestor_record_offset":28720,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::HashSet::clear
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":509,"record_offset":31000,"parent_record_offset":28720,"ancestor_record_offset":28720,"inlinee":19431,"source_line":500,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_direct] tg_mods::identity::impl$5::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-mods/src/identity.rs
//     evidence={"module":509,"record_offset":32792,"parent_record_offset":28720,"ancestor_record_offset":28720,"inlinee":29159,"source_line":47,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":509,"record_offset":30580,"parent_record_offset":30556,"ancestor_record_offset":28720,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":509,"record_offset":30604,"parent_record_offset":30580,"ancestor_record_offset":28720,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":509,"record_offset":30628,"parent_record_offset":30604,"ancestor_record_offset":28720,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":509,"record_offset":30556,"parent_record_offset":30468,"ancestor_record_offset":28720,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::BTreeMap::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\map.rs
//     evidence={"module":509,"record_offset":29696,"parent_record_offset":29672,"ancestor_record_offset":28720,"inlinee":8809,"source_line":2698,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::BTreeMap::keys
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\map.rs
//     evidence={"module":509,"record_offset":29672,"parent_record_offset":29648,"ancestor_record_offset":28720,"inlinee":29112,"source_line":2754,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\map.rs
//     evidence={"module":509,"record_offset":30116,"parent_record_offset":30092,"ancestor_record_offset":28720,"inlinee":4643,"source_line":207,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$36::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\map.rs
//     evidence={"module":509,"record_offset":30220,"parent_record_offset":30196,"ancestor_record_offset":28720,"inlinee":4868,"source_line":1904,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":509,"record_offset":31728,"parent_record_offset":31696,"ancestor_record_offset":28720,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":509,"record_offset":30496,"parent_record_offset":30468,"ancestor_record_offset":28720,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":509,"record_offset":30468,"parent_record_offset":30444,"ancestor_record_offset":28720,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":509,"record_offset":31776,"parent_record_offset":31752,"ancestor_record_offset":28720,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":509,"record_offset":31752,"parent_record_offset":31728,"ancestor_record_offset":28720,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":509,"record_offset":30444,"parent_record_offset":30420,"ancestor_record_offset":28720,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":509,"record_offset":32968,"parent_record_offset":32932,"ancestor_record_offset":28720,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":509,"record_offset":32932,"parent_record_offset":32908,"ancestor_record_offset":28720,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":509,"record_offset":31696,"parent_record_offset":31656,"ancestor_record_offset":28720,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":509,"record_offset":31624,"parent_record_offset":31588,"ancestor_record_offset":28720,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":509,"record_offset":31656,"parent_record_offset":31624,"ancestor_record_offset":28720,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":509,"record_offset":30356,"parent_record_offset":30328,"ancestor_record_offset":28720,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":509,"record_offset":30296,"parent_record_offset":30268,"ancestor_record_offset":28720,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":509,"record_offset":31524,"parent_record_offset":31500,"ancestor_record_offset":28720,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::messages::Messages::write_with_caller
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs
//     evidence={"module":509,"record_offset":31588,"parent_record_offset":31524,"ancestor_record_offset":28720,"inlinee":8681,"source_line":129,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":509,"record_offset":29268,"parent_record_offset":29240,"ancestor_record_offset":28720,"inlinee":4655,"source_line":1319,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":509,"record_offset":29320,"parent_record_offset":29296,"ancestor_record_offset":28720,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter::Filter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\filter.rs
//     evidence={"module":509,"record_offset":29556,"parent_record_offset":29532,"ancestor_record_offset":28720,"inlinee":8666,"source_line":27,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":509,"record_offset":29756,"parent_record_offset":29732,"ancestor_record_offset":28720,"inlinee":7313,"source_line":68,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":509,"record_offset":29588,"parent_record_offset":28936,"ancestor_record_offset":28720,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::filter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":509,"record_offset":29532,"parent_record_offset":28936,"ancestor_record_offset":28720,"inlinee":8664,"source_line":911,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":509,"record_offset":30524,"parent_record_offset":30496,"ancestor_record_offset":28720,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":509,"record_offset":29428,"parent_record_offset":29404,"ancestor_record_offset":28720,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":509,"record_offset":29924,"parent_record_offset":29900,"ancestor_record_offset":28720,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":509,"record_offset":29404,"parent_record_offset":28992,"ancestor_record_offset":28720,"inlinee":11848,"source_line":1438,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":509,"record_offset":29212,"parent_record_offset":29184,"ancestor_record_offset":28720,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":509,"record_offset":30092,"parent_record_offset":30068,"ancestor_record_offset":28720,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":509,"record_offset":31344,"parent_record_offset":31320,"ancestor_record_offset":28720,"inlinee":5028,"source_line":1432,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":509,"record_offset":33000,"parent_record_offset":32968,"ancestor_record_offset":28720,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":509,"record_offset":32036,"parent_record_offset":32008,"ancestor_record_offset":28720,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":509,"record_offset":30140,"parent_record_offset":30116,"ancestor_record_offset":28720,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":509,"record_offset":31816,"parent_record_offset":31656,"ancestor_record_offset":28720,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":509,"record_offset":31368,"parent_record_offset":31344,"ancestor_record_offset":28720,"inlinee":5029,"source_line":697,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":509,"record_offset":31892,"parent_record_offset":31864,"ancestor_record_offset":28720,"inlinee":5322,"source_line":3134,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":509,"record_offset":29840,"parent_record_offset":29816,"ancestor_record_offset":28720,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":509,"record_offset":31916,"parent_record_offset":31892,"ancestor_record_offset":28720,"inlinee":4153,"source_line":3919,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":509,"record_offset":29864,"parent_record_offset":29840,"ancestor_record_offset":28720,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":509,"record_offset":33028,"parent_record_offset":32968,"ancestor_record_offset":28720,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":509,"record_offset":33060,"parent_record_offset":32932,"ancestor_record_offset":28720,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_storage::DecoratorStorage::iter
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_storage.rs
//     evidence={"module":509,"record_offset":28992,"parent_record_offset":28968,"ancestor_record_offset":28720,"inlinee":29137,"source_line":742,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::save::required_clutter::external_clutter_in_storage
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/save/required_clutter.rs
//     evidence={"module":509,"record_offset":28968,"parent_record_offset":28936,"ancestor_record_offset":28720,"inlinee":29154,"source_line":61,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":509,"record_offset":29240,"parent_record_offset":29184,"ancestor_record_offset":28720,"inlinee":4654,"source_line":59,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty_or_deleted
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":509,"record_offset":29296,"parent_record_offset":29240,"ancestor_record_offset":28720,"inlinee":7442,"source_line":101,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::impl$2::fill_tag
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":509,"record_offset":31320,"parent_record_offset":31260,"ancestor_record_offset":28720,"inlinee":5027,"source_line":79,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::clear
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":509,"record_offset":31048,"parent_record_offset":31024,"ancestor_record_offset":28720,"inlinee":19433,"source_line":1009,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":509,"record_offset":29052,"parent_record_offset":29028,"ancestor_record_offset":28720,"inlinee":8380,"source_line":754,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":509,"record_offset":29184,"parent_record_offset":29100,"ancestor_record_offset":28720,"inlinee":4653,"source_line":3584,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::clear
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":509,"record_offset":31072,"parent_record_offset":31048,"ancestor_record_offset":28720,"inlinee":12661,"source_line":872,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::clear_no_drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":509,"record_offset":31172,"parent_record_offset":31148,"ancestor_record_offset":28720,"inlinee":12651,"source_line":866,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":509,"record_offset":29076,"parent_record_offset":29052,"ancestor_record_offset":28720,"inlinee":8381,"source_line":1395,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::buckets
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":509,"record_offset":29156,"parent_record_offset":29100,"ancestor_record_offset":28720,"inlinee":8402,"source_line":2675,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::clear_no_drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":509,"record_offset":31196,"parent_record_offset":31172,"ancestor_record_offset":28720,"inlinee":5023,"source_line":3207,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::ctrl_slice
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":509,"record_offset":31260,"parent_record_offset":31196,"ancestor_record_offset":28720,"inlinee":5024,"source_line":2669,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":509,"record_offset":29128,"parent_record_offset":29100,"ancestor_record_offset":28720,"inlinee":4651,"source_line":2478,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_empty_singleton
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":509,"record_offset":31232,"parent_record_offset":31196,"ancestor_record_offset":28720,"inlinee":4364,"source_line":2696,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":509,"record_offset":29100,"parent_record_offset":29076,"ancestor_record_offset":28720,"inlinee":4650,"source_line":2178,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::num_ctrl_bytes
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":509,"record_offset":31292,"parent_record_offset":31260,"ancestor_record_offset":28720,"inlinee":5025,"source_line":2691,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bucket_mask_to_capacity
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":509,"record_offset":31408,"parent_record_offset":31196,"ancestor_record_offset":28720,"inlinee":5031,"source_line":184,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::clear::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":509,"record_offset":31148,"parent_record_offset":31124,"ancestor_record_offset":28720,"inlinee":12650,"source_line":878,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::scopeguard::impl$3::drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\scopeguard.rs
//     evidence={"module":509,"record_offset":31124,"parent_record_offset":31100,"ancestor_record_offset":28720,"inlinee":12554,"source_line":69,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::clear
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\set.rs
//     evidence={"module":509,"record_offset":31024,"parent_record_offset":31000,"ancestor_record_offset":28720,"inlinee":19432,"source_line":430,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":509,"record_offset":30004,"parent_record_offset":29980,"ancestor_record_offset":28720,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":509,"record_offset":29980,"parent_record_offset":29956,"ancestor_record_offset":28720,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":509,"record_offset":30028,"parent_record_offset":30004,"ancestor_record_offset":28720,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":509,"record_offset":29028,"parent_record_offset":28992,"ancestor_record_offset":28720,"inlinee":8401,"source_line":638,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::required_clutter::collect_missing_external_ids
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 44
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x71E1
// 首条位置: {"module":509,"inlinee":29153}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":509,"record_offset":29588,"parent_record_offset":28936,"ancestor_record_offset":28936,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::filter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":509,"record_offset":29532,"parent_record_offset":28936,"ancestor_record_offset":28936,"inlinee":8664,"source_line":911,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::save::required_clutter::external_clutter_in_storage
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/save/required_clutter.rs
//     evidence={"module":509,"record_offset":28968,"parent_record_offset":28936,"ancestor_record_offset":28936,"inlinee":29154,"source_line":61,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":509,"record_offset":29268,"parent_record_offset":29240,"ancestor_record_offset":28936,"inlinee":4655,"source_line":1319,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":509,"record_offset":29320,"parent_record_offset":29296,"ancestor_record_offset":28936,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter::Filter::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\filter.rs
//     evidence={"module":509,"record_offset":29556,"parent_record_offset":29532,"ancestor_record_offset":28936,"inlinee":8666,"source_line":27,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":509,"record_offset":29428,"parent_record_offset":29404,"ancestor_record_offset":28936,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::iter
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":509,"record_offset":29404,"parent_record_offset":28992,"ancestor_record_offset":28936,"inlinee":11848,"source_line":1438,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":509,"record_offset":29212,"parent_record_offset":29184,"ancestor_record_offset":28936,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_storage::DecoratorStorage::iter
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_storage.rs
//     evidence={"module":509,"record_offset":28992,"parent_record_offset":28968,"ancestor_record_offset":28936,"inlinee":29137,"source_line":742,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":509,"record_offset":29240,"parent_record_offset":29184,"ancestor_record_offset":28936,"inlinee":4654,"source_line":59,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty_or_deleted
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":509,"record_offset":29296,"parent_record_offset":29240,"ancestor_record_offset":28936,"inlinee":7442,"source_line":101,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":509,"record_offset":29052,"parent_record_offset":29028,"ancestor_record_offset":28936,"inlinee":8380,"source_line":754,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":509,"record_offset":29184,"parent_record_offset":29100,"ancestor_record_offset":28936,"inlinee":4653,"source_line":3584,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":509,"record_offset":29076,"parent_record_offset":29052,"ancestor_record_offset":28936,"inlinee":8381,"source_line":1395,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::buckets
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":509,"record_offset":29156,"parent_record_offset":29100,"ancestor_record_offset":28936,"inlinee":8402,"source_line":2675,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":509,"record_offset":29128,"parent_record_offset":29100,"ancestor_record_offset":28936,"inlinee":4651,"source_line":2478,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":509,"record_offset":29100,"parent_record_offset":29076,"ancestor_record_offset":28936,"inlinee":4650,"source_line":2178,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":509,"record_offset":29028,"parent_record_offset":28992,"ancestor_record_offset":28936,"inlinee":8401,"source_line":638,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::required_clutter::collect_missing_external_ids::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=9
// 源码行: 49
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13D16
// 首条位置: {"module":1643,"inlinee":81174}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_none
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1643,"record_offset":77944,"parent_record_offset":76900,"ancestor_record_offset":76900,"inlinee":4877,"source_line":684,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::clutter_registry::ClutterRegistry::get
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/clutter_registry.rs
//     evidence={"module":1643,"record_offset":76924,"parent_record_offset":76900,"ancestor_record_offset":76900,"inlinee":32530,"source_line":395,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1643,"record_offset":77644,"parent_record_offset":77620,"ancestor_record_offset":76900,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1643,"record_offset":77212,"parent_record_offset":77188,"ancestor_record_offset":76900,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1643,"record_offset":77296,"parent_record_offset":77272,"ancestor_record_offset":76900,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1643,"record_offset":77768,"parent_record_offset":77076,"ancestor_record_offset":76900,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":1643,"record_offset":77444,"parent_record_offset":77416,"ancestor_record_offset":76900,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":1643,"record_offset":77416,"parent_record_offset":77388,"ancestor_record_offset":76900,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1643,"record_offset":77968,"parent_record_offset":77944,"ancestor_record_offset":76900,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1643,"record_offset":77236,"parent_record_offset":77212,"ancestor_record_offset":76900,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1643,"record_offset":77560,"parent_record_offset":77536,"ancestor_record_offset":76900,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1643,"record_offset":77716,"parent_record_offset":77692,"ancestor_record_offset":76900,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::clutter_registry::impl$13::eq
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/clutter_registry.rs
//     evidence={"module":1643,"record_offset":77668,"parent_record_offset":77644,"ancestor_record_offset":76900,"inlinee":7241,"source_line":18,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1643,"record_offset":77356,"parent_record_offset":77328,"ancestor_record_offset":76900,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1643,"record_offset":77388,"parent_record_offset":77356,"ancestor_record_offset":76900,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1643,"record_offset":77328,"parent_record_offset":77076,"ancestor_record_offset":76900,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1643,"record_offset":77188,"parent_record_offset":77076,"ancestor_record_offset":76900,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1643,"record_offset":77796,"parent_record_offset":77076,"ancestor_record_offset":76900,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=10
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1643,"record_offset":77272,"parent_record_offset":77076,"ancestor_record_offset":76900,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":1643,"record_offset":77132,"parent_record_offset":77076,"ancestor_record_offset":76900,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":1643,"record_offset":77620,"parent_record_offset":77596,"ancestor_record_offset":76900,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1643,"record_offset":76972,"parent_record_offset":76948,"ancestor_record_offset":76900,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1643,"record_offset":77596,"parent_record_offset":77488,"ancestor_record_offset":76900,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1643,"record_offset":77000,"parent_record_offset":76972,"ancestor_record_offset":76900,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1643,"record_offset":77536,"parent_record_offset":77512,"ancestor_record_offset":76900,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1643,"record_offset":77888,"parent_record_offset":77076,"ancestor_record_offset":76900,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1643,"record_offset":77512,"parent_record_offset":77488,"ancestor_record_offset":76900,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1643,"record_offset":77052,"parent_record_offset":77028,"ancestor_record_offset":76900,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1643,"record_offset":77028,"parent_record_offset":76972,"ancestor_record_offset":76900,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1643,"record_offset":77076,"parent_record_offset":77052,"ancestor_record_offset":76900,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1643,"record_offset":77160,"parent_record_offset":77076,"ancestor_record_offset":76900,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1643,"record_offset":77488,"parent_record_offset":77076,"ancestor_record_offset":76900,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":1643,"record_offset":76948,"parent_record_offset":76924,"ancestor_record_offset":76900,"inlinee":7239,"source_line":997,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] ustr::impl$22::eq
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"ustr","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ustr-1.0.0/src/lib.rs
//     evidence={"module":1643,"record_offset":77692,"parent_record_offset":77668,"ancestor_record_offset":76900,"inlinee":7243,"source_line":166,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::required_clutter::glade_uses_external_clutter
// 记录形式: compiled_procedure; instances=1
// 源码行: 55
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":509,"record_offset":34348,"rva":9610976,"va":5378320096}
// 编译实例 RVAs: 0x92a6e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::iter::adapters::flatten::impl$3::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\flatten.rs
//     evidence={"module":509,"record_offset":34972,"parent_record_offset":34348,"ancestor_record_offset":34348,"inlinee":10940,"source_line":63,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::save::required_clutter::external_clutter_in_storage
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/save/required_clutter.rs
//     evidence={"module":509,"record_offset":34512,"parent_record_offset":34348,"ancestor_record_offset":34348,"inlinee":29154,"source_line":61,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":509,"record_offset":37396,"parent_record_offset":37372,"ancestor_record_offset":34348,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":509,"record_offset":34764,"parent_record_offset":34740,"ancestor_record_offset":34348,"inlinee":4655,"source_line":1319,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::and_then_or_clear
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":509,"record_offset":35168,"parent_record_offset":35136,"ancestor_record_offset":34348,"inlinee":7044,"source_line":331,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":509,"record_offset":35136,"parent_record_offset":35104,"ancestor_record_offset":34348,"inlinee":7042,"source_line":81,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":509,"record_offset":35956,"parent_record_offset":35932,"ancestor_record_offset":34348,"inlinee":7430,"source_line":82,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter_map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\filter_map.rs
//     evidence={"module":509,"record_offset":36668,"parent_record_offset":36644,"ancestor_record_offset":34348,"inlinee":6653,"source_line":63,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::flatten::and_then_or_clear
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\flatten.rs
//     evidence={"module":509,"record_offset":36588,"parent_record_offset":34996,"ancestor_record_offset":34348,"inlinee":10749,"source_line":752,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::flatten::impl$20::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\flatten.rs
//     evidence={"module":509,"record_offset":34996,"parent_record_offset":34972,"ancestor_record_offset":34348,"inlinee":10747,"source_line":518,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::fuse::impl$11::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\fuse.rs
//     evidence={"module":509,"record_offset":35072,"parent_record_offset":35040,"ancestor_record_offset":34348,"inlinee":10753,"source_line":380,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::fuse::impl$3::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\fuse.rs
//     evidence={"module":509,"record_offset":35040,"parent_record_offset":34996,"ancestor_record_offset":34348,"inlinee":10751,"source_line":49,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":509,"record_offset":35104,"parent_record_offset":35072,"ancestor_record_offset":34348,"inlinee":5118,"source_line":106,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find_map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":509,"record_offset":36692,"parent_record_offset":36668,"ancestor_record_offset":34348,"inlinee":6654,"source_line":2969,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find_map::check::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":509,"record_offset":37224,"parent_record_offset":36716,"ancestor_record_offset":34348,"inlinee":9022,"source_line":2976,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":509,"record_offset":36716,"parent_record_offset":36692,"ancestor_record_offset":34348,"inlinee":5047,"source_line":2461,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":509,"record_offset":35608,"parent_record_offset":35584,"ancestor_record_offset":34348,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":509,"record_offset":35584,"parent_record_offset":35560,"ancestor_record_offset":34348,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":509,"record_offset":35200,"parent_record_offset":35168,"ancestor_record_offset":34348,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$3::call_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":509,"record_offset":37248,"parent_record_offset":37224,"ancestor_record_offset":34348,"inlinee":4214,"source_line":297,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":509,"record_offset":36224,"parent_record_offset":36200,"ancestor_record_offset":34348,"inlinee":5124,"source_line":309,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":509,"record_offset":35900,"parent_record_offset":35256,"ancestor_record_offset":34348,"inlinee":4102,"source_line":766,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":509,"record_offset":34868,"parent_record_offset":34844,"ancestor_record_offset":34348,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":509,"record_offset":34844,"parent_record_offset":34532,"ancestor_record_offset":34348,"inlinee":11848,"source_line":1438,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":509,"record_offset":36004,"parent_record_offset":35980,"ancestor_record_offset":34348,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":509,"record_offset":35932,"parent_record_offset":35228,"ancestor_record_offset":34348,"inlinee":7428,"source_line":1647,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":509,"record_offset":34796,"parent_record_offset":34708,"ancestor_record_offset":34348,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":509,"record_offset":35704,"parent_record_offset":35680,"ancestor_record_offset":34348,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":509,"record_offset":37420,"parent_record_offset":37396,"ancestor_record_offset":34348,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":509,"record_offset":37444,"parent_record_offset":37420,"ancestor_record_offset":34348,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::starts_with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":509,"record_offset":37372,"parent_record_offset":37348,"ancestor_record_offset":34348,"inlinee":8287,"source_line":2619,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::starts_with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":509,"record_offset":37324,"parent_record_offset":37300,"ancestor_record_offset":34348,"inlinee":8324,"source_line":1405,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$31::is_prefix_of
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":509,"record_offset":37348,"parent_record_offset":37324,"ancestor_record_offset":34348,"inlinee":8286,"source_line":981,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::clutter_registry::ClutterId::is_builtin
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/clutter_registry.rs
//     evidence={"module":509,"record_offset":37300,"parent_record_offset":37272,"ancestor_record_offset":34348,"inlinee":29165,"source_line":52,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_storage::DecoratorStorage::iter
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_storage.rs
//     evidence={"module":509,"record_offset":34532,"parent_record_offset":34512,"ancestor_record_offset":34348,"inlinee":29137,"source_line":742,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::save::required_clutter::external_clutter_in_storage::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/save/required_clutter.rs
//     evidence={"module":509,"record_offset":36248,"parent_record_offset":36224,"ancestor_record_offset":34348,"inlinee":29161,"source_line":64,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::save::required_clutter::external_clutter_in_storage::closure$0::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/save/required_clutter.rs
//     evidence={"module":509,"record_offset":37272,"parent_record_offset":37248,"ancestor_record_offset":34348,"inlinee":29163,"source_line":68,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/bitmask.rs
//     evidence={"module":509,"record_offset":35524,"parent_record_offset":35488,"ancestor_record_offset":34348,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/bitmask.rs
//     evidence={"module":509,"record_offset":35560,"parent_record_offset":35524,"ancestor_record_offset":34348,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::remove_lowest_bit
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/bitmask.rs
//     evidence={"module":509,"record_offset":35648,"parent_record_offset":35488,"ancestor_record_offset":34348,"inlinee":4664,"source_line":37,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/bitmask.rs
//     evidence={"module":509,"record_offset":35488,"parent_record_offset":35444,"ancestor_record_offset":34348,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":509,"record_offset":34740,"parent_record_offset":34708,"ancestor_record_offset":34348,"inlinee":4654,"source_line":59,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":509,"record_offset":34596,"parent_record_offset":34572,"ancestor_record_offset":34348,"inlinee":8380,"source_line":754,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$44::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":509,"record_offset":35368,"parent_record_offset":35340,"ancestor_record_offset":34348,"inlinee":8502,"source_line":3219,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":509,"record_offset":35680,"parent_record_offset":35444,"ancestor_record_offset":34348,"inlinee":4660,"source_line":490,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":509,"record_offset":34708,"parent_record_offset":34652,"ancestor_record_offset":34348,"inlinee":4653,"source_line":3584,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":509,"record_offset":35444,"parent_record_offset":35404,"ancestor_record_offset":34348,"inlinee":4659,"source_line":3651,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":509,"record_offset":34624,"parent_record_offset":34596,"ancestor_record_offset":34348,"inlinee":8381,"source_line":1395,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":509,"record_offset":34684,"parent_record_offset":34652,"ancestor_record_offset":34348,"inlinee":4651,"source_line":2478,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":509,"record_offset":34652,"parent_record_offset":34624,"ancestor_record_offset":34348,"inlinee":4650,"source_line":2178,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":509,"record_offset":35404,"parent_record_offset":35368,"ancestor_record_offset":34348,"inlinee":4658,"source_line":3848,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":509,"record_offset":34572,"parent_record_offset":34532,"ancestor_record_offset":34348,"inlinee":8401,"source_line":638,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::values
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":509,"record_offset":36272,"parent_record_offset":36248,"ancestor_record_offset":34348,"inlinee":8400,"source_line":543,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$38::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":509,"record_offset":35340,"parent_record_offset":35312,"ancestor_record_offset":34348,"inlinee":8501,"source_line":2077,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$52::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":509,"record_offset":36744,"parent_record_offset":36716,"ancestor_record_offset":34348,"inlinee":10754,"source_line":2236,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::required_clutter::external_clutter_in_storage
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 61
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x71E2
// 首条位置: {"module":509,"inlinee":29154}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::resources::walls::decorator_storage::DecoratorStorage::iter
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_storage.rs
//     evidence={"module":509,"record_offset":28992,"parent_record_offset":28968,"ancestor_record_offset":28968,"inlinee":29137,"source_line":742,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":509,"record_offset":29268,"parent_record_offset":29240,"ancestor_record_offset":28968,"inlinee":4655,"source_line":1319,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":509,"record_offset":29320,"parent_record_offset":29296,"ancestor_record_offset":28968,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":509,"record_offset":29428,"parent_record_offset":29404,"ancestor_record_offset":28968,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::iter
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":509,"record_offset":29404,"parent_record_offset":28992,"ancestor_record_offset":28968,"inlinee":11848,"source_line":1438,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":509,"record_offset":29212,"parent_record_offset":29184,"ancestor_record_offset":28968,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":509,"record_offset":29240,"parent_record_offset":29184,"ancestor_record_offset":28968,"inlinee":4654,"source_line":59,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty_or_deleted
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":509,"record_offset":29296,"parent_record_offset":29240,"ancestor_record_offset":28968,"inlinee":7442,"source_line":101,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":509,"record_offset":29052,"parent_record_offset":29028,"ancestor_record_offset":28968,"inlinee":8380,"source_line":754,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":509,"record_offset":29184,"parent_record_offset":29100,"ancestor_record_offset":28968,"inlinee":4653,"source_line":3584,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":509,"record_offset":29076,"parent_record_offset":29052,"ancestor_record_offset":28968,"inlinee":8381,"source_line":1395,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::buckets
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":509,"record_offset":29156,"parent_record_offset":29100,"ancestor_record_offset":28968,"inlinee":8402,"source_line":2675,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":509,"record_offset":29128,"parent_record_offset":29100,"ancestor_record_offset":28968,"inlinee":4651,"source_line":2478,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":509,"record_offset":29100,"parent_record_offset":29076,"ancestor_record_offset":28968,"inlinee":4650,"source_line":2178,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":509,"record_offset":29028,"parent_record_offset":28992,"ancestor_record_offset":28968,"inlinee":8401,"source_line":638,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::required_clutter::external_clutter_in_storage::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 64
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x71E9
// 首条位置: {"module":509,"inlinee":29161}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::collections::hash::map::HashMap::values
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":509,"record_offset":36272,"parent_record_offset":36248,"ancestor_record_offset":36248,"inlinee":8400,"source_line":543,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":509,"record_offset":36480,"parent_record_offset":36456,"ancestor_record_offset":36248,"inlinee":4655,"source_line":1319,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1682,"record_offset":146048,"parent_record_offset":146024,"ancestor_record_offset":145676,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":509,"record_offset":36512,"parent_record_offset":36428,"ancestor_record_offset":36248,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":509,"record_offset":36456,"parent_record_offset":36428,"ancestor_record_offset":36248,"inlinee":4654,"source_line":59,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty_or_deleted
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1682,"record_offset":146024,"parent_record_offset":145968,"ancestor_record_offset":145676,"inlinee":7442,"source_line":101,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":509,"record_offset":36320,"parent_record_offset":36296,"ancestor_record_offset":36248,"inlinee":8380,"source_line":754,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":509,"record_offset":36428,"parent_record_offset":36368,"ancestor_record_offset":36248,"inlinee":4653,"source_line":3584,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":509,"record_offset":36344,"parent_record_offset":36320,"ancestor_record_offset":36248,"inlinee":8381,"source_line":1395,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::buckets
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1682,"record_offset":145880,"parent_record_offset":145816,"ancestor_record_offset":145676,"inlinee":8402,"source_line":2675,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":509,"record_offset":36400,"parent_record_offset":36368,"ancestor_record_offset":36248,"inlinee":4651,"source_line":2478,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":509,"record_offset":36368,"parent_record_offset":36344,"ancestor_record_offset":36248,"inlinee":4650,"source_line":2178,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":509,"record_offset":36296,"parent_record_offset":36272,"ancestor_record_offset":36248,"inlinee":8401,"source_line":638,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::required_clutter::external_clutter_in_storage::closure$0::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=10
// 源码行: 68
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x71EB
// 首条位置: {"module":509,"inlinee":29163}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::resources::clutter_registry::ClutterId::is_builtin
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/clutter_registry.rs
//     evidence={"module":509,"record_offset":37300,"parent_record_offset":37272,"ancestor_record_offset":37272,"inlinee":29165,"source_line":52,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":509,"record_offset":37396,"parent_record_offset":37372,"ancestor_record_offset":37272,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":509,"record_offset":37420,"parent_record_offset":37396,"ancestor_record_offset":37272,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":509,"record_offset":37444,"parent_record_offset":37420,"ancestor_record_offset":37272,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::starts_with
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":509,"record_offset":37372,"parent_record_offset":37348,"ancestor_record_offset":37272,"inlinee":8287,"source_line":2619,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::starts_with
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":509,"record_offset":37324,"parent_record_offset":37300,"ancestor_record_offset":37272,"inlinee":8324,"source_line":1405,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$31::is_prefix_of
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":509,"record_offset":37348,"parent_record_offset":37324,"ancestor_record_offset":37272,"inlinee":8286,"source_line":981,"archive":"_pdb/C/modules/00509.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::required_clutter::warn_missing_mods_on_load::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 87
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13DCD
// 首条位置: {"module":1643,"inlinee":81357}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::resources::clutter_registry::ClutterId::as_str
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/clutter_registry.rs
//     evidence={"module":1643,"record_offset":626932,"parent_record_offset":626908,"ancestor_record_offset":626908,"inlinee":31752,"source_line":48,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] ustr::Ustr::as_str
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ustr","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ustr-1.0.0/src/lib.rs
//     evidence={"module":1643,"record_offset":626956,"parent_record_offset":626932,"ancestor_record_offset":626908,"inlinee":13928,"source_line":239,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] ustr::Ustr::len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ustr","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ustr-1.0.0/src/lib.rs
//     evidence={"module":1643,"record_offset":626980,"parent_record_offset":626956,"ancestor_record_offset":626908,"inlinee":13929,"source_line":308,"archive":"_pdb/C/modules/01643.jsonl.gz"}
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
// 完整结构及原始类型记录: required_clutter.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: required_clutter.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: required_clutter.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 34; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>
// [文件级接口候选，具体函数归属 unknown] void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,country_core::startup::swap_replacement_mods::ModSwapReloads)
// 候选参数类型: ["enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >", "country_core::startup::swap_replacement_mods::ModSwapReloads"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::missing_mods::ShowMissingModsPopupCmd>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>", "bevy_ecs::change_detection::Res<country_core::resources::clutter_registry::ClutterRegistry>", "bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>", "enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >", "bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>", "bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::missing_mods::ShowMissingModsPopupCmd>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::clutter_registry::ClutterRegistry>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::missing_mods::ShowMissingModsPopupCmd>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> > >,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods> > >,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>", "bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>", "bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>", "bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>", "enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> > >", "enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods> > >", "bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>
// [文件级接口候选，具体函数归属 unknown] void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >,bevy_ecs::change_detection::Res<country_core::resources::mod_status::ModStatus>)
// 候选参数类型: ["enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >", "bevy_ecs::change_detection::Res<country_core::resources::mod_status::ModStatus>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::NextSession>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mod_status::ModStatus>
// [文件级接口候选，具体函数归属 unknown] void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,country_core::startup::swap_replacement_mods::ModSwapReloads)
// 候选参数类型: ["enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >", "bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>", "bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>", "country_core::startup::swap_replacement_mods::ModSwapReloads"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::NextSession>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>
// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<std::fs::DirEntry> > (*)(enum2$<core::result::Result<std::fs::DirEntry,std::io::error::Error> >)
// 候选参数类型: ["enum2$<core::result::Result<std::fs::DirEntry,std::io::error::Error> >"]
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::required_clutter::collect_missing_external_ids
// 信息 ID: 47e48fe4aaab5d71f8e48a87
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::save::required_clutter::warn_missing_mods_on_load; module=509, inline_record=28936; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::steam::missing_mods::refresh_missing_mods_on_pack_change; module=529, inline_record=53104; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::required_clutter::collect_missing_external_ids::closure$0
// 信息 ID: 32da9001142f3c038b348aa0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::save::required_clutter::collect_missing_external_ids", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::save::required_clutter::collect_missing_external_ids::closure_env$0 <- alloc::collections::btree::set::impl$15::from_iter<country_core::resources::clutter_registry::ClutterId,core::iter::adapters::filter::Filter<core::iter::adapters::flatten::FlatMap<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<std::collections::hash::map::Iter<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_storage::impl$17::iter::closure_env$0>,core::iter::adapters::map::Map<core::option::Iter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_storage::impl$17::iter::closure_env$1> >,core::iter::adapters::map::Map<core::option::Iter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_storage::impl$17::iter::closure_env$2> >,core::iter::adapters::filter_map::FilterMap<std::collections::hash::map::Values<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,country_core::systems::save::required_clutter::external_clutter_in_storage::closure$0::closure_env$0>,country_core::systems::save::required_clutter::external_clutter_in_storage::closure_env$0>,country_core::systems::save::required_clutter::collect_missing_external_ids::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::save::required_clutter::collect_missing_external_ids::closure_env$0 <- core::iter::adapters::filter::impl$3::next<core::iter::adapters::flatten::FlatMap<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<std::collections::hash::map::Iter<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_storage::impl$17::iter::closure_env$0>,core::iter::adapters::map::Map<core::option::Iter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_storage::impl$17::iter::closure_env$1> >,core::iter::adapters::map::Map<core::option::Iter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_storage::impl$17::iter::closure_env$2> >,core::iter::adapters::filter_map::FilterMap<std::collections::hash::map::Values<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,country_core::systems::save::required_clutter::external_clutter_in_storage::closure$0::closure_env$0>,country_core::systems::save::required_clutter::external_clutter_in_storage::closure_env$0>,country_core::systems::save::required_clutter::collect_missing_external_ids::closure_env$0>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::save::required_clutter::collect_missing_external_ids::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<country_core::resources::clutter_registry::ClutterId,core::iter::adapters::filter::Filter<core::iter::adapters::flatten::FlatMap<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<std::collections::hash::map::Iter<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_storage::impl$17::iter::closure_env$0>,core::iter::adapters::map::Map<core::option::Iter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_storage::impl$17::iter::closure_env$1> >,core::iter::adapters::map::Map<core::option::Iter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_storage::impl$17::iter::closure_env$2> >,core::iter::adapters::filter_map::FilterMap<std::collections::hash::map::Values<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,country_core::systems::save::required_clutter::external_clutter_in_storage::closure$0::closure_env$0>,country_core::systems::save::required_clutter::external_clutter_in_storage::closure_env$0>,country_core::systems::save::required_clutter::collect_missing_external_ids::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::save::required_clutter::collect_missing_external_ids::closure_env$0 <- core::iter::adapters::map::impl$2::try_fold<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,ref$<country_core::resources::walls::decorator_storage::Decorators> >,std::collections::hash::map::Iter<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_storage::impl$17::iter::closure_env$0,tuple$<>,ref_mut$<ref_mut$<core::iter::adapters::map::map_try_fold::closure_env$0<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,ref$<country_core::resources::walls::decorator_storage::Decorators> >,core::iter::adapters::filter_map::FilterMap<std::collections::hash::map::Values<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,country_core::systems::save::required_clutter::external_clutter_in_storage::closure$0::closure_env$0>,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::clutter_registry::ClutterId,tuple$<> > >,country_core::systems::save::required_clutter::external_clutter_in_storage::closure_env$0,core::iter::adapters::flatten::impl$18::iter_try_fold::flatten::closure_env$0<core::iter::adapters::filter_map::FilterMap<std::collections::hash::map::Values<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,country_core::systems::save::required_clutter::external_clutter_in_storage::closure$0::closure_env$0>,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::clutter_registry::ClutterId,tuple$<> > >,core::iter::adapters::flatten::impl$20::try_fold::flatten::closure_env$0<core::iter::adapters::filter_map::FilterMap<std::collections::hash::map::Values<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,country_core::systems::save::required_clutter::external_clutter_in_storage::closure$0::closure_env$0>,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::clutter_registry::ClutterId,tuple$<> > >,core::iter::traits::iterator::Iterator::find::check::closure_env$0<country_core::resources::clutter_registry::ClutterId,ref_mut$<country_core::systems::save::required_clutter::collect_missing_external_ids::closure_env$0> > > > > > >,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::clutter_registry::ClutterId,tuple$<> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::save::required_clutter::collect_missing_external_ids::closure_env$0 <- core::iter::adapters::map::impl$2::try_fold<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,ref$<country_core::resources::walls::decorator_storage::Decorators> >,core::option::Iter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_storage::impl$17::iter::closure_env$2,tuple$<>,core::iter::adapters::map::map_try_fold::closure_env$0<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,ref$<country_core::resources::walls::decorator_storage::Decorators> >,core::iter::adapters::filter_map::FilterMap<std::collections::hash::map::Values<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,country_core::systems::save::required_clutter::external_clutter_in_storage::closure$0::closure_env$0>,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::clutter_registry::ClutterId,tuple$<> > >,country_core::systems::save::required_clutter::external_clutter_in_storage::closure_env$0,core::iter::adapters::flatten::impl$18::iter_try_fold::flatten::closure_env$0<core::iter::adapters::filter_map::FilterMap<std::collections::hash::map::Values<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,country_core::systems::save::required_clutter::external_clutter_in_storage::closure$0::closure_env$0>,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::clutter_registry::ClutterId,tuple$<> > >,core::iter::adapters::flatten::impl$20::try_fold::flatten::closure_env$0<core::iter::adapters::filter_map::FilterMap<std::collections::hash::map::Values<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,country_core::systems::save::required_clutter::external_clutter_in_storage::closure$0::closure_env$0>,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::clutter_registry::ClutterId,tuple$<> > >,core::iter::traits::iterator::Iterator::find::check::closure_env$0<country_core::resources::clutter_registry::ClutterId,ref_mut$<country_core::systems::save::required_clutter::collect_missing_external_ids::closure_env$0> > > > >,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::clutter_registry::ClutterId,tuple$<> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::save::required_clutter::collect_missing_external_ids::closure_env$0 <- core::iter::adapters::map::impl$2::try_fold<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,ref$<country_core::resources::walls::decorator_storage::Decorators> >,core::option::Iter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_storage::impl$17::iter::closure_env$1,tuple$<>,ref_mut$<core::iter::adapters::map::map_try_fold::closure_env$0<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,ref$<country_core::resources::walls::decorator_storage::Decorators> >,core::iter::adapters::filter_map::FilterMap<std::collections::hash::map::Values<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,country_core::systems::save::required_clutter::external_clutter_in_storage::closure$0::closure_env$0>,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::clutter_registry::ClutterId,tuple$<> > >,country_core::systems::save::required_clutter::external_clutter_in_storage::closure_env$0,core::iter::adapters::flatten::impl$18::iter_try_fold::flatten::closure_env$0<core::iter::adapters::filter_map::FilterMap<std::collections::hash::map::Values<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,country_core::systems::save::required_clutter::external_clutter_in_storage::closure$0::closure_env$0>,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::clutter_registry::ClutterId,tuple$<> > >,core::iter::adapters::flatten::impl$20::try_fold::flatten::closure_env$0<core::iter::adapters::filter_map::FilterMap<std::collections::hash::map::Values<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,country_core::systems::save::required_clutter::external_clutter_in_storage::closure$0::closure_env$0>,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::clutter_registry::ClutterId,tuple$<> > >,core::iter::traits::iterator::Iterator::find::check::closure_env$0<country_core::resources::clutter_registry::ClutterId,ref_mut$<country_core::systems::save::required_clutter::collect_missing_external_ids::closure_env$0> > > > > >,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::clutter_registry::ClutterId,tuple$<> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 5 条；闭包环境类型引用 6 条。
// [PDB 内联宿主] core::iter::adapters::filter::impl$3::next<core::iter::adapters::flatten::FlatMap<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<std::collections::hash::map::Iter<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_storage::impl$17::iter::closure_env$0>,core::iter::adapters::map::Map<core::option::Iter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_storage::impl$17::iter::closure_env$1> >,core::iter::adapters::map::Map<core::option::Iter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_storage::impl$17::iter::closure_env$2> >,core::iter::adapters::filter_map::FilterMap<std::collections::hash::map::Values<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,country_core::systems::save::required_clutter::external_clutter_in_storage::closure$0::closure_env$0>,country_core::systems::save::required_clutter::external_clutter_in_storage::closure_env$0>,country_core::systems::save::required_clutter::collect_missing_external_ids::closure_env$0>; module=1643, inline_record=76900; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::filter::impl$3::next<core::iter::adapters::flatten::FlatMap<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<std::collections::hash::map::Iter<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_storage::impl$17::iter::closure_env$0>,core::iter::adapters::map::Map<core::option::Iter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_storage::impl$17::iter::closure_env$1> >,core::iter::adapters::map::Map<core::option::Iter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_storage::impl$17::iter::closure_env$2> >,core::iter::adapters::filter_map::FilterMap<std::collections::hash::map::Values<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,country_core::systems::save::required_clutter::external_clutter_in_storage::closure$0::closure_env$0>,country_core::systems::save::required_clutter::external_clutter_in_storage::closure_env$0>,country_core::systems::save::required_clutter::collect_missing_external_ids::closure_env$0>; module=1643, inline_record=79224; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::try_fold<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,ref$<country_core::resources::walls::decorator_storage::Decorators> >,std::collections::hash::map::Iter<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_storage::impl$17::iter::closure_env$0,tuple$<>,ref_mut$<ref_mut$<core::iter::adapters::map::map_try_fold::closure_env$0<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,ref$<country_core::resources::walls::decorator_storage::Decorators> >,core::iter::adapters::filter_map::FilterMap<std::collections::hash::map::Values<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,country_core::systems::save::required_clutter::external_clutter_in_storage::closure$0::closure_env$0>,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::clutter_registry::ClutterId,tuple$<> > >,country_core::systems::save::required_clutter::external_clutter_in_storage::closure_env$0,core::iter::adapters::flatten::impl$18::iter_try_fold::flatten::closure_env$0<core::iter::adapters::filter_map::FilterMap<std::collections::hash::map::Values<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,country_core::systems::save::required_clutter::external_clutter_in_storage::closure$0::closure_env$0>,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::clutter_registry::ClutterId,tuple$<> > >,core::iter::adapters::flatten::impl$20::try_fold::flatten::closure_env$0<core::iter::adapters::filter_map::FilterMap<std::collections::hash::map::Values<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,country_core::systems::save::required_clutter::external_clutter_in_storage::closure$0::closure_env$0>,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::clutter_registry::ClutterId,tuple$<> > >,core::iter::traits::iterator::Iterator::find::check::closure_env$0<country_core::resources::clutter_registry::ClutterId,ref_mut$<country_core::systems::save::required_clutter::collect_missing_external_ids::closure_env$0> > > > > > >,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::clutter_registry::ClutterId,tuple$<> > > >; module=1682, inline_record=147252; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::try_fold<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,ref$<country_core::resources::walls::decorator_storage::Decorators> >,core::option::Iter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_storage::impl$17::iter::closure_env$2,tuple$<>,core::iter::adapters::map::map_try_fold::closure_env$0<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,ref$<country_core::resources::walls::decorator_storage::Decorators> >,core::iter::adapters::filter_map::FilterMap<std::collections::hash::map::Values<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,country_core::systems::save::required_clutter::external_clutter_in_storage::closure$0::closure_env$0>,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::clutter_registry::ClutterId,tuple$<> > >,country_core::systems::save::required_clutter::external_clutter_in_storage::closure_env$0,core::iter::adapters::flatten::impl$18::iter_try_fold::flatten::closure_env$0<core::iter::adapters::filter_map::FilterMap<std::collections::hash::map::Values<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,country_core::systems::save::required_clutter::external_clutter_in_storage::closure$0::closure_env$0>,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::clutter_registry::ClutterId,tuple$<> > >,core::iter::adapters::flatten::impl$20::try_fold::flatten::closure_env$0<core::iter::adapters::filter_map::FilterMap<std::collections::hash::map::Values<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,country_core::systems::save::required_clutter::external_clutter_in_storage::closure$0::closure_env$0>,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::clutter_registry::ClutterId,tuple$<> > >,core::iter::traits::iterator::Iterator::find::check::closure_env$0<country_core::resources::clutter_registry::ClutterId,ref_mut$<country_core::systems::save::required_clutter::collect_missing_external_ids::closure_env$0> > > > >,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::clutter_registry::ClutterId,tuple$<> > > >; module=1682, inline_record=152724; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::try_fold<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,ref$<country_core::resources::walls::decorator_storage::Decorators> >,core::option::Iter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_storage::impl$17::iter::closure_env$1,tuple$<>,ref_mut$<core::iter::adapters::map::map_try_fold::closure_env$0<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,ref$<country_core::resources::walls::decorator_storage::Decorators> >,core::iter::adapters::filter_map::FilterMap<std::collections::hash::map::Values<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,country_core::systems::save::required_clutter::external_clutter_in_storage::closure$0::closure_env$0>,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::clutter_registry::ClutterId,tuple$<> > >,country_core::systems::save::required_clutter::external_clutter_in_storage::closure_env$0,core::iter::adapters::flatten::impl$18::iter_try_fold::flatten::closure_env$0<core::iter::adapters::filter_map::FilterMap<std::collections::hash::map::Values<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,country_core::systems::save::required_clutter::external_clutter_in_storage::closure$0::closure_env$0>,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::clutter_registry::ClutterId,tuple$<> > >,core::iter::adapters::flatten::impl$20::try_fold::flatten::closure_env$0<core::iter::adapters::filter_map::FilterMap<std::collections::hash::map::Values<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,country_core::systems::save::required_clutter::external_clutter_in_storage::closure$0::closure_env$0>,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::clutter_registry::ClutterId,tuple$<> > >,core::iter::traits::iterator::Iterator::find::check::closure_env$0<country_core::resources::clutter_registry::ClutterId,ref_mut$<country_core::systems::save::required_clutter::collect_missing_external_ids::closure_env$0> > > > > >,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::clutter_registry::ClutterId,tuple$<> > > >; module=1682, inline_record=172516; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::required_clutter::external_clutter_in_storage
// 信息 ID: 6a2af3c0dc74b1c362a832e8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::save::required_clutter::warn_missing_mods_on_load; module=509, inline_record=28968; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::save::required_clutter::glade_uses_external_clutter; module=509, inline_record=34512; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::steam::missing_mods::refresh_missing_mods_on_pack_change; module=529, inline_record=53136; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::required_clutter::external_clutter_in_storage::closure$0
// 信息 ID: 458a8c60f31546fc731af611
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::save::required_clutter::external_clutter_in_storage", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::save::required_clutter::external_clutter_in_storage::closure_env$0 <- alloc::collections::btree::set::impl$15::from_iter<country_core::resources::clutter_registry::ClutterId,core::iter::adapters::filter::Filter<core::iter::adapters::flatten::FlatMap<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<std::collections::hash::map::Iter<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_storage::impl$17::iter::closure_env$0>,core::iter::adapters::map::Map<core::option::Iter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_storage::impl$17::iter::closure_env$1> >,core::iter::adapters::map::Map<core::option::Iter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_storage::impl$17::iter::closure_env$2> >,core::iter::adapters::filter_map::FilterMap<std::collections::hash::map::Values<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,country_core::systems::save::required_clutter::external_clutter_in_storage::closure$0::closure_env$0>,country_core::systems::save::required_clutter::external_clutter_in_storage::closure_env$0>,country_core::systems::save::required_clutter::collect_missing_external_ids::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::save::required_clutter::external_clutter_in_storage::closure_env$0 <- core::iter::adapters::filter::impl$3::next<core::iter::adapters::flatten::FlatMap<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<std::collections::hash::map::Iter<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_storage::impl$17::iter::closure_env$0>,core::iter::adapters::map::Map<core::option::Iter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_storage::impl$17::iter::closure_env$1> >,core::iter::adapters::map::Map<core::option::Iter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_storage::impl$17::iter::closure_env$2> >,core::iter::adapters::filter_map::FilterMap<std::collections::hash::map::Values<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,country_core::systems::save::required_clutter::external_clutter_in_storage::closure$0::closure_env$0>,country_core::systems::save::required_clutter::external_clutter_in_storage::closure_env$0>,country_core::systems::save::required_clutter::collect_missing_external_ids::closure_env$0>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::save::required_clutter::external_clutter_in_storage::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<country_core::resources::clutter_registry::ClutterId,core::iter::adapters::filter::Filter<core::iter::adapters::flatten::FlatMap<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<std::collections::hash::map::Iter<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_storage::impl$17::iter::closure_env$0>,core::iter::adapters::map::Map<core::option::Iter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_storage::impl$17::iter::closure_env$1> >,core::iter::adapters::map::Map<core::option::Iter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_storage::impl$17::iter::closure_env$2> >,core::iter::adapters::filter_map::FilterMap<std::collections::hash::map::Values<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,country_core::systems::save::required_clutter::external_clutter_in_storage::closure$0::closure_env$0>,country_core::systems::save::required_clutter::external_clutter_in_storage::closure_env$0>,country_core::systems::save::required_clutter::collect_missing_external_ids::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::save::required_clutter::external_clutter_in_storage::closure_env$0 <- core::iter::adapters::map::impl$2::try_fold<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,ref$<country_core::resources::walls::decorator_storage::Decorators> >,std::collections::hash::map::Iter<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_storage::impl$17::iter::closure_env$0,tuple$<>,ref_mut$<ref_mut$<core::iter::adapters::map::map_try_fold::closure_env$0<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,ref$<country_core::resources::walls::decorator_storage::Decorators> >,core::iter::adapters::filter_map::FilterMap<std::collections::hash::map::Values<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,country_core::systems::save::required_clutter::external_clutter_in_storage::closure$0::closure_env$0>,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::clutter_registry::ClutterId,tuple$<> > >,country_core::systems::save::required_clutter::external_clutter_in_storage::closure_env$0,core::iter::adapters::flatten::impl$18::iter_try_fold::flatten::closure_env$0<core::iter::adapters::filter_map::FilterMap<std::collections::hash::map::Values<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,country_core::systems::save::required_clutter::external_clutter_in_storage::closure$0::closure_env$0>,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::clutter_registry::ClutterId,tuple$<> > >,core::iter::adapters::flatten::impl$20::try_fold::flatten::closure_env$0<core::iter::adapters::filter_map::FilterMap<std::collections::hash::map::Values<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,country_core::systems::save::required_clutter::external_clutter_in_storage::closure$0::closure_env$0>,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::clutter_registry::ClutterId,tuple$<> > >,core::iter::traits::iterator::Iterator::find::check::closure_env$0<country_core::resources::clutter_registry::ClutterId,ref_mut$<country_core::systems::save::required_clutter::collect_missing_external_ids::closure_env$0> > > > > > >,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::clutter_registry::ClutterId,tuple$<> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::save::required_clutter::external_clutter_in_storage::closure_env$0 <- core::iter::adapters::map::impl$2::try_fold<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,ref$<country_core::resources::walls::decorator_storage::Decorators> >,core::option::Iter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_storage::impl$17::iter::closure_env$2,tuple$<>,core::iter::adapters::map::map_try_fold::closure_env$0<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,ref$<country_core::resources::walls::decorator_storage::Decorators> >,core::iter::adapters::filter_map::FilterMap<std::collections::hash::map::Values<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,country_core::systems::save::required_clutter::external_clutter_in_storage::closure$0::closure_env$0>,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::clutter_registry::ClutterId,tuple$<> > >,country_core::systems::save::required_clutter::external_clutter_in_storage::closure_env$0,core::iter::adapters::flatten::impl$18::iter_try_fold::flatten::closure_env$0<core::iter::adapters::filter_map::FilterMap<std::collections::hash::map::Values<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,country_core::systems::save::required_clutter::external_clutter_in_storage::closure$0::closure_env$0>,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::clutter_registry::ClutterId,tuple$<> > >,core::iter::adapters::flatten::impl$20::try_fold::flatten::closure_env$0<core::iter::adapters::filter_map::FilterMap<std::collections::hash::map::Values<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,country_core::systems::save::required_clutter::external_clutter_in_storage::closure$0::closure_env$0>,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::clutter_registry::ClutterId,tuple$<> > >,core::iter::traits::iterator::Iterator::find::check::closure_env$0<country_core::resources::clutter_registry::ClutterId,ref_mut$<country_core::systems::save::required_clutter::collect_missing_external_ids::closure_env$0> > > > >,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::clutter_registry::ClutterId,tuple$<> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::save::required_clutter::external_clutter_in_storage::closure_env$0 <- core::iter::adapters::map::impl$2::try_fold<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,ref$<country_core::resources::walls::decorator_storage::Decorators> >,core::option::Iter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_storage::impl$17::iter::closure_env$1,tuple$<>,ref_mut$<core::iter::adapters::map::map_try_fold::closure_env$0<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,ref$<country_core::resources::walls::decorator_storage::Decorators> >,core::iter::adapters::filter_map::FilterMap<std::collections::hash::map::Values<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,country_core::systems::save::required_clutter::external_clutter_in_storage::closure$0::closure_env$0>,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::clutter_registry::ClutterId,tuple$<> > >,country_core::systems::save::required_clutter::external_clutter_in_storage::closure_env$0,core::iter::adapters::flatten::impl$18::iter_try_fold::flatten::closure_env$0<core::iter::adapters::filter_map::FilterMap<std::collections::hash::map::Values<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,country_core::systems::save::required_clutter::external_clutter_in_storage::closure$0::closure_env$0>,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::clutter_registry::ClutterId,tuple$<> > >,core::iter::adapters::flatten::impl$20::try_fold::flatten::closure_env$0<core::iter::adapters::filter_map::FilterMap<std::collections::hash::map::Values<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,country_core::systems::save::required_clutter::external_clutter_in_storage::closure$0::closure_env$0>,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::clutter_registry::ClutterId,tuple$<> > >,core::iter::traits::iterator::Iterator::find::check::closure_env$0<country_core::resources::clutter_registry::ClutterId,ref_mut$<country_core::systems::save::required_clutter::collect_missing_external_ids::closure_env$0> > > > > >,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::clutter_registry::ClutterId,tuple$<> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 6 条。
// [PDB 内联宿主] country_core::systems::save::required_clutter::glade_uses_external_clutter; module=509, inline_record=36248; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::try_fold<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,ref$<country_core::resources::walls::decorator_storage::Decorators> >,std::collections::hash::map::Iter<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_storage::impl$17::iter::closure_env$0,tuple$<>,ref_mut$<ref_mut$<core::iter::adapters::map::map_try_fold::closure_env$0<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,ref$<country_core::resources::walls::decorator_storage::Decorators> >,core::iter::adapters::filter_map::FilterMap<std::collections::hash::map::Values<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,country_core::systems::save::required_clutter::external_clutter_in_storage::closure$0::closure_env$0>,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::clutter_registry::ClutterId,tuple$<> > >,country_core::systems::save::required_clutter::external_clutter_in_storage::closure_env$0,core::iter::adapters::flatten::impl$18::iter_try_fold::flatten::closure_env$0<core::iter::adapters::filter_map::FilterMap<std::collections::hash::map::Values<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,country_core::systems::save::required_clutter::external_clutter_in_storage::closure$0::closure_env$0>,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::clutter_registry::ClutterId,tuple$<> > >,core::iter::adapters::flatten::impl$20::try_fold::flatten::closure_env$0<core::iter::adapters::filter_map::FilterMap<std::collections::hash::map::Values<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,country_core::systems::save::required_clutter::external_clutter_in_storage::closure$0::closure_env$0>,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::clutter_registry::ClutterId,tuple$<> > >,core::iter::traits::iterator::Iterator::find::check::closure_env$0<country_core::resources::clutter_registry::ClutterId,ref_mut$<country_core::systems::save::required_clutter::collect_missing_external_ids::closure_env$0> > > > > > >,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::clutter_registry::ClutterId,tuple$<> > > >; module=1682, inline_record=145676; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::try_fold<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,ref$<country_core::resources::walls::decorator_storage::Decorators> >,core::option::Iter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_storage::impl$17::iter::closure_env$2,tuple$<>,core::iter::adapters::map::map_try_fold::closure_env$0<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,ref$<country_core::resources::walls::decorator_storage::Decorators> >,core::iter::adapters::filter_map::FilterMap<std::collections::hash::map::Values<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,country_core::systems::save::required_clutter::external_clutter_in_storage::closure$0::closure_env$0>,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::clutter_registry::ClutterId,tuple$<> > >,country_core::systems::save::required_clutter::external_clutter_in_storage::closure_env$0,core::iter::adapters::flatten::impl$18::iter_try_fold::flatten::closure_env$0<core::iter::adapters::filter_map::FilterMap<std::collections::hash::map::Values<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,country_core::systems::save::required_clutter::external_clutter_in_storage::closure$0::closure_env$0>,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::clutter_registry::ClutterId,tuple$<> > >,core::iter::adapters::flatten::impl$20::try_fold::flatten::closure_env$0<core::iter::adapters::filter_map::FilterMap<std::collections::hash::map::Values<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,country_core::systems::save::required_clutter::external_clutter_in_storage::closure$0::closure_env$0>,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::clutter_registry::ClutterId,tuple$<> > >,core::iter::traits::iterator::Iterator::find::check::closure_env$0<country_core::resources::clutter_registry::ClutterId,ref_mut$<country_core::systems::save::required_clutter::collect_missing_external_ids::closure_env$0> > > > >,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::clutter_registry::ClutterId,tuple$<> > > >; module=1682, inline_record=151148; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::try_fold<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,ref$<country_core::resources::walls::decorator_storage::Decorators> >,core::option::Iter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_storage::impl$17::iter::closure_env$1,tuple$<>,ref_mut$<core::iter::adapters::map::map_try_fold::closure_env$0<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,ref$<country_core::resources::walls::decorator_storage::Decorators> >,core::iter::adapters::filter_map::FilterMap<std::collections::hash::map::Values<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,country_core::systems::save::required_clutter::external_clutter_in_storage::closure$0::closure_env$0>,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::clutter_registry::ClutterId,tuple$<> > >,country_core::systems::save::required_clutter::external_clutter_in_storage::closure_env$0,core::iter::adapters::flatten::impl$18::iter_try_fold::flatten::closure_env$0<core::iter::adapters::filter_map::FilterMap<std::collections::hash::map::Values<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,country_core::systems::save::required_clutter::external_clutter_in_storage::closure$0::closure_env$0>,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::clutter_registry::ClutterId,tuple$<> > >,core::iter::adapters::flatten::impl$20::try_fold::flatten::closure_env$0<core::iter::adapters::filter_map::FilterMap<std::collections::hash::map::Values<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,country_core::systems::save::required_clutter::external_clutter_in_storage::closure$0::closure_env$0>,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::clutter_registry::ClutterId,tuple$<> > >,core::iter::traits::iterator::Iterator::find::check::closure_env$0<country_core::resources::clutter_registry::ClutterId,ref_mut$<country_core::systems::save::required_clutter::collect_missing_external_ids::closure_env$0> > > > > >,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::clutter_registry::ClutterId,tuple$<> > > >; module=1682, inline_record=170940; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::required_clutter::external_clutter_in_storage::closure$0::closure$0
// 信息 ID: 532adc46c3936d582d2a70e3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::save::required_clutter::external_clutter_in_storage", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::save::required_clutter::external_clutter_in_storage::closure$0::closure_env$0 <- alloc::collections::btree::set::impl$15::from_iter<country_core::resources::clutter_registry::ClutterId,core::iter::adapters::filter::Filter<core::iter::adapters::flatten::FlatMap<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<std::collections::hash::map::Iter<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_storage::impl$17::iter::closure_env$0>,core::iter::adapters::map::Map<core::option::Iter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_storage::impl$17::iter::closure_env$1> >,core::iter::adapters::map::Map<core::option::Iter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_storage::impl$17::iter::closure_env$2> >,core::iter::adapters::filter_map::FilterMap<std::collections::hash::map::Values<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,country_core::systems::save::required_clutter::external_clutter_in_storage::closure$0::closure_env$0>,country_core::systems::save::required_clutter::external_clutter_in_storage::closure_env$0>,country_core::systems::save::required_clutter::collect_missing_external_ids::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::save::required_clutter::external_clutter_in_storage::closure$0::closure_env$0 <- core::iter::adapters::filter::impl$3::next<core::iter::adapters::flatten::FlatMap<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<std::collections::hash::map::Iter<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_storage::impl$17::iter::closure_env$0>,core::iter::adapters::map::Map<core::option::Iter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_storage::impl$17::iter::closure_env$1> >,core::iter::adapters::map::Map<core::option::Iter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_storage::impl$17::iter::closure_env$2> >,core::iter::adapters::filter_map::FilterMap<std::collections::hash::map::Values<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,country_core::systems::save::required_clutter::external_clutter_in_storage::closure$0::closure_env$0>,country_core::systems::save::required_clutter::external_clutter_in_storage::closure_env$0>,country_core::systems::save::required_clutter::collect_missing_external_ids::closure_env$0>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::save::required_clutter::external_clutter_in_storage::closure$0::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<country_core::resources::clutter_registry::ClutterId,core::iter::adapters::filter::Filter<core::iter::adapters::flatten::FlatMap<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<std::collections::hash::map::Iter<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_storage::impl$17::iter::closure_env$0>,core::iter::adapters::map::Map<core::option::Iter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_storage::impl$17::iter::closure_env$1> >,core::iter::adapters::map::Map<core::option::Iter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_storage::impl$17::iter::closure_env$2> >,core::iter::adapters::filter_map::FilterMap<std::collections::hash::map::Values<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,country_core::systems::save::required_clutter::external_clutter_in_storage::closure$0::closure_env$0>,country_core::systems::save::required_clutter::external_clutter_in_storage::closure_env$0>,country_core::systems::save::required_clutter::collect_missing_external_ids::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::save::required_clutter::external_clutter_in_storage::closure$0::closure_env$0 <- core::iter::adapters::map::impl$2::try_fold<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,ref$<country_core::resources::walls::decorator_storage::Decorators> >,std::collections::hash::map::Iter<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_storage::impl$17::iter::closure_env$0,tuple$<>,ref_mut$<ref_mut$<core::iter::adapters::map::map_try_fold::closure_env$0<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,ref$<country_core::resources::walls::decorator_storage::Decorators> >,core::iter::adapters::filter_map::FilterMap<std::collections::hash::map::Values<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,country_core::systems::save::required_clutter::external_clutter_in_storage::closure$0::closure_env$0>,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::clutter_registry::ClutterId,tuple$<> > >,country_core::systems::save::required_clutter::external_clutter_in_storage::closure_env$0,core::iter::adapters::flatten::impl$18::iter_try_fold::flatten::closure_env$0<core::iter::adapters::filter_map::FilterMap<std::collections::hash::map::Values<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,country_core::systems::save::required_clutter::external_clutter_in_storage::closure$0::closure_env$0>,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::clutter_registry::ClutterId,tuple$<> > >,core::iter::adapters::flatten::impl$20::try_fold::flatten::closure_env$0<core::iter::adapters::filter_map::FilterMap<std::collections::hash::map::Values<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,country_core::systems::save::required_clutter::external_clutter_in_storage::closure$0::closure_env$0>,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::clutter_registry::ClutterId,tuple$<> > >,core::iter::traits::iterator::Iterator::find::check::closure_env$0<country_core::resources::clutter_registry::ClutterId,ref_mut$<country_core::systems::save::required_clutter::collect_missing_external_ids::closure_env$0> > > > > > >,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::clutter_registry::ClutterId,tuple$<> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::save::required_clutter::external_clutter_in_storage::closure$0::closure_env$0 <- core::iter::adapters::map::impl$2::try_fold<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,ref$<country_core::resources::walls::decorator_storage::Decorators> >,std::collections::hash::map::Iter<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_storage::impl$17::iter::closure_env$0,tuple$<>,ref_mut$<ref_mut$<core::iter::adapters::map::map_try_fold::closure_env$0<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,ref$<country_core::resources::walls::decorator_storage::Decorators> >,core::iter::adapters::filter_map::FilterMap<std::collections::hash::map::Values<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,country_core::systems::save::required_clutter::external_clutter_in_storage::closure$0::closure_env$0>,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::clutter_registry::ClutterId,tuple$<> > >,country_core::systems::save::required_clutter::external_clutter_in_storage::closure_env$0,core::iter::adapters::flatten::impl$18::iter_try_fold::flatten::closure_env$0<core::iter::adapters::filter_map::FilterMap<std::collections::hash::map::Values<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,country_core::systems::save::required_clutter::external_clutter_in_storage::closure$0::closure_env$0>,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::clutter_registry::ClutterId,tuple$<> > >,core::iter::adapters::flatten::impl$20::try_fold::flatten::closure_env$0<core::iter::adapters::filter_map::FilterMap<std::collections::hash::map::Values<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,country_core::systems::save::required_clutter::external_clutter_in_storage::closure$0::closure_env$0>,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::clutter_registry::ClutterId,tuple$<> > >,core::iter::traits::iterator::Iterator::find::check::closure_env$0<country_core::resources::clutter_registry::ClutterId,ref_mut$<country_core::systems::save::required_clutter::collect_missing_external_ids::closure_env$0> > > > > > >,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::clutter_registry::ClutterId,tuple$<> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::save::required_clutter::external_clutter_in_storage::closure$0::closure_env$0 <- core::iter::adapters::map::impl$2::try_fold<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,ref$<country_core::resources::walls::decorator_storage::Decorators> >,std::collections::hash::map::Iter<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_storage::impl$17::iter::closure_env$0,tuple$<>,ref_mut$<ref_mut$<core::iter::adapters::map::map_try_fold::closure_env$0<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,ref$<country_core::resources::walls::decorator_storage::Decorators> >,core::iter::adapters::filter_map::FilterMap<std::collections::hash::map::Values<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,country_core::systems::save::required_clutter::external_clutter_in_storage::closure$0::closure_env$0>,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::clutter_registry::ClutterId,tuple$<> > >,country_core::systems::save::required_clutter::external_clutter_in_storage::closure_env$0,core::iter::adapters::flatten::impl$18::iter_try_fold::flatten::closure_env$0<core::iter::adapters::filter_map::FilterMap<std::collections::hash::map::Values<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,country_core::systems::save::required_clutter::external_clutter_in_storage::closure$0::closure_env$0>,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::clutter_registry::ClutterId,tuple$<> > >,core::iter::adapters::flatten::impl$20::try_fold::flatten::closure_env$0<core::iter::adapters::filter_map::FilterMap<std::collections::hash::map::Values<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,country_core::systems::save::required_clutter::external_clutter_in_storage::closure$0::closure_env$0>,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::clutter_registry::ClutterId,tuple$<> > >,core::iter::traits::iterator::Iterator::find::check::closure_env$0<country_core::resources::clutter_registry::ClutterId,ref_mut$<country_core::systems::save::required_clutter::collect_missing_external_ids::closure_env$0> > > > > > >,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::clutter_registry::ClutterId,tuple$<> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::save::required_clutter::external_clutter_in_storage::closure$0::closure_env$0 <- core::iter::adapters::map::impl$2::try_fold<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,ref$<country_core::resources::walls::decorator_storage::Decorators> >,core::option::Iter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_storage::impl$17::iter::closure_env$2,tuple$<>,core::iter::adapters::map::map_try_fold::closure_env$0<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,ref$<country_core::resources::walls::decorator_storage::Decorators> >,core::iter::adapters::filter_map::FilterMap<std::collections::hash::map::Values<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,country_core::systems::save::required_clutter::external_clutter_in_storage::closure$0::closure_env$0>,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::clutter_registry::ClutterId,tuple$<> > >,country_core::systems::save::required_clutter::external_clutter_in_storage::closure_env$0,core::iter::adapters::flatten::impl$18::iter_try_fold::flatten::closure_env$0<core::iter::adapters::filter_map::FilterMap<std::collections::hash::map::Values<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,country_core::systems::save::required_clutter::external_clutter_in_storage::closure$0::closure_env$0>,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::clutter_registry::ClutterId,tuple$<> > >,core::iter::adapters::flatten::impl$20::try_fold::flatten::closure_env$0<core::iter::adapters::filter_map::FilterMap<std::collections::hash::map::Values<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,country_core::systems::save::required_clutter::external_clutter_in_storage::closure$0::closure_env$0>,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::clutter_registry::ClutterId,tuple$<> > >,core::iter::traits::iterator::Iterator::find::check::closure_env$0<country_core::resources::clutter_registry::ClutterId,ref_mut$<country_core::systems::save::required_clutter::collect_missing_external_ids::closure_env$0> > > > >,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::clutter_registry::ClutterId,tuple$<> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::save::required_clutter::external_clutter_in_storage::closure$0::closure_env$0 <- core::iter::adapters::map::impl$2::try_fold<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,ref$<country_core::resources::walls::decorator_storage::Decorators> >,core::option::Iter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_storage::impl$17::iter::closure_env$2,tuple$<>,core::iter::adapters::map::map_try_fold::closure_env$0<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,ref$<country_core::resources::walls::decorator_storage::Decorators> >,core::iter::adapters::filter_map::FilterMap<std::collections::hash::map::Values<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,country_core::systems::save::required_clutter::external_clutter_in_storage::closure$0::closure_env$0>,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::clutter_registry::ClutterId,tuple$<> > >,country_core::systems::save::required_clutter::external_clutter_in_storage::closure_env$0,core::iter::adapters::flatten::impl$18::iter_try_fold::flatten::closure_env$0<core::iter::adapters::filter_map::FilterMap<std::collections::hash::map::Values<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,country_core::systems::save::required_clutter::external_clutter_in_storage::closure$0::closure_env$0>,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::clutter_registry::ClutterId,tuple$<> > >,core::iter::adapters::flatten::impl$20::try_fold::flatten::closure_env$0<core::iter::adapters::filter_map::FilterMap<std::collections::hash::map::Values<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,country_core::systems::save::required_clutter::external_clutter_in_storage::closure$0::closure_env$0>,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::clutter_registry::ClutterId,tuple$<> > >,core::iter::traits::iterator::Iterator::find::check::closure_env$0<country_core::resources::clutter_registry::ClutterId,ref_mut$<country_core::systems::save::required_clutter::collect_missing_external_ids::closure_env$0> > > > >,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::clutter_registry::ClutterId,tuple$<> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 6 条；闭包环境类型引用 12 条。
// [PDB 内联宿主] country_core::systems::save::required_clutter::glade_uses_external_clutter; module=509, inline_record=37272; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::filter::impl$3::next<core::iter::adapters::flatten::FlatMap<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<std::collections::hash::map::Iter<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_storage::impl$17::iter::closure_env$0>,core::iter::adapters::map::Map<core::option::Iter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_storage::impl$17::iter::closure_env$1> >,core::iter::adapters::map::Map<core::option::Iter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_storage::impl$17::iter::closure_env$2> >,core::iter::adapters::filter_map::FilterMap<std::collections::hash::map::Values<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,country_core::systems::save::required_clutter::external_clutter_in_storage::closure$0::closure_env$0>,country_core::systems::save::required_clutter::external_clutter_in_storage::closure_env$0>,country_core::systems::save::required_clutter::collect_missing_external_ids::closure_env$0>; module=1643, inline_record=76596; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::filter::impl$3::next<core::iter::adapters::flatten::FlatMap<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<std::collections::hash::map::Iter<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_storage::impl$17::iter::closure_env$0>,core::iter::adapters::map::Map<core::option::Iter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_storage::impl$17::iter::closure_env$1> >,core::iter::adapters::map::Map<core::option::Iter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_storage::impl$17::iter::closure_env$2> >,core::iter::adapters::filter_map::FilterMap<std::collections::hash::map::Values<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,country_core::systems::save::required_clutter::external_clutter_in_storage::closure$0::closure_env$0>,country_core::systems::save::required_clutter::external_clutter_in_storage::closure_env$0>,country_core::systems::save::required_clutter::collect_missing_external_ids::closure_env$0>; module=1643, inline_record=78920; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::try_fold<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,ref$<country_core::resources::walls::decorator_storage::Decorators> >,std::collections::hash::map::Iter<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_storage::impl$17::iter::closure_env$0,tuple$<>,ref_mut$<ref_mut$<core::iter::adapters::map::map_try_fold::closure_env$0<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,ref$<country_core::resources::walls::decorator_storage::Decorators> >,core::iter::adapters::filter_map::FilterMap<std::collections::hash::map::Values<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,country_core::systems::save::required_clutter::external_clutter_in_storage::closure$0::closure_env$0>,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::clutter_registry::ClutterId,tuple$<> > >,country_core::systems::save::required_clutter::external_clutter_in_storage::closure_env$0,core::iter::adapters::flatten::impl$18::iter_try_fold::flatten::closure_env$0<core::iter::adapters::filter_map::FilterMap<std::collections::hash::map::Values<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,country_core::systems::save::required_clutter::external_clutter_in_storage::closure$0::closure_env$0>,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::clutter_registry::ClutterId,tuple$<> > >,core::iter::adapters::flatten::impl$20::try_fold::flatten::closure_env$0<core::iter::adapters::filter_map::FilterMap<std::collections::hash::map::Values<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,country_core::systems::save::required_clutter::external_clutter_in_storage::closure$0::closure_env$0>,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::clutter_registry::ClutterId,tuple$<> > >,core::iter::traits::iterator::Iterator::find::check::closure_env$0<country_core::resources::clutter_registry::ClutterId,ref_mut$<country_core::systems::save::required_clutter::collect_missing_external_ids::closure_env$0> > > > > > >,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::clutter_registry::ClutterId,tuple$<> > > >; module=1682, inline_record=146948; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::try_fold<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,ref$<country_core::resources::walls::decorator_storage::Decorators> >,core::option::Iter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_storage::impl$17::iter::closure_env$2,tuple$<>,core::iter::adapters::map::map_try_fold::closure_env$0<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,ref$<country_core::resources::walls::decorator_storage::Decorators> >,core::iter::adapters::filter_map::FilterMap<std::collections::hash::map::Values<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,country_core::systems::save::required_clutter::external_clutter_in_storage::closure$0::closure_env$0>,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::clutter_registry::ClutterId,tuple$<> > >,country_core::systems::save::required_clutter::external_clutter_in_storage::closure_env$0,core::iter::adapters::flatten::impl$18::iter_try_fold::flatten::closure_env$0<core::iter::adapters::filter_map::FilterMap<std::collections::hash::map::Values<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,country_core::systems::save::required_clutter::external_clutter_in_storage::closure$0::closure_env$0>,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::clutter_registry::ClutterId,tuple$<> > >,core::iter::adapters::flatten::impl$20::try_fold::flatten::closure_env$0<core::iter::adapters::filter_map::FilterMap<std::collections::hash::map::Values<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,country_core::systems::save::required_clutter::external_clutter_in_storage::closure$0::closure_env$0>,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::clutter_registry::ClutterId,tuple$<> > >,core::iter::traits::iterator::Iterator::find::check::closure_env$0<country_core::resources::clutter_registry::ClutterId,ref_mut$<country_core::systems::save::required_clutter::collect_missing_external_ids::closure_env$0> > > > >,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::clutter_registry::ClutterId,tuple$<> > > >; module=1682, inline_record=152420; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::try_fold<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,ref$<country_core::resources::walls::decorator_storage::Decorators> >,core::option::Iter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_storage::impl$17::iter::closure_env$1,tuple$<>,ref_mut$<core::iter::adapters::map::map_try_fold::closure_env$0<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,ref$<country_core::resources::walls::decorator_storage::Decorators> >,core::iter::adapters::filter_map::FilterMap<std::collections::hash::map::Values<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,country_core::systems::save::required_clutter::external_clutter_in_storage::closure$0::closure_env$0>,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::clutter_registry::ClutterId,tuple$<> > >,country_core::systems::save::required_clutter::external_clutter_in_storage::closure_env$0,core::iter::adapters::flatten::impl$18::iter_try_fold::flatten::closure_env$0<core::iter::adapters::filter_map::FilterMap<std::collections::hash::map::Values<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,country_core::systems::save::required_clutter::external_clutter_in_storage::closure$0::closure_env$0>,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::clutter_registry::ClutterId,tuple$<> > >,core::iter::adapters::flatten::impl$20::try_fold::flatten::closure_env$0<core::iter::adapters::filter_map::FilterMap<std::collections::hash::map::Values<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,country_core::systems::save::required_clutter::external_clutter_in_storage::closure$0::closure_env$0>,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::clutter_registry::ClutterId,tuple$<> > >,core::iter::traits::iterator::Iterator::find::check::closure_env$0<country_core::resources::clutter_registry::ClutterId,ref_mut$<country_core::systems::save::required_clutter::collect_missing_external_ids::closure_env$0> > > > > >,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::clutter_registry::ClutterId,tuple$<> > > >; module=1682, inline_record=172212; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::required_clutter::glade_uses_external_clutter
// 信息 ID: 3903822a6c693c3c8244ff1e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::render_loop::draw_passes::logo_overlay::render_logo_overlay @ 0xa0a8d7 -> 0x92a6e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::required_clutter::warn_missing_mods_on_load
// 信息 ID: 131232d422b04a9f9265a4fb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 560680baaf47f31f15cc30e5: void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::missing_mods::ShowMissingModsPopupCmd>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::missing_mods::ShowMissingModsPopupCmd>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::missing_mods::ShowMissingModsPopupCmd>)> @ 0x17eeaf1 -> 0x929f30 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::missing_mods::ShowMissingModsPopupCmd>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::missing_mods::ShowMissingModsPopupCmd>)> > @ 0x1916b90 -> 0x929f30 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::required_clutter::warn_missing_mods_on_load::closure$0
// 信息 ID: 19c6941c581fa033e8399e50
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::save::required_clutter::warn_missing_mods_on_load", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::save::required_clutter::warn_missing_mods_on_load::closure_env$0 <- alloc::collections::btree::set::impl$15::from_iter<ref$<str$>,core::iter::adapters::map::Map<alloc::collections::btree::set::Iter<country_core::resources::clutter_registry::ClutterId>,country_core::systems::save::required_clutter::warn_missing_mods_on_load::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::save::required_clutter::warn_missing_mods_on_load::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<ref$<str$>,core::iter::adapters::map::Map<alloc::collections::btree::set::Iter<country_core::resources::clutter_registry::ClutterId>,country_core::systems::save::required_clutter::warn_missing_mods_on_load::closure_env$0> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<ref$<str$>,core::iter::adapters::map::Map<alloc::collections::btree::set::Iter<country_core::resources::clutter_registry::ClutterId>,country_core::systems::save::required_clutter::warn_missing_mods_on_load::closure_env$0> >; module=1643, inline_record=626908; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===

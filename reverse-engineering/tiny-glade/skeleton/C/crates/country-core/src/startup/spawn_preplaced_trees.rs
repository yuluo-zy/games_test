// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/country-core/src/startup/spawn_preplaced_trees.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 45d8f9fc290e522ef0717c9d9aa54e3a5834a7200d3af6a258ff07321f6b7ba3
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: spawn_preplaced_trees.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 579
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use anyhow; // namespace candidate
//   package identities: [{"name":"anyhow","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.100"}]
// use asset; // namespace candidate
//   package identities: [{"name":"asset","source":"local_or_path","version":"0.1.0"}]
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.30.10"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.27"}]
// use std; // namespace candidate
//   package identities: []
// use tg_mods; // namespace candidate
//   package identities: [{"name":"tg-mods","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::startup::spawn_preplaced_trees::spawn_preplaced_trees
// 记录形式: compiled_procedure; instances=1
// 源码行: 25
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":579,"record_offset":12908,"rva":12268176,"va":5380977296}
// 编译实例 RVAs: 0xbb3290
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::path::Path>::_join
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t3678478","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380977296,"raw_hex":"e80e213800","size":5,"soft_fail":false,"target_va":5384656240,"va":5380977757,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3631718","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380977296,"raw_hex":"e8666a3700","size":5,"soft_fail":false,"target_va":5384609408,"va":5380977685,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=18
//     evidence={"assembly":"\tcallq\t-5299239","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380977296,"raw_hex":"e8d923afff","size":5,"soft_fail":false,"target_va":5375678704,"va":5380977938,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14009ae10
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: spawn_preplaced_trees.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11638919","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380977296,"raw_hex":"e879674eff","size":5,"soft_fail":false,"target_va":5369343504,"va":5380982418,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140972520
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: spawn_preplaced_trees.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2365688","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380977296,"raw_hex":"e808e7dbff","size":5,"soft_fail":false,"target_va":5378614560,"va":5380980243,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14099c6a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: spawn_preplaced_trees.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2195642","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380977296,"raw_hex":"e8467fdeff","size":5,"soft_fail":false,"target_va":5378786976,"va":5380982613,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409e61f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: spawn_preplaced_trees.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1889882","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380977296,"raw_hex":"e8a629e3ff","size":5,"soft_fail":false,"target_va":5379088880,"va":5380978757,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b992e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: spawn_preplaced_trees.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-106948","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380977296,"raw_hex":"e83c5efeff","size":5,"soft_fail":false,"target_va":5380870880,"va":5380977823,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411af090
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: spawn_preplaced_trees.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6273017","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380977296,"raw_hex":"e8f9b75f00","size":5,"soft_fail":false,"target_va":5387251856,"va":5380978834,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141415eb0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: spawn_preplaced_trees.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8790619","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380977296,"raw_hex":"e85b228600","size":5,"soft_fail":false,"target_va":5389770416,"va":5380979792,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141629070
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: spawn_preplaced_trees.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10966566","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380977296,"raw_hex":"e82656a700","size":5,"soft_fail":false,"target_va":5391945840,"va":5380979269,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141651090
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: spawn_preplaced_trees.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11130792","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380977296,"raw_hex":"e8a8d7a900","size":5,"soft_fail":false,"target_va":5392109712,"va":5380978915,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141651af0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: spawn_preplaced_trees.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11134489","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380977296,"raw_hex":"e819e6a900","size":5,"soft_fail":false,"target_va":5392112368,"va":5380977874,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14181c970
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: spawn_preplaced_trees.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13013065","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380977296,"raw_hex":"e84990c600","size":5,"soft_fail":false,"target_va":5393992048,"va":5380978978,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6fd10
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: spawn_preplaced_trees.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t35373161","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380977296,"raw_hex":"e869c01b02","size":5,"soft_fail":false,"target_va":5416353040,"va":5380979874,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t35402233","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380977296,"raw_hex":"e8f9311c02","size":5,"soft_fail":false,"target_va":5416381904,"va":5380979666,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-5030978","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380977296,"raw_hex":"e8be3bb3ff","size":5,"soft_fail":false,"target_va":5375949040,"va":5380980013,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t3643318","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380977296,"raw_hex":"e8b6973700","size":5,"soft_fail":false,"target_va":5384624080,"va":5380980757,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::entity::Entities::reserve_entity
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t3247085","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380977296,"raw_hex":"e8ed8b3100","size":5,"soft_fail":false,"target_va":5384226192,"va":5380979102,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t35641783","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380977296,"raw_hex":"e8b7d91f02","size":5,"soft_fail":false,"target_va":5416622208,"va":5380980420,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t35643370","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380977296,"raw_hex":"e8eadf1f02","size":5,"soft_fail":false,"target_va":5416621632,"va":5380978257,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::startup::spawn_preplaced_trees::spawn_preplaced_trees::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t35410792","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380977296,"raw_hex":"e868531c02","size":5,"soft_fail":false,"target_va":5416391232,"va":5380980435,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::read_to_string::inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t3772444","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380977296,"raw_hex":"e81c903900","size":5,"soft_fail":false,"target_va":5384751264,"va":5380978815,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_mods::overlay::resolve
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1940865","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380977296,"raw_hex":"e8819d1d00","size":5,"soft_fail":false,"target_va":5382919648,"va":5380978778,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::string::impl$93::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":579,"record_offset":15312,"parent_record_offset":12908,"ancestor_record_offset":12908,"inlinee":21307,"source_line":2666,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":579,"record_offset":15072,"parent_record_offset":12908,"ancestor_record_offset":12908,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":579,"record_offset":15128,"parent_record_offset":12908,"ancestor_record_offset":12908,"inlinee":5075,"source_line":213,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_direct] asset::prefab::impl$8::clone
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/prefab.rs
//     evidence={"module":579,"record_offset":18004,"parent_record_offset":12908,"ancestor_record_offset":12908,"inlinee":28802,"source_line":10,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":579,"record_offset":19808,"parent_record_offset":12908,"ancestor_record_offset":12908,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::iterators::impl$0::last
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":579,"record_offset":13720,"parent_record_offset":12908,"ancestor_record_offset":12908,"inlinee":13153,"source_line":26,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::message_reader::MessageReader::read
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_reader.rs
//     evidence={"module":579,"record_offset":13156,"parent_record_offset":12908,"ancestor_record_offset":12908,"inlinee":9339,"source_line":27,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::message_writer::MessageWriter::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_writer.rs
//     evidence={"module":579,"record_offset":19180,"parent_record_offset":12908,"ancestor_record_offset":12908,"inlinee":11206,"source_line":69,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::commands::Commands::spawn
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":579,"record_offset":17696,"parent_record_offset":12908,"ancestor_record_offset":12908,"inlinee":11391,"source_line":380,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::commands::EntityCommands::insert
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":579,"record_offset":18120,"parent_record_offset":12908,"ancestor_record_offset":12908,"inlinee":11388,"source_line":1390,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":579,"record_offset":13996,"parent_record_offset":12908,"ancestor_record_offset":12908,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::abs
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":579,"record_offset":17668,"parent_record_offset":12908,"ancestor_record_offset":12908,"inlinee":11731,"source_line":1569,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":579,"record_offset":14420,"parent_record_offset":12908,"ancestor_record_offset":12908,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::expect
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":579,"record_offset":14304,"parent_record_offset":12908,"ancestor_record_offset":12908,"inlinee":6800,"source_line":1179,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":579,"record_offset":17448,"parent_record_offset":12908,"ancestor_record_offset":12908,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":579,"record_offset":17408,"parent_record_offset":12908,"ancestor_record_offset":12908,"inlinee":6444,"source_line":1616,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":579,"record_offset":17492,"parent_record_offset":12908,"ancestor_record_offset":12908,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":579,"record_offset":17576,"parent_record_offset":12908,"ancestor_record_offset":12908,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::session::ActiveSession::assets_root
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":579,"record_offset":14164,"parent_record_offset":12908,"ancestor_record_offset":12908,"inlinee":8070,"source_line":291,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::tree_storage::TreeStorage::insert
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/tree_storage.rs
//     evidence={"module":579,"record_offset":19864,"parent_record_offset":12908,"ancestor_record_offset":12908,"inlinee":36868,"source_line":84,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":579,"record_offset":18712,"parent_record_offset":12908,"ancestor_record_offset":12908,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":579,"record_offset":14052,"parent_record_offset":12908,"ancestor_record_offset":12908,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":579,"record_offset":13912,"parent_record_offset":12908,"ancestor_record_offset":12908,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":579,"record_offset":14276,"parent_record_offset":12908,"ancestor_record_offset":12908,"inlinee":7504,"source_line":3010,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_direct] tg_mods::overlay::read_to_string
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-mods/src/overlay.rs
//     evidence={"module":579,"record_offset":16544,"parent_record_offset":12908,"ancestor_record_offset":12908,"inlinee":22155,"source_line":688,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":579,"record_offset":14596,"parent_record_offset":14572,"ancestor_record_offset":12908,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":579,"record_offset":14620,"parent_record_offset":14596,"ancestor_record_offset":12908,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":579,"record_offset":14644,"parent_record_offset":14620,"ancestor_record_offset":12908,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":579,"record_offset":14572,"parent_record_offset":14516,"ancestor_record_offset":12908,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":579,"record_offset":19436,"parent_record_offset":19404,"ancestor_record_offset":12908,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":579,"record_offset":17852,"parent_record_offset":17824,"ancestor_record_offset":12908,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":579,"record_offset":17936,"parent_record_offset":17912,"ancestor_record_offset":12908,"inlinee":4788,"source_line":616,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":579,"record_offset":14544,"parent_record_offset":14516,"ancestor_record_offset":12908,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":579,"record_offset":14516,"parent_record_offset":14492,"ancestor_record_offset":12908,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":579,"record_offset":17912,"parent_record_offset":17880,"ancestor_record_offset":12908,"inlinee":5068,"source_line":764,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":579,"record_offset":19484,"parent_record_offset":19460,"ancestor_record_offset":12908,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":579,"record_offset":19460,"parent_record_offset":19436,"ancestor_record_offset":12908,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":579,"record_offset":17880,"parent_record_offset":17852,"ancestor_record_offset":12908,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":579,"record_offset":14492,"parent_record_offset":14468,"ancestor_record_offset":12908,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":579,"record_offset":20096,"parent_record_offset":20060,"ancestor_record_offset":12908,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$32::clone
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":579,"record_offset":18028,"parent_record_offset":18004,"ancestor_record_offset":12908,"inlinee":5906,"source_line":2376,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":579,"record_offset":19648,"parent_record_offset":19624,"ancestor_record_offset":12908,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":579,"record_offset":19404,"parent_record_offset":19356,"ancestor_record_offset":12908,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":579,"record_offset":13392,"parent_record_offset":13368,"ancestor_record_offset":12908,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":579,"record_offset":19324,"parent_record_offset":19280,"ancestor_record_offset":12908,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":579,"record_offset":19356,"parent_record_offset":19324,"ancestor_record_offset":12908,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":579,"record_offset":17824,"parent_record_offset":17792,"ancestor_record_offset":12908,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":579,"record_offset":13368,"parent_record_offset":13268,"ancestor_record_offset":12908,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":579,"record_offset":19252,"parent_record_offset":19212,"ancestor_record_offset":12908,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":579,"record_offset":19212,"parent_record_offset":19180,"ancestor_record_offset":12908,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::iterators::MessageIteratorWithId::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":579,"record_offset":13268,"parent_record_offset":13236,"ancestor_record_offset":12908,"inlinee":9344,"source_line":54,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::iterators::impl$3::last
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":579,"record_offset":13752,"parent_record_offset":13720,"ancestor_record_offset":12908,"inlinee":10306,"source_line":112,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::message_cursor::MessageCursor::read_with_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_cursor.rs
//     evidence={"module":579,"record_offset":13236,"parent_record_offset":13188,"ancestor_record_offset":12908,"inlinee":9342,"source_line":89,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::messages::Messages::write_with_caller
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs
//     evidence={"module":579,"record_offset":19280,"parent_record_offset":19212,"ancestor_record_offset":12908,"inlinee":8681,"source_line":129,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":579,"record_offset":17764,"parent_record_offset":17736,"ancestor_record_offset":12908,"inlinee":8582,"source_line":588,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue_internal
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":579,"record_offset":17792,"parent_record_offset":17764,"ancestor_record_offset":12908,"inlinee":8583,"source_line":652,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::EntityCommands::queue
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs
//     evidence={"module":579,"record_offset":17736,"parent_record_offset":17696,"ancestor_record_offset":12908,"inlinee":11368,"source_line":1889,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$14::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/system_param.rs
//     evidence={"module":579,"record_offset":13188,"parent_record_offset":13156,"ancestor_record_offset":12908,"inlinee":9614,"source_line":1064,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::max
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":579,"record_offset":19952,"parent_record_offset":19864,"ancestor_record_offset":12908,"inlinee":5501,"source_line":1033,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":579,"record_offset":15368,"parent_record_offset":15340,"ancestor_record_offset":12908,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":579,"record_offset":16584,"parent_record_offset":16544,"ancestor_record_offset":12908,"inlinee":5107,"source_line":717,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::last
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":579,"record_offset":13788,"parent_record_offset":13752,"ancestor_record_offset":12908,"inlinee":10307,"source_line":174,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":579,"record_offset":13496,"parent_record_offset":13472,"ancestor_record_offset":12908,"inlinee":7937,"source_line":917,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":579,"record_offset":13312,"parent_record_offset":13268,"ancestor_record_offset":12908,"inlinee":9345,"source_line":2344,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":579,"record_offset":20356,"parent_record_offset":20328,"ancestor_record_offset":12908,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":579,"record_offset":18736,"parent_record_offset":18712,"ancestor_record_offset":12908,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":579,"record_offset":13816,"parent_record_offset":13788,"ancestor_record_offset":12908,"inlinee":5449,"source_line":1541,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":579,"record_offset":14020,"parent_record_offset":13996,"ancestor_record_offset":12908,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":579,"record_offset":19980,"parent_record_offset":19864,"ancestor_record_offset":12908,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$47::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":579,"record_offset":13876,"parent_record_offset":13752,"ancestor_record_offset":12908,"inlinee":5260,"source_line":2765,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=73
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":579,"record_offset":14344,"parent_record_offset":14304,"ancestor_record_offset":12908,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":579,"record_offset":15096,"parent_record_offset":15072,"ancestor_record_offset":12908,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":579,"record_offset":15224,"parent_record_offset":15128,"ancestor_record_offset":12908,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":579,"record_offset":20128,"parent_record_offset":20096,"ancestor_record_offset":12908,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":579,"record_offset":15252,"parent_record_offset":15128,"ancestor_record_offset":12908,"inlinee":5076,"source_line":985,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":579,"record_offset":15180,"parent_record_offset":15128,"ancestor_record_offset":12908,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":579,"record_offset":15276,"parent_record_offset":15252,"ancestor_record_offset":12908,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":579,"record_offset":19524,"parent_record_offset":19356,"ancestor_record_offset":12908,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":579,"record_offset":15396,"parent_record_offset":15368,"ancestor_record_offset":12908,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":579,"record_offset":15424,"parent_record_offset":15396,"ancestor_record_offset":12908,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":579,"record_offset":13424,"parent_record_offset":13268,"ancestor_record_offset":12908,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":579,"record_offset":13472,"parent_record_offset":13448,"ancestor_record_offset":12908,"inlinee":8594,"source_line":366,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":579,"record_offset":13448,"parent_record_offset":13424,"ancestor_record_offset":12908,"inlinee":8593,"source_line":545,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":579,"record_offset":17516,"parent_record_offset":17492,"ancestor_record_offset":12908,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":579,"record_offset":15340,"parent_record_offset":15312,"ancestor_record_offset":12908,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":579,"record_offset":18056,"parent_record_offset":18028,"ancestor_record_offset":12908,"inlinee":5322,"source_line":3134,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":579,"record_offset":19684,"parent_record_offset":19648,"ancestor_record_offset":12908,"inlinee":4732,"source_line":3165,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":579,"record_offset":13936,"parent_record_offset":13912,"ancestor_record_offset":12908,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":579,"record_offset":18080,"parent_record_offset":18056,"ancestor_record_offset":12908,"inlinee":4153,"source_line":3919,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":579,"record_offset":13960,"parent_record_offset":13936,"ancestor_record_offset":12908,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":579,"record_offset":19708,"parent_record_offset":19684,"ancestor_record_offset":12908,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":579,"record_offset":19740,"parent_record_offset":19648,"ancestor_record_offset":12908,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] country_core::components::TreeStorageIndex::next
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/components/mod.rs
//     evidence={"module":579,"record_offset":19924,"parent_record_offset":19864,"ancestor_record_offset":12908,"inlinee":28909,"source_line":65,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::GladeType::assets_folder
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":579,"record_offset":14212,"parent_record_offset":14188,"ancestor_record_offset":12908,"inlinee":27998,"source_line":52,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::SessionMeta::assets_root
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":579,"record_offset":14188,"parent_record_offset":14164,"ancestor_record_offset":12908,"inlinee":27997,"source_line":208,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":579,"record_offset":14100,"parent_record_offset":14076,"ancestor_record_offset":12908,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":579,"record_offset":14076,"parent_record_offset":14052,"ancestor_record_offset":12908,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":579,"record_offset":14124,"parent_record_offset":14100,"ancestor_record_offset":12908,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::insert
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":579,"record_offset":19896,"parent_record_offset":19864,"ancestor_record_offset":12908,"inlinee":5499,"source_line":1295,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::read_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":579,"record_offset":16612,"parent_record_offset":16544,"ancestor_record_offset":12908,"inlinee":8116,"source_line":383,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::join
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":579,"record_offset":14240,"parent_record_offset":14188,"ancestor_record_offset":12908,"inlinee":7504,"source_line":3010,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::spawn_preplaced_trees::spawn_preplaced_trees::closure$0
// 记录形式: compiled_procedure; instances=1
// 源码行: 53
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":579,"record_offset":23944,"rva":47682112,"va":5416391232}
// 编译实例 RVAs: 0x2d79240
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-31767239","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5416391232,"raw_hex":"e839451bfe","size":5,"soft_fail":false,"target_va":5384624080,"va":5416391314,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t230948","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5416391232,"raw_hex":"e824860300","size":5,"soft_fail":false,"target_va":5416622240,"va":5416391287,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":579,"record_offset":24108,"parent_record_offset":23944,"ancestor_record_offset":23944,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00579.jsonl.gz"}
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
// 完整结构及原始类型记录: spawn_preplaced_trees.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: spawn_preplaced_trees.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: spawn_preplaced_trees.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_reader::MessageReader<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::TreeStorage>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::prefab::SpawnPrefabCmd>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>", "bevy_ecs::system::commands::Commands", "bevy_ecs::message::message_reader::MessageReader<country_core::resources::history::tree_edit::SpawnPreplacedTrees>", "bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::TreeStorage>", "bevy_ecs::message::message_writer::MessageWriter<country_core::systems::prefab::SpawnPrefabCmd>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::resources::history::tree_edit::SpawnPreplacedTrees>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::TreeStorage>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::prefab::SpawnPrefabCmd>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands

// ------------------------------------------------------------------
// 函数: country_core::startup::spawn_preplaced_trees::spawn_preplaced_trees
// 信息 ID: b1546d81a79b4e74bf1bde17
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] f6065ed52f5f66d82012e2f1: void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_reader::MessageReader<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::TreeStorage>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::prefab::SpawnPrefabCmd>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_reader::MessageReader<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::TreeStorage>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::prefab::SpawnPrefabCmd>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_reader::MessageReader<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::TreeStorage>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::prefab::SpawnPrefabCmd>)> @ 0x184b975 -> 0xbb3290 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_reader::MessageReader<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::TreeStorage>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::prefab::SpawnPrefabCmd>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_reader::MessageReader<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::TreeStorage>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::prefab::SpawnPrefabCmd>)> > @ 0x19250f6 -> 0xbb3290 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::spawn_preplaced_trees::spawn_preplaced_trees::closure$0
// 信息 ID: 57783327ecdccec7a6da91b1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::startup::spawn_preplaced_trees::spawn_preplaced_trees", "captures": "unknown"}
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::startup::spawn_preplaced_trees::spawn_preplaced_trees @ 0xbb3ed3 -> 0x2d79240 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===

// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/country-core/src/systems/steam/workshop_mods.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 3fa7cd446556332df4e972637869936490d6107ac4508ab429ad4bfdabf783a5
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: workshop_mods.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 559, 1656, 1658, 1682, 1822
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
// use steamworks; // namespace candidate
//   package identities: [{"name":"bitflags","source":"PDB_registry_path_only","version":"1.3.2"},{"name":"steamworks","source":"git+https://github.com/PounceLight/steamworks-rs.git?rev=f441c6128f04669ce471cf7bfd85e70755a9b466#f441c6128f04669ce471cf7bfd85e70755a9b466","version":"0.11.0"}]
// use tg_mods; // namespace candidate
//   package identities: [{"name":"tg-mods","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::steam::workshop_mods::register_workshop_mod
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":559,"record_offset":13660,"rva":11524320,"va":5380233440}
// 编译实例 RVAs: 0xafd8e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <alloc::string::String as core::clone::Clone>::clone
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4561363","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380233440,"raw_hex":"e8d3994500","size":5,"soft_fail":false,"target_va":5384795984,"va":5380234616,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t4375396","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380233440,"raw_hex":"e864c34200","size":5,"soft_fail":false,"target_va":5384609408,"va":5380234007,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-4555954","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380233440,"raw_hex":"e84e7bbaff","size":5,"soft_fail":false,"target_va":5375678688,"va":5380234637,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=11
//     evidence={"assembly":"\tcallq\t-4555703","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380233440,"raw_hex":"e8497cbaff","size":5,"soft_fail":false,"target_va":5375678704,"va":5380234402,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: workshop_mods.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11512435","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380233440,"raw_hex":"e88d5550ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5380234622,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140099f90
//     owner=unresolved_namespace; count=12
//     target_aliases=4 candidates; complete list: workshop_mods.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10895126","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380233440,"raw_hex":"e8eac059ff","size":5,"soft_fail":false,"target_va":5369339792,"va":5380234913,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14040ebd0
//     owner=unresolved_namespace; count=1
//     target_aliases=7 candidates; complete list: workshop_mods.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7269974","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380233440,"raw_hex":"e8aa1191ff","size":5,"soft_fail":false,"target_va":5372963792,"va":5380233761,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048c880
//     owner=unresolved_namespace; count=1
//     target_aliases=6 candidates; complete list: workshop_mods.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6754538","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380233440,"raw_hex":"e816ef98ff","size":5,"soft_fail":false,"target_va":5373479040,"va":5380233573,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140523d10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: workshop_mods.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6137083","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380233440,"raw_hex":"e8055ba2ff","size":5,"soft_fail":false,"target_va":5374098704,"va":5380235782,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14099ffa0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: workshop_mods.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1434824","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380233440,"raw_hex":"e8381beaff","size":5,"soft_fail":false,"target_va":5378801568,"va":5380236387,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ca76c0
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: workshop_mods.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1744187","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380233440,"raw_hex":"e83b9d1a00","size":5,"soft_fail":false,"target_va":5381977792,"va":5380233600,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1417a8ab0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: workshop_mods.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13282693","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380233440,"raw_hex":"e885adca00","size":5,"soft_fail":false,"target_va":5393517232,"va":5380234534,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1b3e0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: workshop_mods.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t36821134","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380233440,"raw_hex":"e88ed83102","size":5,"soft_fail":false,"target_va":5417055200,"va":5380234061,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1b400
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: workshop_mods.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t36821125","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380233440,"raw_hex":"e885d83102","size":5,"soft_fail":false,"target_va":5417055232,"va":5380234102,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: workshop_mods.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t36821855","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380233440,"raw_hex":"e85fdb3102","size":5,"soft_fail":false,"target_va":5417057040,"va":5380235180,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=7
//     target_aliases=1 candidates; complete list: workshop_mods.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t36822590","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380233440,"raw_hex":"e83ede3102","size":5,"soft_fail":false,"target_va":5417057056,"va":5380234461,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t36384811","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380233440,"raw_hex":"e82b302b02","size":5,"soft_fail":false,"target_va":5416620769,"va":5380235953,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1763283","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380233440,"raw_hex":"e8d3e71a00","size":5,"soft_fail":false,"target_va":5381997408,"va":5380234120,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::clutter_packs::WorkshopSidecar::read
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1666217","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380233440,"raw_hex":"e8a96c1900","size":5,"soft_fail":false,"target_va":5381901760,"va":5380235538,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::clutter_packs::verify_pack_checksums
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1680948","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380233440,"raw_hex":"e834a61900","size":5,"soft_fail":false,"target_va":5381914480,"va":5380233527,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls::clear_status
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1007993","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380233440,"raw_hex":"e879610f00","size":5,"soft_fail":false,"target_va":5381242368,"va":5380234370,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls::set_status
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1008291","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380233440,"raw_hex":"e8a3620f00","size":5,"soft_fail":false,"target_va":5381242016,"va":5380233720,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_mods::discovery::mod_at_root
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2693787","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380233440,"raw_hex":"e89b1a2900","size":5,"soft_fail":false,"target_va":5382927456,"va":5380233664,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_mods::overlay::add_mods
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2686841","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380233440,"raw_hex":"e879ff2800","size":5,"soft_fail":false,"target_va":5382921568,"va":5380234722,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_mods::overlay::installed_mods
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2668902","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380233440,"raw_hex":"e866b92800","size":5,"soft_fail":false,"target_va":5382903888,"va":5380234981,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::boxed::box_assume_init_into_vec_unsafe
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":559,"record_offset":16164,"parent_record_offset":13660,"ancestor_record_offset":13660,"inlinee":4459,"source_line":262,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] alloc::boxed::impl$0::new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":559,"record_offset":15980,"parent_record_offset":13660,"ancestor_record_offset":13660,"inlinee":4443,"source_line":311,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":559,"record_offset":16248,"parent_record_offset":13660,"ancestor_record_offset":13660,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":559,"record_offset":16364,"parent_record_offset":13660,"ancestor_record_offset":13660,"inlinee":5075,"source_line":213,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":559,"record_offset":19740,"parent_record_offset":13660,"ancestor_record_offset":13660,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::find
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":559,"record_offset":17624,"parent_record_offset":13660,"ancestor_record_offset":13660,"inlinee":7940,"source_line":2937,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_none
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":559,"record_offset":17448,"parent_record_offset":13660,"ancestor_record_offset":13660,"inlinee":4877,"source_line":684,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":559,"record_offset":14884,"parent_record_offset":13660,"ancestor_record_offset":13660,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls::clear_redownload_attempt
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/steam/workshop_clutter.rs
//     evidence={"module":559,"record_offset":13864,"parent_record_offset":13660,"ancestor_record_offset":13660,"inlinee":34993,"source_line":197,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls::mark_provided
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/steam/workshop_clutter.rs
//     evidence={"module":559,"record_offset":15536,"parent_record_offset":13660,"ancestor_record_offset":13660,"inlinee":34996,"source_line":172,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls::request_redownload_once
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/steam/workshop_clutter.rs
//     evidence={"module":559,"record_offset":14048,"parent_record_offset":13660,"ancestor_record_offset":13660,"inlinee":34994,"source_line":205,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":559,"record_offset":14660,"parent_record_offset":13660,"ancestor_record_offset":13660,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":559,"record_offset":14508,"parent_record_offset":13660,"ancestor_record_offset":13660,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] tg_mods::identity::impl$5::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-mods/src/identity.rs
//     evidence={"module":559,"record_offset":15952,"parent_record_offset":13660,"ancestor_record_offset":13660,"inlinee":29159,"source_line":47,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":559,"record_offset":16064,"parent_record_offset":16040,"ancestor_record_offset":13660,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":559,"record_offset":16088,"parent_record_offset":16064,"ancestor_record_offset":13660,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":559,"record_offset":15060,"parent_record_offset":15036,"ancestor_record_offset":13660,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":559,"record_offset":15084,"parent_record_offset":15060,"ancestor_record_offset":13660,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":559,"record_offset":16112,"parent_record_offset":16088,"ancestor_record_offset":13660,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":559,"record_offset":15108,"parent_record_offset":15084,"ancestor_record_offset":13660,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":559,"record_offset":16040,"parent_record_offset":16008,"ancestor_record_offset":13660,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":559,"record_offset":15036,"parent_record_offset":14980,"ancestor_record_offset":13660,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":559,"record_offset":16008,"parent_record_offset":15980,"ancestor_record_offset":13660,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":559,"record_offset":15008,"parent_record_offset":14980,"ancestor_record_offset":13660,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":559,"record_offset":14980,"parent_record_offset":14956,"ancestor_record_offset":13660,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":559,"record_offset":14956,"parent_record_offset":14932,"ancestor_record_offset":13660,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":559,"record_offset":16188,"parent_record_offset":16164,"ancestor_record_offset":13660,"inlinee":4460,"source_line":480,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$81::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":559,"record_offset":17944,"parent_record_offset":17916,"ancestor_record_offset":13660,"inlinee":5490,"source_line":350,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":559,"record_offset":14196,"parent_record_offset":14168,"ancestor_record_offset":13660,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$32::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":559,"record_offset":14168,"parent_record_offset":14140,"ancestor_record_offset":13660,"inlinee":5906,"source_line":2376,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":559,"record_offset":14372,"parent_record_offset":14348,"ancestor_record_offset":13660,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":559,"record_offset":16212,"parent_record_offset":16188,"ancestor_record_offset":13660,"inlinee":4461,"source_line":1181,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::IntoIter::as_raw_mut_slice
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":559,"record_offset":16580,"parent_record_offset":16544,"ancestor_record_offset":13660,"inlinee":4695,"source_line":117,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":559,"record_offset":16544,"parent_record_offset":16516,"ancestor_record_offset":13660,"inlinee":4693,"source_line":497,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop::impl$0::drop
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":559,"record_offset":16832,"parent_record_offset":16808,"ancestor_record_offset":13660,"inlinee":4706,"source_line":501,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$6::size_hint
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":559,"record_offset":16652,"parent_record_offset":16628,"ancestor_record_offset":13660,"inlinee":4701,"source_line":234,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$6::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":559,"record_offset":17660,"parent_record_offset":17624,"ancestor_record_offset":13660,"inlinee":5518,"source_line":334,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::partial_eq::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\partial_eq.rs
//     evidence={"module":559,"record_offset":17972,"parent_record_offset":17944,"ancestor_record_offset":13660,"inlinee":5492,"source_line":15,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":559,"record_offset":17708,"parent_record_offset":17660,"ancestor_record_offset":13660,"inlinee":4487,"source_line":264,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::ExactSizeIterator::len
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\exact_size.rs
//     evidence={"module":559,"record_offset":16628,"parent_record_offset":16604,"ancestor_record_offset":13660,"inlinee":4699,"source_line":116,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::impl$0::len
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\exact_size.rs
//     evidence={"module":559,"record_offset":16604,"parent_record_offset":16580,"ancestor_record_offset":13660,"inlinee":4697,"source_line":155,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find::check::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":559,"record_offset":17856,"parent_record_offset":17660,"ancestor_record_offset":13660,"inlinee":5484,"source_line":2944,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\manually_drop.rs
//     evidence={"module":559,"record_offset":16276,"parent_record_offset":16248,"ancestor_record_offset":13660,"inlinee":5072,"source_line":184,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_dangling::MaybeDangling::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_dangling.rs
//     evidence={"module":559,"record_offset":16300,"parent_record_offset":16276,"ancestor_record_offset":13660,"inlinee":5074,"source_line":80,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":559,"record_offset":15780,"parent_record_offset":15748,"ancestor_record_offset":13660,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::control_flow::ControlFlow::break_value
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\control_flow.rs
//     evidence={"module":559,"record_offset":18084,"parent_record_offset":17624,"ancestor_record_offset":13660,"inlinee":11015,"source_line":190,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":559,"record_offset":17472,"parent_record_offset":17448,"ancestor_record_offset":13660,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":559,"record_offset":14624,"parent_record_offset":14508,"ancestor_record_offset":13660,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":559,"record_offset":16724,"parent_record_offset":16700,"ancestor_record_offset":13660,"inlinee":4704,"source_line":701,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=43
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":559,"record_offset":14324,"parent_record_offset":14048,"ancestor_record_offset":13660,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":559,"record_offset":16332,"parent_record_offset":16248,"ancestor_record_offset":13660,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":559,"record_offset":16700,"parent_record_offset":16676,"ancestor_record_offset":13660,"inlinee":4703,"source_line":882,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":559,"record_offset":16428,"parent_record_offset":16364,"ancestor_record_offset":13660,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":559,"record_offset":14228,"parent_record_offset":14196,"ancestor_record_offset":13660,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset_from_unsigned
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":559,"record_offset":16676,"parent_record_offset":16652,"ancestor_record_offset":13660,"inlinee":4702,"source_line":948,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":559,"record_offset":16456,"parent_record_offset":16364,"ancestor_record_offset":13660,"inlinee":5076,"source_line":985,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":559,"record_offset":16400,"parent_record_offset":16364,"ancestor_record_offset":13660,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":559,"record_offset":16480,"parent_record_offset":16456,"ancestor_record_offset":13660,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":559,"record_offset":18000,"parent_record_offset":17972,"ancestor_record_offset":13660,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":559,"record_offset":18028,"parent_record_offset":18000,"ancestor_record_offset":13660,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":559,"record_offset":14408,"parent_record_offset":14372,"ancestor_record_offset":13660,"inlinee":4732,"source_line":3165,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":559,"record_offset":14560,"parent_record_offset":14508,"ancestor_record_offset":13660,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":559,"record_offset":14256,"parent_record_offset":14196,"ancestor_record_offset":13660,"inlinee":4153,"source_line":3919,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":559,"record_offset":14588,"parent_record_offset":14560,"ancestor_record_offset":13660,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":559,"record_offset":14432,"parent_record_offset":14408,"ancestor_record_offset":13660,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":559,"record_offset":14464,"parent_record_offset":14372,"ancestor_record_offset":13660,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::steam::workshop_mods::register_workshop_mod::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/steam/workshop_mods.rs
//     evidence={"module":559,"record_offset":17892,"parent_record_offset":17856,"ancestor_record_offset":13660,"inlinee":34998,"source_line":146,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::remove
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":559,"record_offset":13944,"parent_record_offset":13916,"ancestor_record_offset":13660,"inlinee":4348,"source_line":2005,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::remove_entry
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":559,"record_offset":13972,"parent_record_offset":13944,"ancestor_record_offset":13660,"inlinee":4349,"source_line":2044,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":559,"record_offset":14000,"parent_record_offset":13972,"ancestor_record_offset":13660,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::insert
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\set.rs
//     evidence={"module":559,"record_offset":14108,"parent_record_offset":14084,"ancestor_record_offset":13660,"inlinee":5647,"source_line":1092,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::remove
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\set.rs
//     evidence={"module":559,"record_offset":13916,"parent_record_offset":13888,"ancestor_record_offset":13660,"inlinee":13817,"source_line":1175,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":559,"record_offset":14708,"parent_record_offset":14684,"ancestor_record_offset":13660,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":559,"record_offset":14684,"parent_record_offset":14660,"ancestor_record_offset":13660,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":559,"record_offset":14732,"parent_record_offset":14708,"ancestor_record_offset":13660,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::insert
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":559,"record_offset":15572,"parent_record_offset":15536,"ancestor_record_offset":13660,"inlinee":5499,"source_line":1295,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::HashSet::insert
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":559,"record_offset":14084,"parent_record_offset":14048,"ancestor_record_offset":13660,"inlinee":5646,"source_line":995,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::HashSet::remove
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":559,"record_offset":13888,"parent_record_offset":13864,"ancestor_record_offset":13660,"inlinee":13816,"source_line":1042,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] steamworks::Client::ugc
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"steamworks","version":"0.11.0","source":"git+https://github.com/PounceLight/steamworks-rs.git?rev=f441c6128f04669ce471cf7bfd85e70755a9b466#f441c6128f04669ce471cf7bfd85e70755a9b466"}]
//     definition_source=/home/h3/.cargo/git/checkouts/steamworks-rs-f13bba748a20c02a/f441c61/src/lib.rs
//     evidence={"module":559,"record_offset":14140,"parent_record_offset":14048,"ancestor_record_offset":13660,"inlinee":31762,"source_line":454,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] steamworks::ugc::UGC::download_item
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"steamworks","version":"0.11.0","source":"git+https://github.com/PounceLight/steamworks-rs.git?rev=f441c6128f04669ce471cf7bfd85e70755a9b466#f441c6128f04669ce471cf7bfd85e70755a9b466"}]
//     definition_source=/home/h3/.cargo/git/checkouts/steamworks-rs-f13bba748a20c02a/f441c61/src/ugc.rs
//     evidence={"module":559,"record_offset":14296,"parent_record_offset":14048,"ancestor_record_offset":13660,"inlinee":34995,"source_line":704,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] tg_mods::identity::impl$11::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-mods/src/identity.rs
//     evidence={"module":559,"record_offset":17916,"parent_record_offset":17892,"ancestor_record_offset":13660,"inlinee":35000,"source_line":47,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::steam::workshop_mods::register_subscribed_workshop_mods
// 记录形式: compiled_procedure; instances=1
// 源码行: 34
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":559,"record_offset":21608,"rva":11528048,"va":5380237168}
// 编译实例 RVAs: 0xafe770
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::hash::random::RandomState>::new::KEYS::{K#0}::{closure#1}::__RUST_STD_INTERNAL_VAL::{shim:tls#0}
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4407611","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380237168,"raw_hex":"e83b414300","size":5,"soft_fail":false,"target_va":5384644912,"va":5380237296,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t4370809","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380237168,"raw_hex":"e879b14200","size":5,"soft_fail":false,"target_va":5384609408,"va":5380238594,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc_zeroed
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-4559193","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380237168,"raw_hex":"e8a76ebaff","size":5,"soft_fail":false,"target_va":5375678736,"va":5380237924,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=6
//     evidence={"assembly":"\tcallq\t-4558904","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380237168,"raw_hex":"e8c86fbaff","size":5,"soft_fail":false,"target_va":5375678704,"va":5380237603,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: workshop_mods.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11515724","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380237168,"raw_hex":"e8b44850ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5380237911,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14007db70
//     owner=unresolved_namespace; count=2
//     target_aliases=27 candidates; complete list: workshop_mods.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11015467","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380237168,"raw_hex":"e8d5ea57ff","size":5,"soft_fail":false,"target_va":5369224048,"va":5380239510,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140099f90
//     owner=unresolved_namespace; count=3
//     target_aliases=4 candidates; complete list: workshop_mods.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10897766","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380237168,"raw_hex":"e89ab659ff","size":5,"soft_fail":false,"target_va":5369339792,"va":5380237553,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400b05c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: workshop_mods.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10807950","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380237168,"raw_hex":"e872155bff","size":5,"soft_fail":false,"target_va":5369431488,"va":5380239433,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14040ebd0
//     owner=unresolved_namespace; count=1
//     target_aliases=7 candidates; complete list: workshop_mods.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7273645","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380237168,"raw_hex":"e8530391ff","size":5,"soft_fail":false,"target_va":5372963792,"va":5380237432,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048c880
//     owner=unresolved_namespace; count=1
//     target_aliases=6 candidates; complete list: workshop_mods.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6759189","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380237168,"raw_hex":"e8ebdc98ff","size":5,"soft_fail":false,"target_va":5373479040,"va":5380238224,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1417a8ab0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: workshop_mods.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13279437","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380237168,"raw_hex":"e8cda0ca00","size":5,"soft_fail":false,"target_va":5393517232,"va":5380237790,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141c240a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: workshop_mods.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t17978062","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380237168,"raw_hex":"e8ce521201","size":5,"soft_fail":false,"target_va":5398216864,"va":5380238797,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1b3e0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: workshop_mods.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t36817347","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380237168,"raw_hex":"e8c3c93102","size":5,"soft_fail":false,"target_va":5417055200,"va":5380237848,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1b400
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: workshop_mods.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t36816807","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380237168,"raw_hex":"e8a7c73102","size":5,"soft_fail":false,"target_va":5417055232,"va":5380238420,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1b410
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: workshop_mods.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t36817350","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380237168,"raw_hex":"e8c6c93102","size":5,"soft_fail":false,"target_va":5417055248,"va":5380237893,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1b420
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: workshop_mods.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t36817294","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380237168,"raw_hex":"e88ec93102","size":5,"soft_fail":false,"target_va":5417055264,"va":5380237965,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1b430
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: workshop_mods.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t36816881","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380237168,"raw_hex":"e8f1c73102","size":5,"soft_fail":false,"target_va":5417055280,"va":5380238394,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t36381819","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380237168,"raw_hex":"e87b242b02","size":5,"soft_fail":false,"target_va":5416620794,"va":5380238970,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t1759352","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380237168,"raw_hex":"e878d81a00","size":5,"soft_fail":false,"target_va":5381997408,"va":5380238051,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::clutter_packs::WorkshopSidecar::read
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1664338","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380237168,"raw_hex":"e852651900","size":5,"soft_fail":false,"target_va":5381901760,"va":5380237417,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::steam::workshop_mods::register_workshop_mod
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-5475","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380237168,"raw_hex":"e89deaffff","size":5,"soft_fail":false,"target_va":5380233440,"va":5380238910,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::random::hashmap_random_keys
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4487743","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380237168,"raw_hex":"e83f7a4400","size":5,"soft_fail":false,"target_va":5384726688,"va":5380238940,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_mods::overlay::installed_mods
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2666606","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380237168,"raw_hex":"e86eb02800","size":5,"soft_fail":false,"target_va":5382903888,"va":5380237277,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":559,"record_offset":25772,"parent_record_offset":21608,"ancestor_record_offset":21608,"inlinee":5075,"source_line":213,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":559,"record_offset":24580,"parent_record_offset":21608,"ancestor_record_offset":21608,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":559,"record_offset":28076,"parent_record_offset":21608,"ancestor_record_offset":21608,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":559,"record_offset":28496,"parent_record_offset":21608,"ancestor_record_offset":21608,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":559,"record_offset":25896,"parent_record_offset":21608,"ancestor_record_offset":21608,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls::mark_provided
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/steam/workshop_clutter.rs
//     evidence={"module":559,"record_offset":24636,"parent_record_offset":21608,"ancestor_record_offset":21608,"inlinee":34996,"source_line":172,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::steam::workshop_mods::enumerate_locally_overridden_items
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/steam/workshop_mods.rs
//     evidence={"module":559,"record_offset":21836,"parent_record_offset":21608,"ancestor_record_offset":21608,"inlinee":35002,"source_line":165,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":559,"record_offset":28132,"parent_record_offset":21608,"ancestor_record_offset":21608,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":559,"record_offset":27992,"parent_record_offset":21608,"ancestor_record_offset":21608,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::HashSet::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":559,"record_offset":26940,"parent_record_offset":21608,"ancestor_record_offset":21608,"inlinee":5587,"source_line":767,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::impl$20::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":559,"record_offset":23724,"parent_record_offset":21608,"ancestor_record_offset":21608,"inlinee":27428,"source_line":1627,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::impl$23::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":559,"record_offset":24072,"parent_record_offset":21608,"ancestor_record_offset":21608,"inlinee":16842,"source_line":1676,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] steamworks::Client::ugc
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"steamworks","version":"0.11.0","source":"git+https://github.com/PounceLight/steamworks-rs.git?rev=f441c6128f04669ce471cf7bfd85e70755a9b466#f441c6128f04669ce471cf7bfd85e70755a9b466"}]
//     definition_source=/home/h3/.cargo/git/checkouts/steamworks-rs-f13bba748a20c02a/f441c61/src/lib.rs
//     evidence={"module":559,"record_offset":25048,"parent_record_offset":21608,"ancestor_record_offset":21608,"inlinee":31762,"source_line":454,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] steamworks::ugc::ItemState::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":559,"record_offset":27936,"parent_record_offset":21608,"ancestor_record_offset":21608,"inlinee":35010,"source_line":603,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] steamworks::ugc::UGC::download_item
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"steamworks","version":"0.11.0","source":"git+https://github.com/PounceLight/steamworks-rs.git?rev=f441c6128f04669ce471cf7bfd85e70755a9b466#f441c6128f04669ce471cf7bfd85e70755a9b466"}]
//     definition_source=/home/h3/.cargo/git/checkouts/steamworks-rs-f13bba748a20c02a/f441c61/src/ugc.rs
//     evidence={"module":559,"record_offset":27964,"parent_record_offset":21608,"ancestor_record_offset":21608,"inlinee":34995,"source_line":704,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] steamworks::ugc::UGC::item_state
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"steamworks","version":"0.11.0","source":"git+https://github.com/PounceLight/steamworks-rs.git?rev=f441c6128f04669ce471cf7bfd85e70755a9b466#f441c6128f04669ce471cf7bfd85e70755a9b466"}]
//     definition_source=/home/h3/.cargo/git/checkouts/steamworks-rs-f13bba748a20c02a/f441c61/src/ugc.rs
//     evidence={"module":559,"record_offset":27908,"parent_record_offset":21608,"ancestor_record_offset":21608,"inlinee":35008,"source_line":654,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] steamworks::ugc::UGC::subscribed_items
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"steamworks","version":"0.11.0","source":"git+https://github.com/PounceLight/steamworks-rs.git?rev=f441c6128f04669ce471cf7bfd85e70755a9b466#f441c6128f04669ce471cf7bfd85e70755a9b466"}]
//     definition_source=/home/h3/.cargo/git/checkouts/steamworks-rs-f13bba748a20c02a/f441c61/src/ugc.rs
//     evidence={"module":559,"record_offset":25204,"parent_record_offset":21608,"ancestor_record_offset":21608,"inlinee":35005,"source_line":643,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":559,"record_offset":25604,"parent_record_offset":25580,"ancestor_record_offset":21608,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":559,"record_offset":25628,"parent_record_offset":25604,"ancestor_record_offset":21608,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":559,"record_offset":23356,"parent_record_offset":23332,"ancestor_record_offset":21608,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":559,"record_offset":23380,"parent_record_offset":23356,"ancestor_record_offset":21608,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc_zeroed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":559,"record_offset":25652,"parent_record_offset":25628,"ancestor_record_offset":21608,"inlinee":6459,"source_line":172,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":559,"record_offset":23404,"parent_record_offset":23380,"ancestor_record_offset":21608,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate_zeroed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":559,"record_offset":25580,"parent_record_offset":25420,"ancestor_record_offset":21608,"inlinee":6458,"source_line":434,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":559,"record_offset":23332,"parent_record_offset":23244,"ancestor_record_offset":21608,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_zeroed_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":559,"record_offset":25336,"parent_record_offset":25296,"ancestor_record_offset":21608,"inlinee":8417,"source_line":219,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":559,"record_offset":23272,"parent_record_offset":23244,"ancestor_record_offset":21608,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":559,"record_offset":23244,"parent_record_offset":23220,"ancestor_record_offset":21608,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":559,"record_offset":25420,"parent_record_offset":25376,"ancestor_record_offset":21608,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_zeroed_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":559,"record_offset":25376,"parent_record_offset":25336,"ancestor_record_offset":21608,"inlinee":8418,"source_line":588,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":559,"record_offset":23220,"parent_record_offset":23196,"ancestor_record_offset":21608,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":559,"record_offset":25468,"parent_record_offset":25420,"ancestor_record_offset":21608,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":559,"record_offset":25104,"parent_record_offset":25076,"ancestor_record_offset":21608,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$32::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":559,"record_offset":25076,"parent_record_offset":25048,"ancestor_record_offset":21608,"inlinee":5906,"source_line":2376,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":559,"record_offset":26284,"parent_record_offset":26260,"ancestor_record_offset":21608,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::from_elem
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":559,"record_offset":25256,"parent_record_offset":25204,"ancestor_record_offset":21608,"inlinee":8414,"source_line":3650,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":559,"record_offset":25712,"parent_record_offset":25204,"ancestor_record_offset":21608,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":559,"record_offset":23132,"parent_record_offset":23104,"ancestor_record_offset":21608,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":559,"record_offset":25920,"parent_record_offset":25896,"ancestor_record_offset":21608,"inlinee":4693,"source_line":497,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":559,"record_offset":25968,"parent_record_offset":25944,"ancestor_record_offset":21608,"inlinee":4706,"source_line":501,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_elem::impl$1::from_elem
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_elem.rs
//     evidence={"module":559,"record_offset":25296,"parent_record_offset":25256,"ancestor_record_offset":21608,"inlinee":8416,"source_line":22,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":559,"record_offset":24604,"parent_record_offset":24580,"ancestor_record_offset":21608,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":559,"record_offset":25492,"parent_record_offset":25468,"ancestor_record_offset":21608,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":559,"record_offset":22484,"parent_record_offset":22456,"ancestor_record_offset":21608,"inlinee":5188,"source_line":552,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":559,"record_offset":22384,"parent_record_offset":22180,"ancestor_record_offset":21608,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":559,"record_offset":27664,"parent_record_offset":27640,"ancestor_record_offset":21608,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":559,"record_offset":23980,"parent_record_offset":23956,"ancestor_record_offset":21608,"inlinee":4655,"source_line":1319,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":559,"record_offset":27232,"parent_record_offset":27208,"ancestor_record_offset":21608,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":559,"record_offset":27316,"parent_record_offset":27292,"ancestor_record_offset":21608,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":559,"record_offset":27732,"parent_record_offset":27096,"ancestor_record_offset":21608,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter::filter_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\filter.rs
//     evidence={"module":559,"record_offset":22892,"parent_record_offset":22820,"ancestor_record_offset":21608,"inlinee":11536,"source_line":79,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter::impl$3::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\filter.rs
//     evidence={"module":559,"record_offset":22796,"parent_record_offset":22772,"ancestor_record_offset":21608,"inlinee":11531,"source_line":161,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter_map::filter_map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\filter_map.rs
//     evidence={"module":559,"record_offset":22920,"parent_record_offset":22892,"ancestor_record_offset":21608,"inlinee":17049,"source_line":39,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter_map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\filter_map.rs
//     evidence={"module":559,"record_offset":22772,"parent_record_offset":22748,"ancestor_record_offset":21608,"inlinee":17047,"source_line":142,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":559,"record_offset":22748,"parent_record_offset":22724,"ancestor_record_offset":21608,"inlinee":6631,"source_line":124,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":559,"record_offset":22972,"parent_record_offset":22920,"ancestor_record_offset":21608,"inlinee":5396,"source_line":88,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":559,"record_offset":21888,"parent_record_offset":21836,"ancestor_record_offset":21608,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":559,"record_offset":22724,"parent_record_offset":22700,"ancestor_record_offset":21608,"inlinee":5042,"source_line":835,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":559,"record_offset":22996,"parent_record_offset":22972,"ancestor_record_offset":21608,"inlinee":5057,"source_line":842,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":559,"record_offset":22348,"parent_record_offset":22324,"ancestor_record_offset":21608,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":559,"record_offset":26720,"parent_record_offset":26692,"ancestor_record_offset":21608,"inlinee":4369,"source_line":761,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":559,"record_offset":25516,"parent_record_offset":25492,"ancestor_record_offset":21608,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":559,"record_offset":25540,"parent_record_offset":25516,"ancestor_record_offset":21608,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":559,"record_offset":23300,"parent_record_offset":23272,"ancestor_record_offset":21608,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":559,"record_offset":22512,"parent_record_offset":22456,"ancestor_record_offset":21608,"inlinee":5710,"source_line":2456,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":559,"record_offset":27464,"parent_record_offset":27436,"ancestor_record_offset":21608,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":559,"record_offset":24320,"parent_record_offset":24296,"ancestor_record_offset":21608,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":559,"record_offset":22092,"parent_record_offset":22056,"ancestor_record_offset":21608,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":559,"record_offset":28100,"parent_record_offset":28076,"ancestor_record_offset":21608,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":559,"record_offset":22212,"parent_record_offset":22180,"ancestor_record_offset":21608,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":559,"record_offset":24012,"parent_record_offset":23928,"ancestor_record_offset":21608,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":559,"record_offset":27256,"parent_record_offset":27232,"ancestor_record_offset":21608,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=17
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":559,"record_offset":23104,"parent_record_offset":21836,"ancestor_record_offset":21608,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":559,"record_offset":25736,"parent_record_offset":25712,"ancestor_record_offset":21608,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::replace
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":559,"record_offset":22300,"parent_record_offset":22180,"ancestor_record_offset":21608,"inlinee":5714,"source_line":1490,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":559,"record_offset":24440,"parent_record_offset":24416,"ancestor_record_offset":21608,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":559,"record_offset":25136,"parent_record_offset":25104,"ancestor_record_offset":21608,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":559,"record_offset":25836,"parent_record_offset":25772,"ancestor_record_offset":21608,"inlinee":5076,"source_line":985,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":559,"record_offset":22864,"parent_record_offset":22820,"ancestor_record_offset":21608,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":559,"record_offset":25860,"parent_record_offset":25836,"ancestor_record_offset":21608,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::replace
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":559,"record_offset":22324,"parent_record_offset":22300,"ancestor_record_offset":21608,"inlinee":5715,"source_line":1542,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":559,"record_offset":22820,"parent_record_offset":22796,"ancestor_record_offset":21608,"inlinee":6636,"source_line":259,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":559,"record_offset":28016,"parent_record_offset":27992,"ancestor_record_offset":21608,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":559,"record_offset":25164,"parent_record_offset":25104,"ancestor_record_offset":21608,"inlinee":4153,"source_line":3919,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":559,"record_offset":28040,"parent_record_offset":28016,"ancestor_record_offset":21608,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":559,"record_offset":26380,"parent_record_offset":26320,"ancestor_record_offset":21608,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":559,"record_offset":26412,"parent_record_offset":26284,"ancestor_record_offset":21608,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::clutter_packs::impl$12::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/clutter_packs.rs
//     evidence={"module":559,"record_offset":27688,"parent_record_offset":27664,"ancestor_record_offset":21608,"inlinee":35007,"source_line":25,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::steam::workshop_mods::enumerate_locally_overridden_items::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/steam/workshop_mods.rs
//     evidence={"module":559,"record_offset":22944,"parent_record_offset":22920,"ancestor_record_offset":21608,"inlinee":35004,"source_line":169,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":559,"record_offset":24264,"parent_record_offset":24236,"ancestor_record_offset":21608,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":559,"record_offset":24296,"parent_record_offset":24264,"ancestor_record_offset":21608,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":559,"record_offset":24236,"parent_record_offset":24196,"ancestor_record_offset":21608,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":559,"record_offset":27208,"parent_record_offset":27096,"ancestor_record_offset":21608,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":559,"record_offset":23956,"parent_record_offset":23928,"ancestor_record_offset":21608,"inlinee":4654,"source_line":59,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":559,"record_offset":27760,"parent_record_offset":27096,"ancestor_record_offset":21608,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/group/sse2.rs
//     evidence={"module":559,"record_offset":27292,"parent_record_offset":27096,"ancestor_record_offset":21608,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":559,"record_offset":27152,"parent_record_offset":27096,"ancestor_record_offset":21608,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":559,"record_offset":27640,"parent_record_offset":27616,"ancestor_record_offset":21608,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::contains_key
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":559,"record_offset":26988,"parent_record_offset":26964,"ancestor_record_offset":21608,"inlinee":5590,"source_line":1416,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":559,"record_offset":23820,"parent_record_offset":23796,"ancestor_record_offset":21608,"inlinee":8380,"source_line":754,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::keys
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":559,"record_offset":23796,"parent_record_offset":23772,"ancestor_record_offset":21608,"inlinee":9032,"source_line":650,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/map.rs
//     evidence={"module":559,"record_offset":27616,"parent_record_offset":27508,"ancestor_record_offset":21608,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$44::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":559,"record_offset":24144,"parent_record_offset":24120,"ancestor_record_offset":21608,"inlinee":8502,"source_line":3219,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$58::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":559,"record_offset":24120,"parent_record_offset":24096,"ancestor_record_offset":21608,"inlinee":9034,"source_line":3365,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$82::extend
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":559,"record_offset":22700,"parent_record_offset":22676,"ancestor_record_offset":21608,"inlinee":12036,"source_line":4649,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$82::extend::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":559,"record_offset":23020,"parent_record_offset":22996,"ancestor_record_offset":21608,"inlinee":12038,"source_line":4661,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":559,"record_offset":27020,"parent_record_offset":26988,"ancestor_record_offset":21608,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":559,"record_offset":27556,"parent_record_offset":27532,"ancestor_record_offset":21608,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":559,"record_offset":24416,"parent_record_offset":24196,"ancestor_record_offset":21608,"inlinee":4660,"source_line":490,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":559,"record_offset":27852,"parent_record_offset":27096,"ancestor_record_offset":21608,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":559,"record_offset":23928,"parent_record_offset":23868,"ancestor_record_offset":21608,"inlinee":4653,"source_line":3584,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":559,"record_offset":24196,"parent_record_offset":24168,"ancestor_record_offset":21608,"inlinee":4659,"source_line":3651,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":559,"record_offset":27532,"parent_record_offset":27508,"ancestor_record_offset":21608,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":559,"record_offset":27072,"parent_record_offset":27048,"ancestor_record_offset":21608,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":559,"record_offset":27048,"parent_record_offset":26988,"ancestor_record_offset":21608,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":559,"record_offset":23844,"parent_record_offset":23820,"ancestor_record_offset":21608,"inlinee":8381,"source_line":1395,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::allocation_info
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":559,"record_offset":26660,"parent_record_offset":26628,"ancestor_record_offset":21608,"inlinee":4366,"source_line":3160,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":559,"record_offset":23900,"parent_record_offset":23868,"ancestor_record_offset":21608,"inlinee":4651,"source_line":2478,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::drop_inner_table
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":559,"record_offset":26572,"parent_record_offset":26548,"ancestor_record_offset":21608,"inlinee":4363,"source_line":2311,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":559,"record_offset":27096,"parent_record_offset":27072,"ancestor_record_offset":21608,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::free_buckets
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":559,"record_offset":26628,"parent_record_offset":26572,"ancestor_record_offset":21608,"inlinee":4365,"source_line":3130,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_empty_singleton
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":559,"record_offset":26600,"parent_record_offset":26572,"ancestor_record_offset":21608,"inlinee":4364,"source_line":2696,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":559,"record_offset":23868,"parent_record_offset":23844,"ancestor_record_offset":21608,"inlinee":4650,"source_line":2178,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":559,"record_offset":27180,"parent_record_offset":27096,"ancestor_record_offset":21608,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::TableLayout::calculate_layout_for
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":559,"record_offset":26692,"parent_record_offset":26660,"ancestor_record_offset":21608,"inlinee":4368,"source_line":218,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":559,"record_offset":26548,"parent_record_offset":26524,"ancestor_record_offset":21608,"inlinee":4648,"source_line":3493,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":559,"record_offset":24168,"parent_record_offset":24144,"ancestor_record_offset":21608,"inlinee":4658,"source_line":3848,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/raw/mod.rs
//     evidence={"module":559,"record_offset":27508,"parent_record_offset":27096,"ancestor_record_offset":21608,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::contains
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\set.rs
//     evidence={"module":559,"record_offset":26964,"parent_record_offset":26940,"ancestor_record_offset":21608,"inlinee":5589,"source_line":862,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\set.rs
//     evidence={"module":559,"record_offset":23772,"parent_record_offset":23748,"ancestor_record_offset":21608,"inlinee":11496,"source_line":290,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::impl$10::extend
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\set.rs
//     evidence={"module":559,"record_offset":22676,"parent_record_offset":22652,"ancestor_record_offset":21608,"inlinee":12034,"source_line":1307,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::impl$25::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\set.rs
//     evidence={"module":559,"record_offset":24096,"parent_record_offset":24072,"ancestor_record_offset":21608,"inlinee":11498,"source_line":1798,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":559,"record_offset":28180,"parent_record_offset":28156,"ancestor_record_offset":21608,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":559,"record_offset":28156,"parent_record_offset":28132,"ancestor_record_offset":21608,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":559,"record_offset":28204,"parent_record_offset":28180,"ancestor_record_offset":21608,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::insert
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":559,"record_offset":24668,"parent_record_offset":24636,"ancestor_record_offset":21608,"inlinee":5499,"source_line":1295,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::HashSet::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":559,"record_offset":23748,"parent_record_offset":23724,"ancestor_record_offset":21608,"inlinee":12128,"source_line":349,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::HashSet::with_hasher
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":559,"record_offset":22624,"parent_record_offset":21924,"ancestor_record_offset":21608,"inlinee":11864,"source_line":234,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::impl$11::extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":559,"record_offset":22652,"parent_record_offset":21924,"ancestor_record_offset":21608,"inlinee":11866,"source_line":1200,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::impl$9::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":559,"record_offset":21924,"parent_record_offset":21888,"ancestor_record_offset":21608,"inlinee":11863,"source_line":1149,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":559,"record_offset":22000,"parent_record_offset":21972,"ancestor_record_offset":21608,"inlinee":5645,"source_line":55,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__rust_std_internal_init_fn
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":559,"record_offset":22260,"parent_record_offset":22236,"ancestor_record_offset":21608,"inlinee":5713,"source_line":85,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::constant$0::closure$1
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":559,"record_offset":22120,"parent_record_offset":22092,"ancestor_record_offset":21608,"inlinee":5705,"source_line":100,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":559,"record_offset":22456,"parent_record_offset":22056,"ancestor_record_offset":21608,"inlinee":5709,"source_line":71,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":559,"record_offset":21972,"parent_record_offset":21924,"ancestor_record_offset":21608,"inlinee":5643,"source_line":149,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$38::from
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":559,"record_offset":28544,"parent_record_offset":28520,"ancestor_record_offset":21608,"inlinee":22267,"source_line":1970,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$40::from
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":559,"record_offset":28520,"parent_record_offset":28496,"ancestor_record_offset":21608,"inlinee":32372,"source_line":1992,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":559,"record_offset":22148,"parent_record_offset":22120,"ancestor_record_offset":21608,"inlinee":5707,"source_line":58,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init_slow
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":559,"record_offset":22180,"parent_record_offset":22148,"ancestor_record_offset":21608,"inlinee":5711,"source_line":69,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":559,"record_offset":22056,"parent_record_offset":22028,"ancestor_record_offset":21608,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":559,"record_offset":22028,"parent_record_offset":22000,"ancestor_record_offset":21608,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::steam::workshop_mods::register_workshop_mod::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 146
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x88B6
// 首条位置: {"module":559,"inlinee":34998}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] tg_mods::identity::impl$11::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-mods/src/identity.rs
//     evidence={"module":559,"record_offset":17916,"parent_record_offset":17892,"ancestor_record_offset":17892,"inlinee":35000,"source_line":47,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$81::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":559,"record_offset":17944,"parent_record_offset":17916,"ancestor_record_offset":17892,"inlinee":5490,"source_line":350,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::partial_eq::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\partial_eq.rs
//     evidence={"module":559,"record_offset":17972,"parent_record_offset":17944,"ancestor_record_offset":17892,"inlinee":5492,"source_line":15,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":559,"record_offset":18000,"parent_record_offset":17972,"ancestor_record_offset":17892,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":559,"record_offset":18028,"parent_record_offset":18000,"ancestor_record_offset":17892,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::steam::workshop_mods::enumerate_locally_overridden_items
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 165
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x88BA
// 首条位置: {"module":559,"inlinee":35002}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":559,"record_offset":21888,"parent_record_offset":21836,"ancestor_record_offset":21836,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":559,"record_offset":23104,"parent_record_offset":21836,"ancestor_record_offset":21836,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":559,"record_offset":23356,"parent_record_offset":23332,"ancestor_record_offset":21836,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":559,"record_offset":23380,"parent_record_offset":23356,"ancestor_record_offset":21836,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":559,"record_offset":23404,"parent_record_offset":23380,"ancestor_record_offset":21836,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":559,"record_offset":23332,"parent_record_offset":23244,"ancestor_record_offset":21836,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":559,"record_offset":23272,"parent_record_offset":23244,"ancestor_record_offset":21836,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":559,"record_offset":23244,"parent_record_offset":23220,"ancestor_record_offset":21836,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":559,"record_offset":23220,"parent_record_offset":23196,"ancestor_record_offset":21836,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":559,"record_offset":23132,"parent_record_offset":23104,"ancestor_record_offset":21836,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":559,"record_offset":22484,"parent_record_offset":22456,"ancestor_record_offset":21836,"inlinee":5188,"source_line":552,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":559,"record_offset":22384,"parent_record_offset":22180,"ancestor_record_offset":21836,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter::filter_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\filter.rs
//     evidence={"module":559,"record_offset":22892,"parent_record_offset":22820,"ancestor_record_offset":21836,"inlinee":11536,"source_line":79,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter::impl$3::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\filter.rs
//     evidence={"module":559,"record_offset":22796,"parent_record_offset":22772,"ancestor_record_offset":21836,"inlinee":11531,"source_line":161,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter_map::filter_map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\filter_map.rs
//     evidence={"module":559,"record_offset":22920,"parent_record_offset":22892,"ancestor_record_offset":21836,"inlinee":17049,"source_line":39,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter_map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\filter_map.rs
//     evidence={"module":559,"record_offset":22772,"parent_record_offset":22748,"ancestor_record_offset":21836,"inlinee":17047,"source_line":142,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":559,"record_offset":22748,"parent_record_offset":22724,"ancestor_record_offset":21836,"inlinee":6631,"source_line":124,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":559,"record_offset":22972,"parent_record_offset":22920,"ancestor_record_offset":21836,"inlinee":5396,"source_line":88,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":559,"record_offset":22724,"parent_record_offset":22700,"ancestor_record_offset":21836,"inlinee":5042,"source_line":835,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":559,"record_offset":22996,"parent_record_offset":22972,"ancestor_record_offset":21836,"inlinee":5057,"source_line":842,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":559,"record_offset":22348,"parent_record_offset":22324,"ancestor_record_offset":21836,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":559,"record_offset":23300,"parent_record_offset":23272,"ancestor_record_offset":21836,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":559,"record_offset":22512,"parent_record_offset":22456,"ancestor_record_offset":21836,"inlinee":5710,"source_line":2456,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":559,"record_offset":22092,"parent_record_offset":22056,"ancestor_record_offset":21836,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":559,"record_offset":22212,"parent_record_offset":22180,"ancestor_record_offset":21836,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":559,"record_offset":23160,"parent_record_offset":23132,"ancestor_record_offset":21836,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::replace
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":559,"record_offset":22300,"parent_record_offset":22180,"ancestor_record_offset":21836,"inlinee":5714,"source_line":1490,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":559,"record_offset":22864,"parent_record_offset":22820,"ancestor_record_offset":21836,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::replace
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":559,"record_offset":22324,"parent_record_offset":22300,"ancestor_record_offset":21836,"inlinee":5715,"source_line":1542,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":559,"record_offset":22820,"parent_record_offset":22796,"ancestor_record_offset":21836,"inlinee":6636,"source_line":259,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::steam::workshop_mods::enumerate_locally_overridden_items::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/steam/workshop_mods.rs
//     evidence={"module":559,"record_offset":22944,"parent_record_offset":22920,"ancestor_record_offset":21836,"inlinee":35004,"source_line":169,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$82::extend
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":559,"record_offset":22700,"parent_record_offset":22676,"ancestor_record_offset":21836,"inlinee":12036,"source_line":4649,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$82::extend::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":559,"record_offset":23020,"parent_record_offset":22996,"ancestor_record_offset":21836,"inlinee":12038,"source_line":4661,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::impl$10::extend
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\set.rs
//     evidence={"module":559,"record_offset":22676,"parent_record_offset":22652,"ancestor_record_offset":21836,"inlinee":12034,"source_line":1307,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::HashSet::with_hasher
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":559,"record_offset":22624,"parent_record_offset":21924,"ancestor_record_offset":21836,"inlinee":11864,"source_line":234,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::impl$11::extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":559,"record_offset":22652,"parent_record_offset":21924,"ancestor_record_offset":21836,"inlinee":11866,"source_line":1200,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::impl$9::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":559,"record_offset":21924,"parent_record_offset":21888,"ancestor_record_offset":21836,"inlinee":11863,"source_line":1149,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":559,"record_offset":22000,"parent_record_offset":21972,"ancestor_record_offset":21836,"inlinee":5645,"source_line":55,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__rust_std_internal_init_fn
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":559,"record_offset":22260,"parent_record_offset":22236,"ancestor_record_offset":21836,"inlinee":5713,"source_line":85,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::constant$0::closure$1
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":559,"record_offset":22120,"parent_record_offset":22092,"ancestor_record_offset":21836,"inlinee":5705,"source_line":100,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":559,"record_offset":22456,"parent_record_offset":22056,"ancestor_record_offset":21836,"inlinee":5709,"source_line":71,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":559,"record_offset":21972,"parent_record_offset":21924,"ancestor_record_offset":21836,"inlinee":5643,"source_line":149,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":559,"record_offset":22148,"parent_record_offset":22120,"ancestor_record_offset":21836,"inlinee":5707,"source_line":58,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init_slow
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":559,"record_offset":22180,"parent_record_offset":22148,"ancestor_record_offset":21836,"inlinee":5711,"source_line":69,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":559,"record_offset":22056,"parent_record_offset":22028,"ancestor_record_offset":21836,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":559,"record_offset":22028,"parent_record_offset":22000,"ancestor_record_offset":21836,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::steam::workshop_mods::enumerate_locally_overridden_items::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 169
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x88BC
// 首条位置: {"module":559,"inlinee":35004}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
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
// 完整结构及原始类型记录: workshop_mods.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: workshop_mods.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: workshop_mods.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 15; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] usize (*)(ref$<hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> >)
// 候选参数类型: ["ref$<hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> >"]
// [文件级接口候选，具体函数归属 unknown] core::cmp::Ordering (*)(ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >,ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >)
// 候选参数类型: ["ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >", "ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >"]
// [文件级接口候选，具体函数归属 unknown] f32 (*)(f32,f32)
// 候选参数类型: ["f32", "f32"]
// [文件级接口候选，具体函数归属 unknown] void (*)(core::pin::Pin<alloc::boxed::Box<dyn$<core::future::future::Future<assoc$<Output,tuple$<> > >,core::marker::Send>,alloc::alloc::Global> >)
// 候选参数类型: ["core::pin::Pin<alloc::boxed::Box<dyn$<core::future::future::Future<assoc$<Output,tuple$<> > >,core::marker::Send>,alloc::alloc::Global> >"]
// [文件级接口候选，具体函数归属 unknown] ref$<str$> (*)(ref$<alloc::string::String>)
// 候选参数类型: ["ref$<alloc::string::String>"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<std::fs::DirEntry> > (*)(enum2$<core::result::Result<std::fs::DirEntry,std::io::error::Error> >)
// 候选参数类型: ["enum2$<core::result::Result<std::fs::DirEntry,std::io::error::Error> >"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::ops::control_flow::ControlFlow<tuple$<ref$<country_core::resources::walls::public_wall_state::PublicWallState>,country_core::systems::ui_wall_transform_gizmos::WallWireMetrics>,tuple$<> > > (*)(tuple$<ref$<country_core::resources::walls::public_wall_state::PublicWallState>,country_core::systems::ui_wall_transform_gizmos::WallWireMetrics>)
// 候选参数类型: ["tuple$<ref$<country_core::resources::walls::public_wall_state::PublicWallState>,country_core::systems::ui_wall_transform_gizmos::WallWireMetrics>"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::ops::control_flow::ControlFlow<rhapsody::raw::buffer::Buffer,tuple$<> > > (*)(rhapsody::raw::buffer::Buffer)
// 候选参数类型: ["rhapsody::raw::buffer::Buffer"]
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::systems::steam::workshop_mods::enumerate_locally_overridden_items
// 信息 ID: b2ff5d8dd4fd17c1660dca53
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::steam::workshop_mods::register_subscribed_workshop_mods; module=559, inline_record=21836; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::steam::workshop_mods::enumerate_locally_overridden_items::closure$1
// 信息 ID: fcc7680f079f6cec4e2bb2d2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::systems::steam::workshop_mods::enumerate_locally_overridden_items", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::steam::workshop_mods::register_subscribed_workshop_mods; module=559, inline_record=22944; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::steam::workshop_mods::register_subscribed_workshop_mods
// 信息 ID: 84185e348f1723bd47a37695
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] b3f619fc6d4aeef81fc2770d: void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> > >)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> > >),tuple$<>,void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> > >)> @ 0x17fc624 -> 0xafe770 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> > >),tuple$<>,void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> > >)> > @ 0x193ba94 -> 0xafe770 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::steam::workshop_mods::register_workshop_mod
// 信息 ID: a5f174f9bc28f445aef6eb8f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::steam::workshop_mods::register_subscribed_workshop_mods @ 0xafee3e -> 0xafd8e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::steam::workshop_clutter::workshop_clutter_hot_install @ 0xbf59b9 -> 0xafd8e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::steam::workshop_mods::register_workshop_mod::closure$0
// 信息 ID: 51775c50345dcf2c6bd8fb1d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::steam::workshop_mods::register_workshop_mod", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::steam::workshop_mods::register_workshop_mod; module=559, inline_record=17892; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===

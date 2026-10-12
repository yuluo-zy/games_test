// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/country-core/src/systems/history/replay_wall.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 8f1b2700123462e0dda0b5b98f5c0cb9863a261357d5ca474f5ac4a6f81e0eea
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: replay_wall.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 594, 1643, 1682, 1715, 1720
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
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.30.10"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.1"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.1"}]
// use lerp; // namespace candidate
//   package identities: [{"name":"lerp","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.0"}]
// use std; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::history::replay_wall::WallReplayCmd::new
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1715,"record_offset":14476,"rva":27007968,"va":5395717088}
// 编译实例 RVAs: 0x19c1be0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-20038452","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395717088,"raw_hex":"e8cc3ccefe","size":5,"soft_fail":false,"target_va":5375678688,"va":5395717135,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: replay_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-26994933","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395717088,"raw_hex":"e80b1764fe","size":5,"soft_fail":false,"target_va":5368722192,"va":5395717120,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1416099b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3900093","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395717088,"raw_hex":"e8437dc4ff","size":5,"soft_fail":false,"target_va":5391817136,"va":5395717224,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t20903484","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395717088,"raw_hex":"e83cf63e01","size":5,"soft_fail":false,"target_va":5416620769,"va":5395717280,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":1715,"record_offset":14632,"parent_record_offset":14476,"ancestor_record_offset":14476,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1715,"record_offset":14828,"parent_record_offset":14476,"ancestor_record_offset":14476,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1715,"record_offset":14744,"parent_record_offset":14476,"ancestor_record_offset":14476,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":1715,"record_offset":14656,"parent_record_offset":14632,"ancestor_record_offset":14476,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::impl$16::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1715,"record_offset":14852,"parent_record_offset":14828,"ancestor_record_offset":14476,"inlinee":12130,"source_line":3641,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::impl$23::from
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1715,"record_offset":14960,"parent_record_offset":14928,"ancestor_record_offset":14476,"inlinee":12134,"source_line":3746,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::spec_from_iter::impl$0::spec_from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\spec_from_iter.rs
//     evidence={"module":1715,"record_offset":14904,"parent_record_offset":14852,"ancestor_record_offset":14476,"inlinee":12132,"source_line":12,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::into_raw_parts_with_alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1715,"record_offset":14988,"parent_record_offset":14960,"ancestor_record_offset":14476,"inlinee":12135,"source_line":1339,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1715,"record_offset":14928,"parent_record_offset":14904,"ancestor_record_offset":14476,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":1715,"record_offset":14876,"parent_record_offset":14852,"ancestor_record_offset":14476,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\manually_drop.rs
//     evidence={"module":1715,"record_offset":15012,"parent_record_offset":14988,"ancestor_record_offset":14476,"inlinee":5072,"source_line":184,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_dangling::MaybeDangling::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_dangling.rs
//     evidence={"module":1715,"record_offset":15036,"parent_record_offset":15012,"ancestor_record_offset":14476,"inlinee":5074,"source_line":80,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1715,"record_offset":14792,"parent_record_offset":14768,"ancestor_record_offset":14476,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":1715,"record_offset":14768,"parent_record_offset":14744,"ancestor_record_offset":14476,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::history::replay_wall::replay_wall
// 记录形式: compiled_procedure; instances=1
// 源码行: 38
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1715,"record_offset":268,"rva":27004288,"va":5395713408}
// 编译实例 RVAs: 0x19c0d80
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <alloc::vec::Vec<_, _>>::remove::assert_failed
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t20904484","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395713408,"raw_hex":"e824fa3e01","size":5,"soft_fail":false,"target_va":5416621000,"va":5395716511,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::hash::random::RandomState>::new::KEYS::{K#0}::{closure#1}::__RUST_STD_INTERNAL_VAL::{shim:tls#0}
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-11068772","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395713408,"raw_hex":"e89c1a57ff","size":5,"soft_fail":false,"target_va":5384644912,"va":5395713679,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=6
//     evidence={"assembly":"\tcallq\t-20036534","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395713408,"raw_hex":"e84a44cefe","size":5,"soft_fail":false,"target_va":5375678704,"va":5395715233,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14007db70
//     owner=unresolved_namespace; count=2
//     target_aliases=27 candidates; complete list: replay_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-26492853","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395713408,"raw_hex":"e84bc06bfe","size":5,"soft_fail":false,"target_va":5369224048,"va":5395716896,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140081340
//     owner=unresolved_namespace; count=1
//     target_aliases=6 candidates; complete list: replay_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-26476826","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395713408,"raw_hex":"e8e6fe6bfe","size":5,"soft_fail":false,"target_va":5369238336,"va":5395715157,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14040ebd0
//     owner=unresolved_namespace; count=2
//     target_aliases=7 candidates; complete list: replay_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-22750318","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395713408,"raw_hex":"e892dba4fe","size":5,"soft_fail":false,"target_va":5372963792,"va":5395714105,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404378e0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: replay_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-22585065","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395713408,"raw_hex":"e81761a7fe","size":5,"soft_fail":false,"target_va":5373130976,"va":5395716036,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048c880
//     owner=unresolved_namespace; count=2
//     target_aliases=6 candidates; complete list: replay_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-22235465","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395713408,"raw_hex":"e8b7b6acfe","size":5,"soft_fail":false,"target_va":5373479040,"va":5395714500,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14073aca0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: replay_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-19426379","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395713408,"raw_hex":"e8b593d7fe","size":5,"soft_fail":false,"target_va":5376289952,"va":5395716326,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14099f7b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16916221","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395713408,"raw_hex":"e803e1fdfe","size":5,"soft_fail":false,"target_va":5378799536,"va":5395715752,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c90880
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13832215","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395713408,"raw_hex":"e8e9ef2cff","size":5,"soft_fail":false,"target_va":5381884032,"va":5395716242,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c91660
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13828789","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395713408,"raw_hex":"e84bfd2cff","size":5,"soft_fail":false,"target_va":5381887584,"va":5395716368,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c916c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13828441","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395713408,"raw_hex":"e8a7fe2cff","size":5,"soft_fail":false,"target_va":5381887680,"va":5395716116,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c918c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13828219","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395713408,"raw_hex":"e885ff2cff","size":5,"soft_fail":false,"target_va":5381888192,"va":5395716406,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c91ff0
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: replay_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13826131","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395713408,"raw_hex":"e8ad072dff","size":5,"soft_fail":false,"target_va":5381890032,"va":5395716158,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c92250
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: replay_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13825359","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395713408,"raw_hex":"e8b10a2dff","size":5,"soft_fail":false,"target_va":5381890640,"va":5395715994,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1419e0090
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t127173","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395713408,"raw_hex":"e8c5f00100","size":5,"soft_fail":false,"target_va":5395841168,"va":5395713990,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb30
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: replay_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t21341186","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395713408,"raw_hex":"e802a44501","size":5,"soft_fail":false,"target_va":5417057072,"va":5395715881,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t20650684","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395713408,"raw_hex":"e8bc1a3b01","size":5,"soft_fail":false,"target_va":5416365968,"va":5395715279,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::random::hashmap_random_keys
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-10988560","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395713408,"raw_hex":"e8f05358ff","size":5,"soft_fail":false,"target_va":5384726688,"va":5395715243,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::collections::vec_deque::VecDeque::front_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1715,"record_offset":1884,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":30608,"source_line":1998,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_direct] alloc::collections::vec_deque::VecDeque::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1715,"record_offset":3508,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":12113,"source_line":907,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_direct] alloc::collections::vec_deque::VecDeque::pop_front
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1715,"record_offset":8180,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":12109,"source_line":2064,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1715,"record_offset":916,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1715,"record_offset":9100,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::remove
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1715,"record_offset":12936,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":6391,"source_line":2341,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$18::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1715,"record_offset":2500,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":8387,"source_line":3913,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$19::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1715,"record_offset":2232,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":9357,"source_line":3923,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":1715,"record_offset":10396,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":5075,"source_line":213,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":1715,"record_offset":484,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1715,"record_offset":1028,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":5459,"source_line":79,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1715,"record_offset":8584,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1715,"record_offset":944,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":6145,"source_line":1060,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::reverse
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1715,"record_offset":10284,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":5742,"source_line":978,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1715,"record_offset":2612,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$179::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1715,"record_offset":2344,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":5064,"source_line":157,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::history::replay_cmd_buffer::ReplayBuffer::front_mut_bucket
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/replay_cmd_buffer.rs
//     evidence={"module":1715,"record_offset":548,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":35487,"source_line":202,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::history::wall_compound_edit::WallHistoryEdit::gather_wall_ids
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/wall_compound_edit.rs
//     evidence={"module":1715,"record_offset":2704,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":35484,"source_line":316,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::HashSet::is_disjoint
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":1715,"record_offset":4128,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":35591,"source_line":916,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::HashSet::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":1715,"record_offset":1168,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":5639,"source_line":149,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1715,"record_offset":8972,"parent_record_offset":8944,"ancestor_record_offset":268,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1715,"record_offset":9000,"parent_record_offset":8972,"ancestor_record_offset":268,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1715,"record_offset":9032,"parent_record_offset":9000,"ancestor_record_offset":268,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1715,"record_offset":8944,"parent_record_offset":8788,"ancestor_record_offset":268,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::buffer_read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1715,"record_offset":8416,"parent_record_offset":8180,"ancestor_record_offset":268,"inlinee":12111,"source_line":208,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::capacity
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1715,"record_offset":684,"parent_record_offset":656,"ancestor_record_offset":268,"inlinee":4786,"source_line":991,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::front_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1715,"record_offset":572,"parent_record_offset":548,"ancestor_record_offset":268,"inlinee":30608,"source_line":1998,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::get_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1715,"record_offset":596,"parent_record_offset":572,"ancestor_record_offset":268,"inlinee":30609,"source_line":937,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1715,"record_offset":8224,"parent_record_offset":8180,"ancestor_record_offset":268,"inlinee":12110,"source_line":1650,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1715,"record_offset":784,"parent_record_offset":596,"ancestor_record_offset":268,"inlinee":4790,"source_line":173,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::to_physical_idx
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1715,"record_offset":628,"parent_record_offset":596,"ancestor_record_offset":268,"inlinee":4784,"source_line":248,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::wrap_add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1715,"record_offset":656,"parent_record_offset":628,"ancestor_record_offset":268,"inlinee":4785,"source_line":243,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1715,"record_offset":716,"parent_record_offset":684,"ancestor_record_offset":268,"inlinee":4787,"source_line":307,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1715,"record_offset":816,"parent_record_offset":784,"ancestor_record_offset":268,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1715,"record_offset":740,"parent_record_offset":716,"ancestor_record_offset":268,"inlinee":4788,"source_line":616,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1715,"record_offset":13448,"parent_record_offset":13420,"ancestor_record_offset":268,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1715,"record_offset":13420,"parent_record_offset":13396,"ancestor_record_offset":268,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1715,"record_offset":864,"parent_record_offset":840,"ancestor_record_offset":268,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1715,"record_offset":840,"parent_record_offset":816,"ancestor_record_offset":268,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1715,"record_offset":13396,"parent_record_offset":13372,"ancestor_record_offset":268,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":1715,"record_offset":10640,"parent_record_offset":10604,"ancestor_record_offset":268,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":1715,"record_offset":10604,"parent_record_offset":10580,"ancestor_record_offset":268,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1715,"record_offset":9160,"parent_record_offset":9124,"ancestor_record_offset":268,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1715,"record_offset":13020,"parent_record_offset":12972,"ancestor_record_offset":268,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1715,"record_offset":9124,"parent_record_offset":9100,"ancestor_record_offset":268,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1715,"record_offset":13240,"parent_record_offset":12972,"ancestor_record_offset":268,"inlinee":4802,"source_line":2161,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::try_remove
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1715,"record_offset":12972,"parent_record_offset":12936,"ancestor_record_offset":268,"inlinee":6392,"source_line":2377,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":1715,"record_offset":13300,"parent_record_offset":13276,"ancestor_record_offset":268,"inlinee":4693,"source_line":497,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":1715,"record_offset":13348,"parent_record_offset":13324,"ancestor_record_offset":268,"inlinee":4706,"source_line":501,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":1715,"record_offset":512,"parent_record_offset":484,"ancestor_record_offset":268,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1715,"record_offset":1736,"parent_record_offset":1708,"ancestor_record_offset":268,"inlinee":5188,"source_line":552,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1715,"record_offset":1636,"parent_record_offset":1432,"ancestor_record_offset":268,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1715,"record_offset":5720,"parent_record_offset":5696,"ancestor_record_offset":268,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1715,"record_offset":4788,"parent_record_offset":4764,"ancestor_record_offset":268,"inlinee":4655,"source_line":1319,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1715,"record_offset":5288,"parent_record_offset":5264,"ancestor_record_offset":268,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1715,"record_offset":5372,"parent_record_offset":5348,"ancestor_record_offset":268,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1715,"record_offset":5788,"parent_record_offset":5184,"ancestor_record_offset":268,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::all
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1715,"record_offset":4360,"parent_record_offset":4128,"ancestor_record_offset":268,"inlinee":8990,"source_line":2809,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::all::check::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1715,"record_offset":4984,"parent_record_offset":4400,"ancestor_record_offset":268,"inlinee":8997,"source_line":2816,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1715,"record_offset":4400,"parent_record_offset":4360,"ancestor_record_offset":268,"inlinee":5047,"source_line":2461,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":1715,"record_offset":1600,"parent_record_offset":1576,"ancestor_record_offset":268,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::swap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":1715,"record_offset":10356,"parent_record_offset":10316,"ancestor_record_offset":268,"inlinee":5745,"source_line":756,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1715,"record_offset":8880,"parent_record_offset":8852,"ancestor_record_offset":268,"inlinee":4369,"source_line":761,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1715,"record_offset":13476,"parent_record_offset":13448,"ancestor_record_offset":268,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1715,"record_offset":3600,"parent_record_offset":3572,"ancestor_record_offset":268,"inlinee":4834,"source_line":2456,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1715,"record_offset":1764,"parent_record_offset":1708,"ancestor_record_offset":268,"inlinee":5710,"source_line":2456,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":1715,"record_offset":5520,"parent_record_offset":5492,"ancestor_record_offset":268,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":1715,"record_offset":4724,"parent_record_offset":4700,"ancestor_record_offset":268,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":1715,"record_offset":1344,"parent_record_offset":1308,"ancestor_record_offset":268,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1715,"record_offset":1464,"parent_record_offset":1432,"ancestor_record_offset":268,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1715,"record_offset":4876,"parent_record_offset":4600,"ancestor_record_offset":268,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1715,"record_offset":5312,"parent_record_offset":5288,"ancestor_record_offset":268,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=36
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1715,"record_offset":8636,"parent_record_offset":8612,"ancestor_record_offset":268,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1715,"record_offset":992,"parent_record_offset":968,"ancestor_record_offset":268,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::replace
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1715,"record_offset":1552,"parent_record_offset":1432,"ancestor_record_offset":268,"inlinee":5714,"source_line":1490,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1715,"record_offset":4844,"parent_record_offset":4820,"ancestor_record_offset":268,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1715,"record_offset":1132,"parent_record_offset":1064,"ancestor_record_offset":268,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1715,"record_offset":10672,"parent_record_offset":10640,"ancestor_record_offset":268,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1715,"record_offset":10468,"parent_record_offset":10396,"ancestor_record_offset":268,"inlinee":5076,"source_line":985,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1715,"record_offset":1100,"parent_record_offset":1064,"ancestor_record_offset":268,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1715,"record_offset":10492,"parent_record_offset":10468,"ancestor_record_offset":268,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::replace
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1715,"record_offset":1576,"parent_record_offset":1552,"ancestor_record_offset":268,"inlinee":5715,"source_line":1542,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1715,"record_offset":9272,"parent_record_offset":9124,"ancestor_record_offset":268,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1715,"record_offset":2524,"parent_record_offset":2500,"ancestor_record_offset":268,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1715,"record_offset":2256,"parent_record_offset":2232,"ancestor_record_offset":268,"inlinee":6145,"source_line":1060,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::reverse::revswap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1715,"record_offset":10316,"parent_record_offset":10284,"ancestor_record_offset":268,"inlinee":5744,"source_line":1001,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":1715,"record_offset":2548,"parent_record_offset":2524,"ancestor_record_offset":268,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":1715,"record_offset":968,"parent_record_offset":944,"ancestor_record_offset":268,"inlinee":6147,"source_line":221,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$179::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1715,"record_offset":1064,"parent_record_offset":1028,"ancestor_record_offset":268,"inlinee":5064,"source_line":157,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1715,"record_offset":10700,"parent_record_offset":10640,"ancestor_record_offset":268,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1715,"record_offset":10732,"parent_record_offset":10604,"ancestor_record_offset":268,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$10::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":1715,"record_offset":5744,"parent_record_offset":5720,"ancestor_record_offset":268,"inlinee":12117,"source_line":82,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1715,"record_offset":4668,"parent_record_offset":4640,"ancestor_record_offset":268,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1715,"record_offset":4700,"parent_record_offset":4668,"ancestor_record_offset":268,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1715,"record_offset":4640,"parent_record_offset":4600,"ancestor_record_offset":268,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1715,"record_offset":5264,"parent_record_offset":5184,"ancestor_record_offset":268,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1715,"record_offset":4764,"parent_record_offset":4600,"ancestor_record_offset":268,"inlinee":4654,"source_line":59,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1715,"record_offset":5816,"parent_record_offset":5184,"ancestor_record_offset":268,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1715,"record_offset":5348,"parent_record_offset":5184,"ancestor_record_offset":268,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":1715,"record_offset":5236,"parent_record_offset":5184,"ancestor_record_offset":268,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":1715,"record_offset":5696,"parent_record_offset":5672,"ancestor_record_offset":268,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::contains_key
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1715,"record_offset":5080,"parent_record_offset":5056,"ancestor_record_offset":268,"inlinee":5590,"source_line":1416,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1715,"record_offset":6024,"parent_record_offset":6000,"ancestor_record_offset":268,"inlinee":8380,"source_line":754,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::keys
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1715,"record_offset":6000,"parent_record_offset":5976,"ancestor_record_offset":268,"inlinee":9032,"source_line":650,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::len
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1715,"record_offset":4212,"parent_record_offset":4188,"ancestor_record_offset":268,"inlinee":12127,"source_line":828,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1715,"record_offset":5672,"parent_record_offset":5564,"ancestor_record_offset":268,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$44::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1715,"record_offset":4532,"parent_record_offset":4500,"ancestor_record_offset":268,"inlinee":8502,"source_line":3219,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$58::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1715,"record_offset":4500,"parent_record_offset":4468,"ancestor_record_offset":268,"inlinee":9034,"source_line":3365,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1715,"record_offset":5108,"parent_record_offset":5080,"ancestor_record_offset":268,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1715,"record_offset":5612,"parent_record_offset":5588,"ancestor_record_offset":268,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1715,"record_offset":4820,"parent_record_offset":4600,"ancestor_record_offset":268,"inlinee":4660,"source_line":490,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1715,"record_offset":5908,"parent_record_offset":5184,"ancestor_record_offset":268,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1715,"record_offset":6128,"parent_record_offset":6072,"ancestor_record_offset":268,"inlinee":4653,"source_line":3584,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1715,"record_offset":4600,"parent_record_offset":4564,"ancestor_record_offset":268,"inlinee":4659,"source_line":3651,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1715,"record_offset":5588,"parent_record_offset":5564,"ancestor_record_offset":268,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1715,"record_offset":5160,"parent_record_offset":5136,"ancestor_record_offset":268,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1715,"record_offset":5136,"parent_record_offset":5080,"ancestor_record_offset":268,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1715,"record_offset":6048,"parent_record_offset":6024,"ancestor_record_offset":268,"inlinee":8381,"source_line":1395,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::len
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1715,"record_offset":4236,"parent_record_offset":4212,"ancestor_record_offset":268,"inlinee":9371,"source_line":1364,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::allocation_info
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1715,"record_offset":8820,"parent_record_offset":8788,"ancestor_record_offset":268,"inlinee":4366,"source_line":3160,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1715,"record_offset":6100,"parent_record_offset":6072,"ancestor_record_offset":268,"inlinee":4651,"source_line":2478,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::drop_inner_table
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1715,"record_offset":8732,"parent_record_offset":8708,"ancestor_record_offset":268,"inlinee":4363,"source_line":2311,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1715,"record_offset":5184,"parent_record_offset":5160,"ancestor_record_offset":268,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::free_buckets
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1715,"record_offset":8788,"parent_record_offset":8732,"ancestor_record_offset":268,"inlinee":4365,"source_line":3130,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_empty_singleton
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1715,"record_offset":8760,"parent_record_offset":8732,"ancestor_record_offset":268,"inlinee":4364,"source_line":2696,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1715,"record_offset":6072,"parent_record_offset":6048,"ancestor_record_offset":268,"inlinee":4650,"source_line":2178,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::TableLayout::calculate_layout_for
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1715,"record_offset":8852,"parent_record_offset":8820,"ancestor_record_offset":268,"inlinee":4368,"source_line":218,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1715,"record_offset":8708,"parent_record_offset":8684,"ancestor_record_offset":268,"inlinee":4648,"source_line":3493,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1715,"record_offset":4564,"parent_record_offset":4532,"ancestor_record_offset":268,"inlinee":4658,"source_line":3848,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1715,"record_offset":5564,"parent_record_offset":5184,"ancestor_record_offset":268,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::contains
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\set.rs
//     evidence={"module":1715,"record_offset":5056,"parent_record_offset":5032,"ancestor_record_offset":268,"inlinee":5589,"source_line":862,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::insert
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\set.rs
//     evidence={"module":1715,"record_offset":2756,"parent_record_offset":2732,"ancestor_record_offset":268,"inlinee":5647,"source_line":1092,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::iter
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\set.rs
//     evidence={"module":1715,"record_offset":5976,"parent_record_offset":4360,"ancestor_record_offset":268,"inlinee":11496,"source_line":290,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::len
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\set.rs
//     evidence={"module":1715,"record_offset":4188,"parent_record_offset":4164,"ancestor_record_offset":268,"inlinee":12126,"source_line":309,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::impl$25::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\set.rs
//     evidence={"module":1715,"record_offset":4468,"parent_record_offset":4436,"ancestor_record_offset":268,"inlinee":11498,"source_line":1798,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::HashSet::contains
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":1715,"record_offset":5032,"parent_record_offset":5008,"ancestor_record_offset":268,"inlinee":5587,"source_line":767,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::HashSet::insert
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":1715,"record_offset":2732,"parent_record_offset":2704,"ancestor_record_offset":268,"inlinee":5646,"source_line":995,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::HashSet::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":1715,"record_offset":4164,"parent_record_offset":4128,"ancestor_record_offset":268,"inlinee":12125,"source_line":367,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::impl$13::default
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":1715,"record_offset":1196,"parent_record_offset":1168,"ancestor_record_offset":268,"inlinee":5641,"source_line":1246,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::impl$23::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":1715,"record_offset":4436,"parent_record_offset":4400,"ancestor_record_offset":268,"inlinee":16842,"source_line":1676,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::impl$4::is_disjoint::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":1715,"record_offset":6916,"parent_record_offset":6892,"ancestor_record_offset":268,"inlinee":35560,"source_line":918,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::impl$4::is_disjoint::closure$1
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":1715,"record_offset":5008,"parent_record_offset":4984,"ancestor_record_offset":268,"inlinee":35559,"source_line":920,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":1715,"record_offset":1252,"parent_record_offset":1224,"ancestor_record_offset":268,"inlinee":5645,"source_line":55,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__rust_std_internal_init_fn
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":1715,"record_offset":1512,"parent_record_offset":1488,"ancestor_record_offset":268,"inlinee":5713,"source_line":85,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::constant$0::closure$1
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":1715,"record_offset":1372,"parent_record_offset":1344,"ancestor_record_offset":268,"inlinee":5705,"source_line":100,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":1715,"record_offset":1708,"parent_record_offset":1308,"ancestor_record_offset":268,"inlinee":5709,"source_line":71,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":1715,"record_offset":1224,"parent_record_offset":1196,"ancestor_record_offset":268,"inlinee":5643,"source_line":149,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":1715,"record_offset":1400,"parent_record_offset":1372,"ancestor_record_offset":268,"inlinee":5707,"source_line":58,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init_slow
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":1715,"record_offset":1432,"parent_record_offset":1400,"ancestor_record_offset":268,"inlinee":5711,"source_line":69,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":1715,"record_offset":1308,"parent_record_offset":1280,"ancestor_record_offset":268,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":1715,"record_offset":1280,"parent_record_offset":1252,"ancestor_record_offset":268,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::history::replay_wall::replay_wall::closure$0
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=4
// 源码行: 82
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x145AD
// 首条位置: {"module":1720,"inlinee":83373}
// 编译实例 RVAs: 0x19e0000
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140c71040
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14086112","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395841024,"raw_hex":"e8201029ff","size":5,"soft_fail":false,"target_va":5381754944,"va":5395841051,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f2b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t20509283","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395841024,"raw_hex":"e863f23801","size":5,"soft_fail":false,"target_va":5416350384,"va":5395841096,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::message::message_writer::MessageWriter::write
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_writer.rs
//     evidence={"module":1720,"record_offset":128288,"parent_record_offset":128136,"ancestor_record_offset":128136,"inlinee":11206,"source_line":69,"archive":"_pdb/C/modules/01720.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1720,"record_offset":128512,"parent_record_offset":128480,"ancestor_record_offset":128136,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/01720.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1720,"record_offset":128560,"parent_record_offset":128536,"ancestor_record_offset":128136,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/01720.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1720,"record_offset":128536,"parent_record_offset":128512,"ancestor_record_offset":128136,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/01720.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1720,"record_offset":128480,"parent_record_offset":128440,"ancestor_record_offset":128136,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/01720.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1720,"record_offset":128408,"parent_record_offset":128372,"ancestor_record_offset":128136,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/01720.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1720,"record_offset":128440,"parent_record_offset":128408,"ancestor_record_offset":128136,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/01720.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":1720,"record_offset":128344,"parent_record_offset":128312,"ancestor_record_offset":128136,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/01720.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":1720,"record_offset":128312,"parent_record_offset":128288,"ancestor_record_offset":128136,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/01720.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::messages::Messages::write_with_caller
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs
//     evidence={"module":1720,"record_offset":128372,"parent_record_offset":128312,"ancestor_record_offset":128136,"inlinee":8681,"source_line":129,"archive":"_pdb/C/modules/01720.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1720,"record_offset":128600,"parent_record_offset":128440,"ancestor_record_offset":128136,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/01720.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::history::replay_wall::replay_wall::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 87
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x145AE
// 首条位置: {"module":1720,"inlinee":83374}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::history::replay_wall::WallReplayCmd::is_single_adjust_height_cmd
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 167
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x93CE
// 首条位置: {"module":594,"inlinee":37838}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::collections::vec_deque::impl$14::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":594,"record_offset":32664,"parent_record_offset":32632,"ancestor_record_offset":32632,"inlinee":13895,"source_line":3626,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":594,"record_offset":32776,"parent_record_offset":32748,"ancestor_record_offset":32632,"inlinee":4786,"source_line":991,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":594,"record_offset":32688,"parent_record_offset":32664,"ancestor_record_offset":32632,"inlinee":12113,"source_line":907,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":594,"record_offset":32876,"parent_record_offset":32688,"ancestor_record_offset":32632,"inlinee":4790,"source_line":173,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::to_physical_idx
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":594,"record_offset":32720,"parent_record_offset":32688,"ancestor_record_offset":32632,"inlinee":4784,"source_line":248,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::wrap_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":594,"record_offset":32748,"parent_record_offset":32720,"ancestor_record_offset":32632,"inlinee":4785,"source_line":243,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":594,"record_offset":32808,"parent_record_offset":32776,"ancestor_record_offset":32632,"inlinee":4787,"source_line":307,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":594,"record_offset":32912,"parent_record_offset":32876,"ancestor_record_offset":32632,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":594,"record_offset":32832,"parent_record_offset":32808,"ancestor_record_offset":32632,"inlinee":4788,"source_line":616,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":594,"record_offset":32960,"parent_record_offset":32936,"ancestor_record_offset":32632,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":594,"record_offset":32936,"parent_record_offset":32912,"ancestor_record_offset":32632,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00594.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::history::replay_wall::impl$0::new::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 185
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13DBB
// 首条位置: {"module":1643,"inlinee":81339}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1643,"record_offset":540036,"parent_record_offset":540000,"ancestor_record_offset":540000,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1643,"record_offset":540728,"parent_record_offset":540000,"ancestor_record_offset":540000,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1643,"record_offset":540492,"parent_record_offset":540468,"ancestor_record_offset":540000,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1643,"record_offset":540516,"parent_record_offset":540492,"ancestor_record_offset":540000,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1643,"record_offset":540540,"parent_record_offset":540516,"ancestor_record_offset":540000,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1643,"record_offset":540468,"parent_record_offset":540288,"ancestor_record_offset":540000,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1643,"record_offset":540232,"parent_record_offset":540204,"ancestor_record_offset":540000,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1643,"record_offset":540288,"parent_record_offset":540256,"ancestor_record_offset":540000,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1643,"record_offset":540256,"parent_record_offset":540232,"ancestor_record_offset":540000,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1643,"record_offset":540324,"parent_record_offset":540288,"ancestor_record_offset":540000,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1643,"record_offset":540624,"parent_record_offset":540600,"ancestor_record_offset":540000,"inlinee":5079,"source_line":3988,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1643,"record_offset":540176,"parent_record_offset":540132,"ancestor_record_offset":540000,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1643,"record_offset":540204,"parent_record_offset":540176,"ancestor_record_offset":540000,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1643,"record_offset":540068,"parent_record_offset":540036,"ancestor_record_offset":540000,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":1643,"record_offset":540600,"parent_record_offset":540132,"ancestor_record_offset":540000,"inlinee":5078,"source_line":26,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":1643,"record_offset":540100,"parent_record_offset":540068,"ancestor_record_offset":540000,"inlinee":6133,"source_line":32,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":1643,"record_offset":540132,"parent_record_offset":540100,"ancestor_record_offset":540000,"inlinee":6135,"source_line":50,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":1643,"record_offset":540348,"parent_record_offset":540324,"ancestor_record_offset":540000,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1643,"record_offset":540428,"parent_record_offset":540372,"ancestor_record_offset":540000,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1643,"record_offset":540648,"parent_record_offset":540624,"ancestor_record_offset":540000,"inlinee":5042,"source_line":835,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1643,"record_offset":540672,"parent_record_offset":540648,"ancestor_record_offset":540000,"inlinee":8052,"source_line":841,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1643,"record_offset":540372,"parent_record_offset":540348,"ancestor_record_offset":540000,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1643,"record_offset":540400,"parent_record_offset":540372,"ancestor_record_offset":540000,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1643,"record_offset":540776,"parent_record_offset":540752,"ancestor_record_offset":540000,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":1643,"record_offset":540752,"parent_record_offset":540728,"ancestor_record_offset":540000,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::history::replay_wall::impl$0::new::closure$0::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 189
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1439B
// 首条位置: {"module":1682,"inlinee":82843}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::sync::impl$32::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":1682,"record_offset":127992,"parent_record_offset":127936,"ancestor_record_offset":127936,"inlinee":5906,"source_line":2376,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::abs
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1682,"record_offset":128108,"parent_record_offset":127936,"ancestor_record_offset":127936,"inlinee":11731,"source_line":1569,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1682,"record_offset":128136,"parent_record_offset":127936,"ancestor_record_offset":127936,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::length_ws
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1682,"record_offset":128080,"parent_record_offset":127936,"ancestor_record_offset":127936,"inlinee":9818,"source_line":169,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1682,"record_offset":128020,"parent_record_offset":127992,"ancestor_record_offset":127936,"inlinee":5322,"source_line":3134,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1682,"record_offset":128044,"parent_record_offset":128020,"ancestor_record_offset":127936,"inlinee":4153,"source_line":3919,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::history::replay_wall::WallReplayEditCmd::process<country_core::systems::history::replay_wall::replay_wall::closure_env$0,country_core::systems::history::replay_wall::replay_wall::closure_env$1>
// 记录形式: compiled_procedure; instances=1
// 源码行: 294
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1720,"record_offset":128652,"rva":27132048,"va":5395841168}
// 编译实例 RVAs: 0x19e0090
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=6
//     evidence={"assembly":"\tcallq\t-20168996","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395841168,"raw_hex":"e8dc3eccfe","size":5,"soft_fail":false,"target_va":5375678704,"va":5395847695,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140081bd0
//     owner=unresolved_namespace; count=6
//     target_aliases=11 candidates; complete list: replay_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-26603052","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395841168,"raw_hex":"e8d4116afe","size":5,"soft_fail":false,"target_va":5369240528,"va":5395843575,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c02660
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: replay_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14541943","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395841168,"raw_hex":"e8891b22ff","size":5,"soft_fail":false,"target_va":5381301856,"va":5395843794,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c71040
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: replay_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14087404","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395841168,"raw_hex":"e8140b29ff","size":5,"soft_fail":false,"target_va":5381754944,"va":5395842343,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df0560
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: replay_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12517283","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395841168,"raw_hex":"e85d0041ff","size":5,"soft_fail":false,"target_va":5383325024,"va":5395842302,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1414786b0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: replay_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5670168","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395841168,"raw_hex":"e8e87aa9ff","size":5,"soft_fail":false,"target_va":5390173872,"va":5395844035,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1414bbc30
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: replay_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5393491","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395841168,"raw_hex":"e8adb3adff","size":5,"soft_fail":false,"target_va":5390449712,"va":5395843198,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1415f7730
//     owner=unresolved_namespace; count=6
//     target_aliases=2 candidates; complete list: replay_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4098716","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395841168,"raw_hex":"e86475c1ff","size":5,"soft_fail":false,"target_va":5391742768,"va":5395841479,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1415f84e0
//     owner=unresolved_namespace; count=6
//     target_aliases=2 candidates; complete list: replay_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4095228","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395841168,"raw_hex":"e80483c1ff","size":5,"soft_fail":false,"target_va":5391746272,"va":5395841495,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14196dd90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-471907","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395841168,"raw_hex":"e89dccf8ff","size":5,"soft_fail":false,"target_va":5395373456,"va":5395845358,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14196e3d0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: replay_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-468872","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395841168,"raw_hex":"e878d8f8ff","size":5,"soft_fail":false,"target_va":5395375056,"va":5395843923,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14197bb70
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: replay_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-414163","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395841168,"raw_hex":"e82daef9ff","size":5,"soft_fail":false,"target_va":5395430256,"va":5395844414,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1419af940
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: replay_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-203528","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395841168,"raw_hex":"e8f8e4fcff","size":5,"soft_fail":false,"target_va":5395642688,"va":5395846211,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1419afc50
//     owner=unresolved_namespace; count=7
//     target_aliases=5 candidates; complete list: replay_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-199385","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395841168,"raw_hex":"e827f5fcff","size":5,"soft_fail":false,"target_va":5395643472,"va":5395842852,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f2b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t20507984","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395841168,"raw_hex":"e850ed3801","size":5,"soft_fail":false,"target_va":5416350384,"va":5395842395,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f390
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t20508595","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395841168,"raw_hex":"e8b3ef3801","size":5,"soft_fail":false,"target_va":5416350608,"va":5395842008,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t20774753","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395841168,"raw_hex":"e861ff3c01","size":5,"soft_fail":false,"target_va":5416622208,"va":5395847450,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t20774737","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395841168,"raw_hex":"e851ff3c01","size":5,"soft_fail":false,"target_va":5416622240,"va":5395847498,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::history::wall_compound_edit::panic_deprecated_operation
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t20542915","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395841168,"raw_hex":"e8c3753901","size":5,"soft_fail":false,"target_va":5416390448,"va":5395847528,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::inner_walls::InnerWalls::get_unwrapped
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-14541633","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395841168,"raw_hex":"e8bf1c22ff","size":5,"soft_fail":false,"target_va":5381302272,"va":5395843900,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::inner_walls::InnerWalls::new_wall_with_id
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-14543317","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395841168,"raw_hex":"e82b1622ff","size":5,"soft_fail":false,"target_va":5381303328,"va":5395846640,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::inner_walls::InnerWalls::remove
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-14533473","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395841168,"raw_hex":"e89f3c22ff","size":5,"soft_fail":false,"target_va":5381310080,"va":5395843548,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::inner_walls::InnerWalls::update_arch_height
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-14536763","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395841168,"raw_hex":"e8c52f22ff","size":5,"soft_fail":false,"target_va":5381305056,"va":5395841814,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::inner_walls::InnerWalls::update_arch_type
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-14539765","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395841168,"raw_hex":"e80b2422ff","size":5,"soft_fail":false,"target_va":5381303472,"va":5395843232,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::inner_walls::InnerWalls::update_board_layout
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-14537967","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395841168,"raw_hex":"e8112b22ff","size":5,"soft_fail":false,"target_va":5381305920,"va":5395843882,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::inner_walls::InnerWalls::update_elevation
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-14538010","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395841168,"raw_hex":"e8e62a22ff","size":5,"soft_fail":false,"target_va":5381303888,"va":5395841893,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::inner_walls::InnerWalls::update_freehand_ty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-14537387","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395841168,"raw_hex":"e8552d22ff","size":5,"soft_fail":false,"target_va":5381305504,"va":5395842886,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::inner_walls::InnerWalls::update_material
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-14541269","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395841168,"raw_hex":"e82b1e22ff","size":5,"soft_fail":false,"target_va":5381302496,"va":5395843760,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::inner_walls::InnerWalls::update_plaster_peel
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-14536587","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395841168,"raw_hex":"e8753022ff","size":5,"soft_fail":false,"target_va":5381306336,"va":5395842918,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::inner_walls::InnerWalls::update_supports
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-14540416","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395841168,"raw_hex":"e8802122ff","size":5,"soft_fail":false,"target_va":5381302912,"va":5395843323,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::inner_walls::InnerWalls::update_wall_state
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-14540788","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395841168,"raw_hex":"e80c2022ff","size":5,"soft_fail":false,"target_va":5381304336,"va":5395845119,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::inner_walls::InnerWalls::update_wall_state_curve
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-14534999","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395841168,"raw_hex":"e8a93622ff","size":5,"soft_fail":false,"target_va":5381307200,"va":5395842194,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::inner_walls::InnerWalls::update_wall_state_enclosure
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-14534315","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395841168,"raw_hex":"e8553922ff","size":5,"soft_fail":false,"target_va":5381308304,"va":5395842614,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::history::replay_wall::_deprecated_morph_curve
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-123942","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395841168,"raw_hex":"e8da1bfeff","size":5,"soft_fail":false,"target_va":5395717776,"va":5395841713,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::history::replay_wall::replay_wall::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-4501","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395841168,"raw_hex":"e86beeffff","size":5,"soft_fail":false,"target_va":5395841024,"va":5395845520,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::history::replay_wall::rigid_transform_curve
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-125489","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395841168,"raw_hex":"e8cf15feff","size":5,"soft_fail":false,"target_va":5395717296,"va":5395842780,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui_wall_transform_gizmos::RigidXZTransform::interpolated
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t891052","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395841168,"raw_hex":"e8ac980d00","size":5,"soft_fail":false,"target_va":5396733216,"va":5395842159,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui_wall_transform_gizmos::RigidXZTransform::transform_curve
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t891319","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395841168,"raw_hex":"e8b7990d00","size":5,"soft_fail":false,"target_va":5396733504,"va":5395842180,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::valid_enclosure::ShapeParameters::as_curve2
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-16245613","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395841168,"raw_hex":"e8931c08ff","size":5,"soft_fail":false,"target_va":5379597392,"va":5395843000,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::circular_f32::CircularF32::lerp
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2230153","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395841168,"raw_hex":"e889072200","size":5,"soft_fail":false,"target_va":5398072720,"va":5395842562,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::easing_functions::EasingFunction::apply
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-12548396","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395841168,"raw_hex":"e8d48640ff","size":5,"soft_fail":false,"target_va":5383297200,"va":5395845591,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::easing_functions::cubic_in_out
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-12544278","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395841168,"raw_hex":"e8ea9640ff","size":5,"soft_fail":false,"target_va":5383297104,"va":5395841377,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::circle::Circle2d::as_curve2
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-12536142","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395841168,"raw_hex":"e8b2b640ff","size":5,"soft_fail":false,"target_va":5383306512,"va":5395842649,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::circle::Circle2d::lerp
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-12536503","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395841168,"raw_hex":"e849b540ff","size":5,"soft_fail":false,"target_va":5383306032,"va":5395842530,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::as_curve2
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-12521007","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395841168,"raw_hex":"e8d1f140ff","size":5,"soft_fail":false,"target_va":5383321632,"va":5395842634,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::lerp
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-12521665","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395841168,"raw_hex":"e83fef40ff","size":5,"soft_fail":false,"target_va":5383320832,"va":5395842492,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1720,"record_offset":128960,"parent_record_offset":128652,"ancestor_record_offset":128652,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/01720.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::history::replay_wall::WallReplayEditCmd::process_animated
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_wall.rs
//     evidence={"module":1720,"record_offset":128992,"parent_record_offset":128652,"ancestor_record_offset":128652,"inlinee":83369,"source_line":718,"archive":"_pdb/C/modules/01720.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::history::replay_wall::WallReplayEditCmd::process_instant
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_wall.rs
//     evidence={"module":1720,"record_offset":132192,"parent_record_offset":128652,"ancestor_record_offset":128652,"inlinee":83370,"source_line":337,"archive":"_pdb/C/modules/01720.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1720,"record_offset":129700,"parent_record_offset":129676,"ancestor_record_offset":128652,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01720.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1720,"record_offset":129724,"parent_record_offset":129700,"ancestor_record_offset":128652,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01720.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1720,"record_offset":129748,"parent_record_offset":129724,"ancestor_record_offset":128652,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01720.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1720,"record_offset":129676,"parent_record_offset":129584,"ancestor_record_offset":128652,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01720.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1720,"record_offset":130272,"parent_record_offset":130240,"ancestor_record_offset":128652,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/01720.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1720,"record_offset":129612,"parent_record_offset":129584,"ancestor_record_offset":128652,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01720.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1720,"record_offset":129584,"parent_record_offset":129560,"ancestor_record_offset":128652,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01720.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1720,"record_offset":130320,"parent_record_offset":130296,"ancestor_record_offset":128652,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/01720.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1720,"record_offset":130296,"parent_record_offset":130272,"ancestor_record_offset":128652,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/01720.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1720,"record_offset":129560,"parent_record_offset":129536,"ancestor_record_offset":128652,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01720.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1720,"record_offset":130240,"parent_record_offset":130200,"ancestor_record_offset":128652,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/01720.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1720,"record_offset":130168,"parent_record_offset":130132,"ancestor_record_offset":128652,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/01720.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1720,"record_offset":130200,"parent_record_offset":130168,"ancestor_record_offset":128652,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/01720.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":1720,"record_offset":130104,"parent_record_offset":130068,"ancestor_record_offset":128652,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/01720.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":1720,"record_offset":130068,"parent_record_offset":130044,"ancestor_record_offset":128652,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/01720.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::message_writer::MessageWriter::write
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_writer.rs
//     evidence={"module":1720,"record_offset":130044,"parent_record_offset":128992,"ancestor_record_offset":128652,"inlinee":11206,"source_line":69,"archive":"_pdb/C/modules/01720.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::messages::Messages::write_with_caller
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs
//     evidence={"module":1720,"record_offset":130132,"parent_record_offset":130068,"ancestor_record_offset":128652,"inlinee":8681,"source_line":129,"archive":"_pdb/C/modules/01720.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1720,"record_offset":130900,"parent_record_offset":128992,"ancestor_record_offset":128652,"inlinee":4487,"source_line":264,"archive":"_pdb/C/modules/01720.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1720,"record_offset":129644,"parent_record_offset":129612,"ancestor_record_offset":128652,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/01720.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1720,"record_offset":131528,"parent_record_offset":131420,"ancestor_record_offset":128652,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/01720.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1720,"record_offset":131472,"parent_record_offset":131420,"ancestor_record_offset":128652,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/01720.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1720,"record_offset":129868,"parent_record_offset":129844,"ancestor_record_offset":128652,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/01720.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1720,"record_offset":132012,"parent_record_offset":128992,"ancestor_record_offset":128652,"inlinee":7434,"source_line":2119,"archive":"_pdb/C/modules/01720.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1720,"record_offset":134748,"parent_record_offset":132192,"ancestor_record_offset":128652,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/01720.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$6::clone
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1720,"record_offset":132724,"parent_record_offset":132192,"ancestor_record_offset":128652,"inlinee":5739,"source_line":2261,"archive":"_pdb/C/modules/01720.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1720,"record_offset":129512,"parent_record_offset":129472,"ancestor_record_offset":128652,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01720.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1720,"record_offset":130360,"parent_record_offset":130200,"ancestor_record_offset":128652,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/01720.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::last
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1720,"record_offset":131984,"parent_record_offset":128992,"ancestor_record_offset":128652,"inlinee":5756,"source_line":281,"archive":"_pdb/C/modules/01720.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::inner_wall_state::WallState::_deprecated_empty_w_height
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":1720,"record_offset":134808,"parent_record_offset":134784,"ancestor_record_offset":128652,"inlinee":38324,"source_line":293,"archive":"_pdb/C/modules/01720.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::inner_wall_state::impl$45::clone
//     owner=private_source_ownership_candidate; count=8
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":1720,"record_offset":129408,"parent_record_offset":128992,"ancestor_record_offset":128652,"inlinee":30580,"source_line":115,"archive":"_pdb/C/modules/01720.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::history::replay_wall::impl$1::process_instant::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_wall.rs
//     evidence={"module":1720,"record_offset":134940,"parent_record_offset":134904,"ancestor_record_offset":128652,"inlinee":83377,"source_line":388,"archive":"_pdb/C/modules/01720.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::history::replay_wall::impl$1::process_instant::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_wall.rs
//     evidence={"module":1720,"record_offset":134784,"parent_record_offset":134748,"ancestor_record_offset":128652,"inlinee":83376,"source_line":588,"archive":"_pdb/C/modules/01720.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::history::replay_wall::replay_wall::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_wall.rs
//     evidence={"module":1720,"record_offset":130496,"parent_record_offset":128992,"ancestor_record_offset":128652,"inlinee":83373,"source_line":82,"archive":"_pdb/C/modules/01720.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::history::replay_wall::replay_wall::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_wall.rs
//     evidence={"module":1720,"record_offset":133664,"parent_record_offset":132192,"ancestor_record_offset":128652,"inlinee":83374,"source_line":87,"archive":"_pdb/C/modules/01720.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::history::replay_wall::rigid_transform_curve
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_wall.rs
//     evidence={"module":1720,"record_offset":130408,"parent_record_offset":128992,"ancestor_record_offset":128652,"inlinee":83346,"source_line":1019,"archive":"_pdb/C/modules/01720.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui_wall_transform_gizmos::RigidXZTransform::from_affine
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui_wall_transform_gizmos.rs
//     evidence={"module":1720,"record_offset":130436,"parent_record_offset":130408,"ancestor_record_offset":128652,"inlinee":83371,"source_line":92,"archive":"_pdb/C/modules/01720.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::valid_enclosure::ShapeParameters::as_curve2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/valid_enclosure/mod.rs
//     evidence={"module":1720,"record_offset":130956,"parent_record_offset":128992,"ancestor_record_offset":128652,"inlinee":33089,"source_line":104,"archive":"_pdb/C/modules/01720.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::valid_enclosure::ShapeParameters::center
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/valid_enclosure/mod.rs
//     evidence={"module":1720,"record_offset":130468,"parent_record_offset":128992,"ancestor_record_offset":128652,"inlinee":29289,"source_line":82,"archive":"_pdb/C/modules/01720.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::valid_enclosure::impl$3::clone
//     owner=private_source_ownership_candidate; count=9
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/valid_enclosure/mod.rs
//     evidence={"module":1720,"record_offset":129808,"parent_record_offset":129408,"ancestor_record_offset":128652,"inlinee":30582,"source_line":11,"archive":"_pdb/C/modules/01720.jsonl.gz"}
//   [pdb_inline_transitive] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":1720,"record_offset":129844,"parent_record_offset":128992,"ancestor_record_offset":128652,"inlinee":8968,"source_line":171,"archive":"_pdb/C/modules/01720.jsonl.gz"}
//   [pdb_inline_transitive] utils::circular_f32::impl$7::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/circular_f32.rs
//     evidence={"module":1720,"record_offset":130924,"parent_record_offset":130900,"ancestor_record_offset":128652,"inlinee":10600,"source_line":7,"archive":"_pdb/C/modules/01720.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::_deprecated_new_empty
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1720,"record_offset":134836,"parent_record_offset":134808,"ancestor_record_offset":128652,"inlinee":38303,"source_line":176,"archive":"_pdb/C/modules/01720.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::u
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1720,"record_offset":131560,"parent_record_offset":128992,"ancestor_record_offset":128652,"inlinee":9861,"source_line":125,"archive":"_pdb/C/modules/01720.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::impl$3::default
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1720,"record_offset":134860,"parent_record_offset":134836,"ancestor_record_offset":128652,"inlinee":38305,"source_line":74,"archive":"_pdb/C/modules/01720.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::impl$45::clone
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1720,"record_offset":129472,"parent_record_offset":129408,"ancestor_record_offset":128652,"inlinee":12078,"source_line":61,"archive":"_pdb/C/modules/01720.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::history::replay_wall::WallReplayEditCmd::process_instant
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 337
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x145AA
// 首条位置: {"module":1720,"inlinee":83370}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1720,"record_offset":134748,"parent_record_offset":132192,"ancestor_record_offset":132192,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/01720.jsonl.gz"}
//   [pdb_inline_direct] core::option::impl$6::clone
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1720,"record_offset":132724,"parent_record_offset":132192,"ancestor_record_offset":132192,"inlinee":5739,"source_line":2261,"archive":"_pdb/C/modules/01720.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1720,"record_offset":133196,"parent_record_offset":132192,"ancestor_record_offset":132192,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01720.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::inner_wall_state::impl$45::clone
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":1720,"record_offset":133224,"parent_record_offset":132192,"ancestor_record_offset":132192,"inlinee":30580,"source_line":115,"archive":"_pdb/C/modules/01720.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::history::replay_wall::replay_wall::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_wall.rs
//     evidence={"module":1720,"record_offset":134708,"parent_record_offset":132192,"ancestor_record_offset":132192,"inlinee":83373,"source_line":82,"archive":"_pdb/C/modules/01720.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::history::replay_wall::replay_wall::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_wall.rs
//     evidence={"module":1720,"record_offset":133664,"parent_record_offset":132192,"ancestor_record_offset":132192,"inlinee":83374,"source_line":87,"archive":"_pdb/C/modules/01720.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::wall::valid_enclosure::impl$3::clone
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/valid_enclosure/mod.rs
//     evidence={"module":1720,"record_offset":132696,"parent_record_offset":132192,"ancestor_record_offset":132192,"inlinee":30582,"source_line":11,"archive":"_pdb/C/modules/01720.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1720,"record_offset":133048,"parent_record_offset":133024,"ancestor_record_offset":132192,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01720.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1720,"record_offset":133072,"parent_record_offset":133048,"ancestor_record_offset":132192,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01720.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1720,"record_offset":133096,"parent_record_offset":133072,"ancestor_record_offset":132192,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01720.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1720,"record_offset":133024,"parent_record_offset":132932,"ancestor_record_offset":132192,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01720.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1720,"record_offset":132960,"parent_record_offset":132932,"ancestor_record_offset":132192,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01720.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1720,"record_offset":132932,"parent_record_offset":132908,"ancestor_record_offset":132192,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01720.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1720,"record_offset":132908,"parent_record_offset":132884,"ancestor_record_offset":132192,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01720.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1720,"record_offset":132992,"parent_record_offset":132960,"ancestor_record_offset":132192,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/01720.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1720,"record_offset":132860,"parent_record_offset":132820,"ancestor_record_offset":132192,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01720.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::inner_wall_state::WallState::_deprecated_empty_w_height
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":1720,"record_offset":134808,"parent_record_offset":134784,"ancestor_record_offset":132192,"inlinee":38324,"source_line":293,"archive":"_pdb/C/modules/01720.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::inner_wall_state::impl$45::clone
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":1720,"record_offset":132764,"parent_record_offset":132724,"ancestor_record_offset":132192,"inlinee":30580,"source_line":115,"archive":"_pdb/C/modules/01720.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::history::replay_wall::impl$1::process_instant::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_wall.rs
//     evidence={"module":1720,"record_offset":134940,"parent_record_offset":134904,"ancestor_record_offset":132192,"inlinee":83377,"source_line":388,"archive":"_pdb/C/modules/01720.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::history::replay_wall::impl$1::process_instant::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_wall.rs
//     evidence={"module":1720,"record_offset":134784,"parent_record_offset":134748,"ancestor_record_offset":132192,"inlinee":83376,"source_line":588,"archive":"_pdb/C/modules/01720.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::valid_enclosure::impl$3::clone
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/valid_enclosure/mod.rs
//     evidence={"module":1720,"record_offset":133156,"parent_record_offset":132764,"ancestor_record_offset":132192,"inlinee":30582,"source_line":11,"archive":"_pdb/C/modules/01720.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::_deprecated_new_empty
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1720,"record_offset":134836,"parent_record_offset":134808,"ancestor_record_offset":132192,"inlinee":38303,"source_line":176,"archive":"_pdb/C/modules/01720.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::impl$3::default
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1720,"record_offset":134860,"parent_record_offset":134836,"ancestor_record_offset":132192,"inlinee":38305,"source_line":74,"archive":"_pdb/C/modules/01720.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::impl$45::clone
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1720,"record_offset":132820,"parent_record_offset":132764,"ancestor_record_offset":132192,"inlinee":12078,"source_line":61,"archive":"_pdb/C/modules/01720.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::history::replay_wall::impl$1::process_instant::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 388
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x145B1
// 首条位置: {"module":1720,"inlinee":83377}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::resources::walls::inner_wall_state::WallState::_deprecated_empty_w_height
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":1720,"record_offset":134964,"parent_record_offset":134940,"ancestor_record_offset":134940,"inlinee":38324,"source_line":293,"archive":"_pdb/C/modules/01720.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::_deprecated_new_empty
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1720,"record_offset":134992,"parent_record_offset":134964,"ancestor_record_offset":134940,"inlinee":38303,"source_line":176,"archive":"_pdb/C/modules/01720.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::impl$3::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1720,"record_offset":135016,"parent_record_offset":134992,"ancestor_record_offset":134940,"inlinee":38305,"source_line":74,"archive":"_pdb/C/modules/01720.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::history::replay_wall::impl$1::process_instant::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 588
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x145B0
// 首条位置: {"module":1720,"inlinee":83376}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::resources::walls::inner_wall_state::WallState::_deprecated_empty_w_height
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":1720,"record_offset":134808,"parent_record_offset":134784,"ancestor_record_offset":134784,"inlinee":38324,"source_line":293,"archive":"_pdb/C/modules/01720.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::_deprecated_new_empty
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1720,"record_offset":134836,"parent_record_offset":134808,"ancestor_record_offset":134784,"inlinee":38303,"source_line":176,"archive":"_pdb/C/modules/01720.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::impl$3::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1720,"record_offset":134860,"parent_record_offset":134836,"ancestor_record_offset":134784,"inlinee":38305,"source_line":74,"archive":"_pdb/C/modules/01720.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::history::replay_wall::WallReplayEditCmd::process_animated
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 718
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x145A9
// 首条位置: {"module":1720,"inlinee":83369}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::message::message_writer::MessageWriter::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_writer.rs
//     evidence={"module":1720,"record_offset":130044,"parent_record_offset":128992,"ancestor_record_offset":128992,"inlinee":11206,"source_line":69,"archive":"_pdb/C/modules/01720.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1720,"record_offset":130900,"parent_record_offset":128992,"ancestor_record_offset":128992,"inlinee":4487,"source_line":264,"archive":"_pdb/C/modules/01720.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1720,"record_offset":132012,"parent_record_offset":128992,"ancestor_record_offset":128992,"inlinee":7434,"source_line":2119,"archive":"_pdb/C/modules/01720.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::last
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1720,"record_offset":131984,"parent_record_offset":128992,"ancestor_record_offset":128992,"inlinee":5756,"source_line":281,"archive":"_pdb/C/modules/01720.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::inner_wall_state::impl$45::clone
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":1720,"record_offset":129408,"parent_record_offset":128992,"ancestor_record_offset":128992,"inlinee":30580,"source_line":115,"archive":"_pdb/C/modules/01720.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::history::replay_wall::replay_wall::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_wall.rs
//     evidence={"module":1720,"record_offset":130496,"parent_record_offset":128992,"ancestor_record_offset":128992,"inlinee":83373,"source_line":82,"archive":"_pdb/C/modules/01720.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::history::replay_wall::rigid_transform_curve
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_wall.rs
//     evidence={"module":1720,"record_offset":130408,"parent_record_offset":128992,"ancestor_record_offset":128992,"inlinee":83346,"source_line":1019,"archive":"_pdb/C/modules/01720.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::wall::valid_enclosure::ShapeParameters::as_curve2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/valid_enclosure/mod.rs
//     evidence={"module":1720,"record_offset":130956,"parent_record_offset":128992,"ancestor_record_offset":128992,"inlinee":33089,"source_line":104,"archive":"_pdb/C/modules/01720.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::wall::valid_enclosure::ShapeParameters::center
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/valid_enclosure/mod.rs
//     evidence={"module":1720,"record_offset":130468,"parent_record_offset":128992,"ancestor_record_offset":128992,"inlinee":29289,"source_line":82,"archive":"_pdb/C/modules/01720.jsonl.gz"}
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":1720,"record_offset":129844,"parent_record_offset":128992,"ancestor_record_offset":128992,"inlinee":8968,"source_line":171,"archive":"_pdb/C/modules/01720.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::u
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1720,"record_offset":131560,"parent_record_offset":128992,"ancestor_record_offset":128992,"inlinee":9861,"source_line":125,"archive":"_pdb/C/modules/01720.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1720,"record_offset":129700,"parent_record_offset":129676,"ancestor_record_offset":128992,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01720.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1720,"record_offset":129724,"parent_record_offset":129700,"ancestor_record_offset":128992,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01720.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1720,"record_offset":129748,"parent_record_offset":129724,"ancestor_record_offset":128992,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01720.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1720,"record_offset":129676,"parent_record_offset":129584,"ancestor_record_offset":128992,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01720.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1720,"record_offset":130272,"parent_record_offset":130240,"ancestor_record_offset":128992,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/01720.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1720,"record_offset":129612,"parent_record_offset":129584,"ancestor_record_offset":128992,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01720.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1720,"record_offset":129584,"parent_record_offset":129560,"ancestor_record_offset":128992,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01720.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1720,"record_offset":130320,"parent_record_offset":130296,"ancestor_record_offset":128992,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/01720.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1720,"record_offset":130296,"parent_record_offset":130272,"ancestor_record_offset":128992,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/01720.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1720,"record_offset":129560,"parent_record_offset":129536,"ancestor_record_offset":128992,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01720.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1720,"record_offset":130240,"parent_record_offset":130200,"ancestor_record_offset":128992,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/01720.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1720,"record_offset":130168,"parent_record_offset":130132,"ancestor_record_offset":128992,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/01720.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1720,"record_offset":130200,"parent_record_offset":130168,"ancestor_record_offset":128992,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/01720.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":1720,"record_offset":130104,"parent_record_offset":130068,"ancestor_record_offset":128992,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/01720.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":1720,"record_offset":130068,"parent_record_offset":130044,"ancestor_record_offset":128992,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/01720.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::message_writer::MessageWriter::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_writer.rs
//     evidence={"module":1720,"record_offset":130532,"parent_record_offset":130496,"ancestor_record_offset":128992,"inlinee":11206,"source_line":69,"archive":"_pdb/C/modules/01720.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::messages::Messages::write_with_caller
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs
//     evidence={"module":1720,"record_offset":130132,"parent_record_offset":130068,"ancestor_record_offset":128992,"inlinee":8681,"source_line":129,"archive":"_pdb/C/modules/01720.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1720,"record_offset":129644,"parent_record_offset":129612,"ancestor_record_offset":128992,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/01720.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1720,"record_offset":131528,"parent_record_offset":131420,"ancestor_record_offset":128992,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/01720.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1720,"record_offset":131472,"parent_record_offset":131420,"ancestor_record_offset":128992,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/01720.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1720,"record_offset":129868,"parent_record_offset":129844,"ancestor_record_offset":128992,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/01720.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1720,"record_offset":129512,"parent_record_offset":129472,"ancestor_record_offset":128992,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01720.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1720,"record_offset":130360,"parent_record_offset":130200,"ancestor_record_offset":128992,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/01720.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui_wall_transform_gizmos::RigidXZTransform::from_affine
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui_wall_transform_gizmos.rs
//     evidence={"module":1720,"record_offset":130436,"parent_record_offset":130408,"ancestor_record_offset":128992,"inlinee":83371,"source_line":92,"archive":"_pdb/C/modules/01720.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::valid_enclosure::impl$3::clone
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/valid_enclosure/mod.rs
//     evidence={"module":1720,"record_offset":129808,"parent_record_offset":129408,"ancestor_record_offset":128992,"inlinee":30582,"source_line":11,"archive":"_pdb/C/modules/01720.jsonl.gz"}
//   [pdb_inline_transitive] utils::circular_f32::impl$7::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/circular_f32.rs
//     evidence={"module":1720,"record_offset":130924,"parent_record_offset":130900,"ancestor_record_offset":128992,"inlinee":10600,"source_line":7,"archive":"_pdb/C/modules/01720.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::impl$45::clone
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1720,"record_offset":129472,"parent_record_offset":129408,"ancestor_record_offset":128992,"inlinee":12078,"source_line":61,"archive":"_pdb/C/modules/01720.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::history::replay_wall::_deprecated_morph_curve
// 记录形式: compiled_procedure; instances=1
// 源码行: 1004
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1715,"record_offset":16120,"rva":27008656,"va":5395717776}
// 编译实例 RVAs: 0x19c1e90
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-20039291","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395717776,"raw_hex":"e88539cefe","size":5,"soft_fail":false,"target_va":5375678688,"va":5395717974,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-20039736","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395717776,"raw_hex":"e8c837cefe","size":5,"soft_fail":false,"target_va":5375678704,"va":5395718435,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=2
//     target_aliases=1699 candidates; complete list: replay_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-26995768","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395717776,"raw_hex":"e8c81364fe","size":5,"soft_fail":false,"target_va":5368722192,"va":5395717955,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14196dc60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-344772","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395717776,"raw_hex":"e83cbdfaff","size":5,"soft_fail":false,"target_va":5395373152,"va":5395717919,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: replay_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t21339044","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395717776,"raw_hex":"e8a49b4501","size":5,"soft_fail":false,"target_va":5417057056,"va":5395718007,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t20902900","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395717776,"raw_hex":"e8f4f33e01","size":5,"soft_fail":false,"target_va":5416620794,"va":5395717889,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::inner_walls::InnerWalls::update_wall_state_curve
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-14411152","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395717776,"raw_hex":"e8701a24ff","size":5,"soft_fail":false,"target_va":5381307200,"va":5395718347,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] utils::curve::impl$45::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1715,"record_offset":16272,"parent_record_offset":16120,"ancestor_record_offset":16120,"inlinee":12078,"source_line":61,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1715,"record_offset":16756,"parent_record_offset":16732,"ancestor_record_offset":16120,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1715,"record_offset":16780,"parent_record_offset":16756,"ancestor_record_offset":16120,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1715,"record_offset":17892,"parent_record_offset":17868,"ancestor_record_offset":16120,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1715,"record_offset":17916,"parent_record_offset":17892,"ancestor_record_offset":16120,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1715,"record_offset":16804,"parent_record_offset":16780,"ancestor_record_offset":16120,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1715,"record_offset":17940,"parent_record_offset":17916,"ancestor_record_offset":16120,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1715,"record_offset":16732,"parent_record_offset":16552,"ancestor_record_offset":16120,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1715,"record_offset":17868,"parent_record_offset":17780,"ancestor_record_offset":16120,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1715,"record_offset":16492,"parent_record_offset":16464,"ancestor_record_offset":16120,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1715,"record_offset":17808,"parent_record_offset":17780,"ancestor_record_offset":16120,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1715,"record_offset":17780,"parent_record_offset":17756,"ancestor_record_offset":16120,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1715,"record_offset":16552,"parent_record_offset":16520,"ancestor_record_offset":16120,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1715,"record_offset":16520,"parent_record_offset":16492,"ancestor_record_offset":16120,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1715,"record_offset":17756,"parent_record_offset":17732,"ancestor_record_offset":16120,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1715,"record_offset":16588,"parent_record_offset":16552,"ancestor_record_offset":16120,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1715,"record_offset":17020,"parent_record_offset":16956,"ancestor_record_offset":16120,"inlinee":8468,"source_line":410,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1715,"record_offset":16424,"parent_record_offset":16356,"ancestor_record_offset":16120,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1715,"record_offset":16396,"parent_record_offset":16356,"ancestor_record_offset":16120,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1715,"record_offset":16464,"parent_record_offset":16424,"ancestor_record_offset":16120,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::clone
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1715,"record_offset":16324,"parent_record_offset":16272,"ancestor_record_offset":16120,"inlinee":5763,"source_line":3747,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1715,"record_offset":16356,"parent_record_offset":16324,"ancestor_record_offset":16120,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":1715,"record_offset":16612,"parent_record_offset":16588,"ancestor_record_offset":16120,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1715,"record_offset":16692,"parent_record_offset":16636,"ancestor_record_offset":16120,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1715,"record_offset":17512,"parent_record_offset":17464,"ancestor_record_offset":16120,"inlinee":5459,"source_line":79,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::take::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\take.rs
//     evidence={"module":1715,"record_offset":17464,"parent_record_offset":17020,"ancestor_record_offset":16120,"inlinee":8469,"source_line":36,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_uninit.rs
//     evidence={"module":1715,"record_offset":17664,"parent_record_offset":17020,"ancestor_record_offset":16120,"inlinee":4509,"source_line":563,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1715,"record_offset":16636,"parent_record_offset":16612,"ancestor_record_offset":16120,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1715,"record_offset":16664,"parent_record_offset":16636,"ancestor_record_offset":16120,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1715,"record_offset":17836,"parent_record_offset":17808,"ancestor_record_offset":16120,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1715,"record_offset":16860,"parent_record_offset":16424,"ancestor_record_offset":16120,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1715,"record_offset":16884,"parent_record_offset":16860,"ancestor_record_offset":16120,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1715,"record_offset":17708,"parent_record_offset":16272,"ancestor_record_offset":16120,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1715,"record_offset":17624,"parent_record_offset":17556,"ancestor_record_offset":16120,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1715,"record_offset":17596,"parent_record_offset":17556,"ancestor_record_offset":16120,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1715,"record_offset":17556,"parent_record_offset":17512,"ancestor_record_offset":16120,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::history::replay_wall::rigid_transform_curve
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 1019
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x14592
// 首条位置: {"module":1715,"record_offset":15096,"rva":27008176,"va":5395717296}
// 编译实例 RVAs: 0x19c1cb0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bd40
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: replay_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t21340144","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395717296,"raw_hex":"e8f09f4501","size":5,"soft_fail":false,"target_va":5417057600,"va":5395717451,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bda0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: replay_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t21340212","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395717296,"raw_hex":"e834a04501","size":5,"soft_fail":false,"target_va":5417057696,"va":5395717479,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bef0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: replay_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t21340560","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395717296,"raw_hex":"e890a14501","size":5,"soft_fail":false,"target_va":5417058032,"va":5395717467,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::inner_walls::InnerWalls::update_wall_state_curve
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-14410503","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395717296,"raw_hex":"e8f91c24ff","size":5,"soft_fail":false,"target_va":5381307200,"va":5395717698,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui_wall_transform_gizmos::RigidXZTransform::transform_curve
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1015815","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395717296,"raw_hex":"e807800f00","size":5,"soft_fail":false,"target_va":5396733504,"va":5395717684,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::systems::ui_wall_transform_gizmos::RigidXZTransform::from_affine
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui_wall_transform_gizmos.rs
//     evidence={"module":1720,"record_offset":130436,"parent_record_offset":130408,"ancestor_record_offset":130408,"inlinee":83371,"source_line":92,"archive":"_pdb/C/modules/01720.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui_wall_transform_gizmos::RigidXZTransform::interpolated
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui_wall_transform_gizmos.rs
//     evidence={"module":1715,"record_offset":15248,"parent_record_offset":15096,"ancestor_record_offset":15096,"inlinee":83335,"source_line":118,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1715,"record_offset":15676,"parent_record_offset":15620,"ancestor_record_offset":15096,"inlinee":7190,"source_line":87,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_set_ps
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1715,"record_offset":15648,"parent_record_offset":15620,"ancestor_record_offset":15096,"inlinee":27811,"source_line":974,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1715,"record_offset":15736,"parent_record_offset":15712,"ancestor_record_offset":15096,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1715,"record_offset":15940,"parent_record_offset":15916,"ancestor_record_offset":15096,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1715,"record_offset":15884,"parent_record_offset":15860,"ancestor_record_offset":15096,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui_wall_transform_gizmos::RigidXZTransform::identity
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui_wall_transform_gizmos.rs
//     evidence={"module":1715,"record_offset":16084,"parent_record_offset":15248,"ancestor_record_offset":15096,"inlinee":83336,"source_line":70,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui_wall_transform_gizmos::RigidXZTransform::rotation_angle
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui_wall_transform_gizmos.rs
//     evidence={"module":1715,"record_offset":15332,"parent_record_offset":15248,"ancestor_record_offset":15096,"inlinee":35208,"source_line":101,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui_wall_transform_gizmos::RigidXZTransform::transform_point
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui_wall_transform_gizmos.rs
//     evidence={"module":1715,"record_offset":15540,"parent_record_offset":15248,"ancestor_record_offset":15096,"inlinee":35205,"source_line":137,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::affine2::Affine2::transform_point2
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/affine2.rs
//     evidence={"module":1715,"record_offset":15568,"parent_record_offset":15540,"ancestor_record_offset":15096,"inlinee":27806,"source_line":270,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::sin_cos
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/math.rs
//     evidence={"module":1715,"record_offset":15416,"parent_record_offset":15388,"ancestor_record_offset":15096,"inlinee":11738,"source_line":182,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::from_angle
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat2.rs
//     evidence={"module":1715,"record_offset":15388,"parent_record_offset":15248,"ancestor_record_offset":15096,"inlinee":35242,"source_line":131,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::mul_vec2
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat2.rs
//     evidence={"module":1715,"record_offset":15620,"parent_record_offset":15596,"ancestor_record_offset":15096,"inlinee":27810,"source_line":326,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat2.rs
//     evidence={"module":1715,"record_offset":15504,"parent_record_offset":15388,"ancestor_record_offset":15096,"inlinee":31978,"source_line":56,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::impl$22::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat2.rs
//     evidence={"module":1715,"record_offset":15596,"parent_record_offset":15568,"ancestor_record_offset":15096,"inlinee":27808,"source_line":581,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1715,"record_offset":15916,"parent_record_offset":15248,"ancestor_record_offset":15096,"inlinee":8712,"source_line":1309,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1715,"record_offset":15712,"parent_record_offset":15568,"ancestor_record_offset":15096,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1715,"record_offset":15860,"parent_record_offset":15248,"ancestor_record_offset":15096,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::atan2
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1715,"record_offset":15356,"parent_record_offset":15332,"ancestor_record_offset":15096,"inlinee":11245,"source_line":880,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sin
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1715,"record_offset":15464,"parent_record_offset":15440,"ancestor_record_offset":15096,"inlinee":11242,"source_line":699,"archive":"_pdb/C/modules/01715.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sin_cos
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1715,"record_offset":15440,"parent_record_offset":15416,"ancestor_record_offset":15096,"inlinee":11739,"source_line":910,"archive":"_pdb/C/modules/01715.jsonl.gz"}
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
// 完整结构及原始类型记录: replay_wall.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::history::replay_wall::WallReplayCmd
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 3; 过程引用数: 6
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<country_core::systems::history::replay_wall::WallReplayCmd>", "constructor": "core::ptr::drop_in_place", "arguments": ["country_core::systems::history::replay_wall::WallReplayCmd"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::history::replay_wall::WallReplayEditCmd
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 6; 过程引用数: 16
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<country_core::systems::history::replay_wall::WallReplayEditCmd,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["country_core::systems::history::replay_wall::WallReplayEditCmd", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<country_core::systems::history::replay_wall::WallReplayEditCmd,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<country_core::systems::history::replay_wall::WallReplayEditCmd,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::collections::vec_deque::impl$1::drop::Dropper<alloc::vec::Vec<country_core::systems::history::replay_wall::WallReplayEditCmd,alloc::alloc::Global> >", "constructor": "alloc::collections::vec_deque::impl$1::drop::Dropper", "arguments": ["alloc::vec::Vec<country_core::systems::history::replay_wall::WallReplayEditCmd,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::collections::vec_deque::impl$1::drop::Dropper<alloc::vec::Vec<country_core::systems::history::replay_wall::WallReplayEditCmd,alloc::alloc::Global> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::collections::vec_deque::impl$1::drop::Dropper<alloc::vec::Vec<country_core::systems::history::replay_wall::WallReplayEditCmd,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<alloc::vec::Vec<country_core::systems::history::replay_wall::WallReplayEditCmd,alloc::alloc::Global>,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["alloc::vec::Vec<country_core::systems::history::replay_wall::WallReplayEditCmd,alloc::alloc::Global>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<alloc::vec::Vec<country_core::systems::history::replay_wall::WallReplayEditCmd,alloc::alloc::Global>,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<alloc::vec::Vec<country_core::systems::history::replay_wall::WallReplayEditCmd,alloc::alloc::Global>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<alloc::vec::Vec<country_core::systems::history::replay_wall::WallReplayEditCmd,alloc::alloc::Global>,core::iter::adapters::map::Map<alloc::boxed::Box<dyn$<core::iter::traits::iterator::Iterator<assoc$<Item,ref$<country_core::resources::history::wall_compound_edit::ConcurrentEdits> > > >,alloc::alloc::Global>,country_core::systems::history::replay_wall::impl$0::new::closure_env$0> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["alloc::vec::Vec<country_core::systems::history::replay_wall::WallReplayEditCmd,alloc::alloc::Global>", "core::iter::adapters::map::Map<alloc::boxed::Box<dyn$<core::iter::traits::iterator::Iterator<assoc$<Item,ref$<country_core::resources::history::wall_compound_edit::ConcurrentEdits> > > >,alloc::alloc::Global>,country_core::systems::history::replay_wall::impl$0::new::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::impl$21::extend_trusted::closure_env$0<country_core::systems::history::replay_wall::WallReplayEditCmd,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<alloc::sync::Arc<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global> >,country_core::systems::history::replay_wall::impl$0::new::closure$0::closure_env$0> >", "constructor": "alloc::vec::impl$21::extend_trusted::closure_env$0", "arguments": ["country_core::systems::history::replay_wall::WallReplayEditCmd", "alloc::alloc::Global", "core::iter::adapters::map::Map<core::slice::iter::Iter<alloc::sync::Arc<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global> >,country_core::systems::history::replay_wall::impl$0::new::closure$0::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<country_core::systems::history::replay_wall::WallReplayEditCmd,alloc::vec::impl$21::extend_trusted::closure_env$0<country_core::systems::history::replay_wall::WallReplayEditCmd,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<alloc::sync::Arc<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global> >,country_core::systems::history::replay_wall::impl$0::new::closure$0::closure_env$0> > >", "constructor": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0", "arguments": ["country_core::systems::history::replay_wall::WallReplayEditCmd", "alloc::vec::impl$21::extend_trusted::closure_env$0<country_core::systems::history::replay_wall::WallReplayEditCmd,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<alloc::sync::Arc<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global> >,country_core::systems::history::replay_wall::impl$0::new::closure$0::closure_env$0> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::impl$2::fold<country_core::systems::history::replay_wall::WallReplayEditCmd,core::slice::iter::Iter<alloc::sync::Arc<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global> >,country_core::systems::history::replay_wall::impl$0::new::closure$0::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<country_core::systems::history::replay_wall::WallReplayEditCmd,alloc::vec::impl$21::extend_trusted::closure_env$0<country_core::systems::history::replay_wall::WallReplayEditCmd,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<alloc::sync::Arc<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global> >,country_core::systems::history::replay_wall::impl$0::new::closure$0::closure_env$0> > > >", "constructor": "core::iter::adapters::map::impl$2::fold", "arguments": ["country_core::systems::history::replay_wall::WallReplayEditCmd", "core::slice::iter::Iter<alloc::sync::Arc<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global> >", "country_core::systems::history::replay_wall::impl$0::new::closure$0::closure_env$0", "tuple$<>", "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<country_core::systems::history::replay_wall::WallReplayEditCmd,alloc::vec::impl$21::extend_trusted::closure_env$0<country_core::systems::history::replay_wall::WallReplayEditCmd,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<alloc::sync::Arc<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global> >,country_core::systems::history::replay_wall::impl$0::new::closure$0::closure_env$0> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::systems::history::replay_wall::WallReplayEditCmd::process<country_core::systems::history::replay_wall::replay_wall::closure_env$0,country_core::systems::history::replay_wall::replay_wall::closure_env$1>", "constructor": "country_core::systems::history::replay_wall::WallReplayEditCmd::process", "arguments": ["country_core::systems::history::replay_wall::replay_wall::closure_env$0", "country_core::systems::history::replay_wall::replay_wall::closure_env$1"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: replay_wall.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// [已有模块证据关联；不自动成为字段事实] evidence/building-history-owners/three-version-owner-map.json; mixed_or_unspecified; applicability requires per-version original evidence; [{"kind": "explicit_source_path_mention", "path": "crates/country-core/src/systems/history/replay_wall.rs", "status": "source mention; exact assertion applicability remains version-scoped"}, {"kind": "explicit_source_path_mention", "path": "crates/country-core/src/systems/history/replay_wall.rs", "status": "source mention; exact assertion applicability remains version-scoped"}, {"kind": "explicit_source_path_mention", "path": "crates/country-core/src/systems/history/replay_wall.rs", "status": "source mention; exact assertion applicability remains version-scoped"}]
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: replay_wall.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 38; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<std::fs::DirEntry> > (*)(enum2$<core::result::Result<std::fs::DirEntry,std::io::error::Error> >)
// 候选参数类型: ["enum2$<core::result::Result<std::fs::DirEntry,std::io::error::Error> >"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<alloc::vec::Vec<country_core::resources::clutter_packs::ClutterPackObject,alloc::alloc::Global> > > (*)(ref$<tg_mods::overlay::Mod>)
// 候选参数类型: ["ref$<tg_mods::overlay::Mod>"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::PushConstants> > (*)(ref_mut$<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::sparse_instance_buffer::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0> >)
// 候选参数类型: ["ref_mut$<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::sparse_instance_buffer::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0> >"]
// [文件级接口候选，具体函数归属 unknown] country_core::systems::wall::structure_outline::WallOutline (*)(country_core::resources::walls::WallId)
// 候选参数类型: ["country_core::resources::walls::WallId"]
// [文件级接口候选，具体函数归属 unknown] tuple$<bool,tuple$<> > (*)(ref_mut$<bevy_ecs::storage::sparse_set::SparseSets>,enum2$<core::option::Option<ref_mut$<bevy_ecs::storage::table::Table> > >,ref$<bevy_ecs::component::info::Components>,ref$<slice2$<bevy_ecs::component::info::ComponentId> >)
// 候选参数类型: ["ref_mut$<bevy_ecs::storage::sparse_set::SparseSets>", "enum2$<core::option::Option<ref_mut$<bevy_ecs::storage::table::Table> > >", "ref$<bevy_ecs::component::info::Components>", "ref$<slice2$<bevy_ecs::component::info::ComponentId> >"]
// [文件级接口候选，具体函数归属 unknown] void (*)(core::pin::Pin<alloc::boxed::Box<dyn$<core::future::future::Future<assoc$<Output,tuple$<> > >,core::marker::Send>,alloc::alloc::Global> >)
// 候选参数类型: ["core::pin::Pin<alloc::boxed::Box<dyn$<core::future::future::Future<assoc$<Output,tuple$<> > >,core::marker::Send>,alloc::alloc::Global> >"]
// [文件级接口候选，具体函数归属 unknown] relative_path::relative_path_buf::RelativePathBuf (*)(ref$<ref$<str$> >)
// 候选参数类型: ["ref$<ref$<str$> >"]
// [文件级接口候选，具体函数归属 unknown] enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr> (*)(country_core::resources::walls::WallId)
// 候选参数类型: ["country_core::resources::walls::WallId"]
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::systems::history::replay_wall::WallReplayCmd::is_single_adjust_height_cmd
// 信息 ID: feb7024c67305d766acc12e0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::history::history_process::replay_edit; module=594, inline_record=32632; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::history::replay_wall::WallReplayCmd::new
// 信息 ID: 7bd82e24dbdc332465925de4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::history::replay_wall::WallReplayEditCmd::process<country_core::systems::history::replay_wall::replay_wall::closure_env$0,country_core::systems::history::replay_wall::replay_wall::closure_env$1>
// 信息 ID: e76fe9492dc5e71a031dcebe
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::systems::history::replay_wall::WallReplayEditCmd::process -> ["country_core::systems::history::replay_wall::replay_wall::closure_env$0", "country_core::systems::history::replay_wall::replay_wall::closure_env$1"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::history::replay_wall::replay_wall @ 0x19c0fc6 -> 0x19e0090 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::history::replay_wall::WallReplayEditCmd::process_animated
// 信息 ID: 46df05d6ad3f5668df1beaa9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::history::replay_wall::WallReplayEditCmd::process<country_core::systems::history::replay_wall::replay_wall::closure_env$0,country_core::systems::history::replay_wall::replay_wall::closure_env$1>; module=1720, inline_record=128992; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::history::replay_wall::WallReplayEditCmd::process_instant
// 信息 ID: 91bbd756f1fdfb8dc9c5177d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::history::replay_wall::WallReplayEditCmd::process<country_core::systems::history::replay_wall::replay_wall::closure_env$0,country_core::systems::history::replay_wall::replay_wall::closure_env$1>; module=1720, inline_record=132192; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::history::replay_wall::_deprecated_morph_curve
// 信息 ID: 78e69b12b6dd4b91b1601aab
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::history::replay_wall::WallReplayEditCmd::process<country_core::systems::history::replay_wall::replay_wall::closure_env$0,country_core::systems::history::replay_wall::replay_wall::closure_env$1> @ 0x19e02b1 -> 0x19c1e90 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::history::replay_wall::impl$0::new::closure$0
// 信息 ID: 88d46490deda244591f31d2e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::history::replay_wall::impl$0::new", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::history::replay_wall::impl$0::new::closure_env$0 <- core::ptr::drop_in_place<core::iter::adapters::map::Map<alloc::boxed::Box<dyn$<core::iter::traits::iterator::Iterator<assoc$<Item,ref$<country_core::resources::history::wall_compound_edit::ConcurrentEdits> > > >,alloc::alloc::Global>,country_core::systems::history::replay_wall::impl$0::new::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::history::replay_wall::impl$0::new::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<alloc::vec::Vec<country_core::systems::history::replay_wall::WallReplayEditCmd,alloc::alloc::Global>,core::iter::adapters::map::Map<alloc::boxed::Box<dyn$<core::iter::traits::iterator::Iterator<assoc$<Item,ref$<country_core::resources::history::wall_compound_edit::ConcurrentEdits> > > >,alloc::alloc::Global>,country_core::systems::history::replay_wall::impl$0::new::closure_env$0> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<alloc::vec::Vec<country_core::systems::history::replay_wall::WallReplayEditCmd,alloc::alloc::Global>,core::iter::adapters::map::Map<alloc::boxed::Box<dyn$<core::iter::traits::iterator::Iterator<assoc$<Item,ref$<country_core::resources::history::wall_compound_edit::ConcurrentEdits> > > >,alloc::alloc::Global>,country_core::systems::history::replay_wall::impl$0::new::closure_env$0> >; module=1643, inline_record=540000; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<alloc::vec::Vec<country_core::systems::history::replay_wall::WallReplayEditCmd,alloc::alloc::Global>,core::iter::adapters::map::Map<alloc::boxed::Box<dyn$<core::iter::traits::iterator::Iterator<assoc$<Item,ref$<country_core::resources::history::wall_compound_edit::ConcurrentEdits> > > >,alloc::alloc::Global>,country_core::systems::history::replay_wall::impl$0::new::closure_env$0> >; module=1643, inline_record=542220; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::history::replay_wall::impl$0::new::closure$0::closure$0
// 信息 ID: aace88583a9bd7ce4005381c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::history::replay_wall::impl$0::new", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::history::replay_wall::impl$0::new::closure$0::closure_env$0 <- core::iter::adapters::map::impl$2::fold<country_core::systems::history::replay_wall::WallReplayEditCmd,core::slice::iter::Iter<alloc::sync::Arc<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global> >,country_core::systems::history::replay_wall::impl$0::new::closure$0::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<country_core::systems::history::replay_wall::WallReplayEditCmd,alloc::vec::impl$21::extend_trusted::closure_env$0<country_core::systems::history::replay_wall::WallReplayEditCmd,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<alloc::sync::Arc<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global> >,country_core::systems::history::replay_wall::impl$0::new::closure$0::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::history::replay_wall::impl$0::new::closure$0::closure_env$0 <- core::iter::adapters::map::impl$2::fold<country_core::systems::history::replay_wall::WallReplayEditCmd,core::slice::iter::Iter<alloc::sync::Arc<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global> >,country_core::systems::history::replay_wall::impl$0::new::closure$0::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<country_core::systems::history::replay_wall::WallReplayEditCmd,alloc::vec::impl$21::extend_trusted::closure_env$0<country_core::systems::history::replay_wall::WallReplayEditCmd,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<alloc::sync::Arc<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global> >,country_core::systems::history::replay_wall::impl$0::new::closure$0::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<country_core::systems::history::replay_wall::WallReplayEditCmd,core::slice::iter::Iter<alloc::sync::Arc<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global> >,country_core::systems::history::replay_wall::impl$0::new::closure$0::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<country_core::systems::history::replay_wall::WallReplayEditCmd,alloc::vec::impl$21::extend_trusted::closure_env$0<country_core::systems::history::replay_wall::WallReplayEditCmd,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<alloc::sync::Arc<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global> >,country_core::systems::history::replay_wall::impl$0::new::closure$0::closure_env$0> > > >; module=1682, inline_record=127936; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::history::replay_wall::impl$1::process_instant::closure$0
// 信息 ID: f9af8515e9a28d4984fae8e2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::history::replay_wall::impl$1::process_instant", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::history::replay_wall::WallReplayEditCmd::process<country_core::systems::history::replay_wall::replay_wall::closure_env$0,country_core::systems::history::replay_wall::replay_wall::closure_env$1>; module=1720, inline_record=134940; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::history::replay_wall::impl$1::process_instant::closure$1
// 信息 ID: a8b960f2e2ddf3bd23824d9b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::systems::history::replay_wall::impl$1::process_instant", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::history::replay_wall::WallReplayEditCmd::process<country_core::systems::history::replay_wall::replay_wall::closure_env$0,country_core::systems::history::replay_wall::replay_wall::closure_env$1>; module=1720, inline_record=134784; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::history::replay_wall::replay_wall
// 信息 ID: 213946fe08b3ef7044f50460
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 8f521fbad85564a1a624b3ea: void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::adjust_wall_height::adjust_wall_height_impl::AdjustWallHeightCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::stone_foundation_offset::SetStoneFoundationOffsetCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_audio_fmod::PlayWallAudio>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetWoodenWallColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::adjust_wall_height::adjust_wall_height_impl::AdjustWallHeightCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::stone_foundation_offset::SetStoneFoundationOffsetCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_audio_fmod::PlayWallAudio>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetWoodenWallColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >),tuple$<>,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::adjust_wall_height::adjust_wall_height_impl::AdjustWallHeightCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::stone_foundation_offset::SetStoneFoundationOffsetCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_audio_fmod::PlayWallAudio>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetWoodenWallColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >)> > @ 0x1958a32 -> 0x19c0d80 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::adjust_wall_height::adjust_wall_height_impl::AdjustWallHeightCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::stone_foundation_offset::SetStoneFoundationOffsetCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_audio_fmod::PlayWallAudio>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetWoodenWallColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >),tuple$<>,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::adjust_wall_height::adjust_wall_height_impl::AdjustWallHeightCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::stone_foundation_offset::SetStoneFoundationOffsetCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_audio_fmod::PlayWallAudio>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetWoodenWallColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >)> @ 0x19df0b1 -> 0x19c0d80 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::history::replay_wall::replay_wall::closure$0
// 信息 ID: 62b8cd84955da35b293cac56
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::history::replay_wall::replay_wall", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::history::replay_wall::replay_wall::closure_env$0 <- country_core::systems::history::replay_wall::WallReplayEditCmd::process<country_core::systems::history::replay_wall::replay_wall::closure_env$0,country_core::systems::history::replay_wall::replay_wall::closure_env$1>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 2 条；闭包环境类型引用 1 条。
// [静态直接引用] country_core::systems::history::replay_wall::WallReplayEditCmd::process<country_core::systems::history::replay_wall::replay_wall::closure_env$0,country_core::systems::history::replay_wall::replay_wall::closure_env$1> @ 0x19e1190 -> 0x19e0000 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::systems::history::replay_wall::WallReplayEditCmd::process<country_core::systems::history::replay_wall::replay_wall::closure_env$0,country_core::systems::history::replay_wall::replay_wall::closure_env$1>; module=1720, inline_record=130496; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::replay_wall::WallReplayEditCmd::process<country_core::systems::history::replay_wall::replay_wall::closure_env$0,country_core::systems::history::replay_wall::replay_wall::closure_env$1>; module=1720, inline_record=134708; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::history::replay_wall::replay_wall::closure$1
// 信息 ID: e163c75d2d1c65a6b84f8ce3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::systems::history::replay_wall::replay_wall", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::history::replay_wall::replay_wall::closure_env$1 <- country_core::systems::history::replay_wall::WallReplayEditCmd::process<country_core::systems::history::replay_wall::replay_wall::closure_env$0,country_core::systems::history::replay_wall::replay_wall::closure_env$1>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] country_core::systems::history::replay_wall::WallReplayEditCmd::process<country_core::systems::history::replay_wall::replay_wall::closure_env$0,country_core::systems::history::replay_wall::replay_wall::closure_env$1>; module=1720, inline_record=133664; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::history::replay_wall::rigid_transform_curve
// 信息 ID: d419a518bb73bcdda5fbd8fd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::history::replay_wall::WallReplayEditCmd::process<country_core::systems::history::replay_wall::replay_wall::closure_env$0,country_core::systems::history::replay_wall::replay_wall::closure_env$1>; module=1720, inline_record=130408; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===

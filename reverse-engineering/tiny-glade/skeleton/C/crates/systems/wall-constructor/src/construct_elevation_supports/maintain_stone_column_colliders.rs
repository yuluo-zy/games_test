// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/systems/wall-constructor/src/construct_elevation_supports/maintain_stone_column_colliders.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 20840b88b3d270b97fd29947d527f9cfc610c55882397a326ed10cbc574c6fbf
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: maintain_stone_column_colliders.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 422, 1455, 1482
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
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.1"}]
// use std; // namespace candidate
//   package identities: []
// use system_wall_constructor; // namespace candidate
//   package identities: [{"name":"system-wall-constructor","source":"local_or_path","version":"0.1.0"}]
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders
// 记录形式: compiled_procedure; instances=1
// 源码行: 17
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":422,"record_offset":280,"rva":8624368,"va":5377333488}
// 编译实例 RVAs: 0x8398f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-1656389","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377333488,"raw_hex":"e8bbb9e6ff","size":5,"soft_fail":false,"target_va":5375678688,"va":5377335072,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t-1655623","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377333488,"raw_hex":"e8b9bce6ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5377334322,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=2
//     target_aliases=1699 candidates; complete list: maintain_stone_column_colliders.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8612872","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377333488,"raw_hex":"e8f8937cff","size":5,"soft_fail":false,"target_va":5368722192,"va":5377335059,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048c880
//     owner=unresolved_namespace; count=1
//     target_aliases=6 candidates; complete list: maintain_stone_column_colliders.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3855600","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377333488,"raw_hex":"e8102bc5ff","size":5,"soft_fail":false,"target_va":5373479040,"va":5377334635,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c915a0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: maintain_stone_column_colliders.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4552974","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377333488,"raw_hex":"e80e794500","size":5,"soft_fail":false,"target_va":5381887392,"va":5377334413,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1413f5fe0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: maintain_stone_column_colliders.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12304756","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377333488,"raw_hex":"e874c1bb00","size":5,"soft_fail":false,"target_va":5389639648,"va":5377334887,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1413fad90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: maintain_stone_column_colliders.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12325016","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377333488,"raw_hex":"e89810bc00","size":5,"soft_fail":false,"target_va":5389659536,"va":5377334515,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14140aae0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: maintain_stone_column_colliders.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12387798","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377333488,"raw_hex":"e8d605bd00","size":5,"soft_fail":false,"target_va":5389724384,"va":5377336581,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141415710
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: maintain_stone_column_colliders.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12434171","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377333488,"raw_hex":"e8fbbabd00","size":5,"soft_fail":false,"target_va":5389768464,"va":5377334288,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141417580
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: maintain_stone_column_colliders.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12441881","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377333488,"raw_hex":"e819d9bd00","size":5,"soft_fail":false,"target_va":5389776256,"va":5377334370,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141418960
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: maintain_stone_column_colliders.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12445722","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377333488,"raw_hex":"e81ae8bd00","size":5,"soft_fail":false,"target_va":5389781344,"va":5377335617,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f400
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: maintain_stone_column_colliders.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t39014996","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377333488,"raw_hex":"e854525302","size":5,"soft_fail":false,"target_va":5416350720,"va":5377335719,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: maintain_stone_column_colliders.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t39721541","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377333488,"raw_hex":"e8451a5e02","size":5,"soft_fail":false,"target_va":5417057056,"va":5377335510,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t39284890","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377333488,"raw_hex":"e89a705702","size":5,"soft_fail":false,"target_va":5416620769,"va":5377335874,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t39284821","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377333488,"raw_hex":"e855705702","size":5,"soft_fail":false,"target_va":5416620794,"va":5377335968,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t39285514","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377333488,"raw_hex":"e80a735702","size":5,"soft_fail":false,"target_va":5416621408,"va":5377335889,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t39285739","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377333488,"raw_hex":"e8eb735702","size":5,"soft_fail":false,"target_va":5416621600,"va":5377335856,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t3176","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377333488,"raw_hex":"e8680c0000","size":5,"soft_fail":false,"target_va":5377336768,"va":5377333587,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t39285365","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377333488,"raw_hex":"e875725702","size":5,"soft_fail":false,"target_va":5416621376,"va":5377336006,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::collision::world::RaycastMesh::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1820994","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377333488,"raw_hex":"e842c91b00","size":5,"soft_fail":false,"target_va":5379156544,"va":5377335545,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::collision::world::RaycastMeshData::new_indexed
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1832537","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377333488,"raw_hex":"e859f61b00","size":5,"soft_fail":false,"target_va":5379167904,"va":5377335362,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t6629863","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377333488,"raw_hex":"e8e7296500","size":5,"soft_fail":false,"target_va":5383963792,"va":5377333924,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::__RUST_STD_INTERNAL_VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t6638370","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377333488,"raw_hex":"e8224b6500","size":5,"soft_fail":false,"target_va":5383972224,"va":5377333849,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t6629108","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377333488,"raw_hex":"e8f4266500","size":5,"soft_fail":false,"target_va":5383964912,"va":5377335799,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t6629254","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377333488,"raw_hex":"e886276500","size":5,"soft_fail":false,"target_va":5383962816,"va":5377333557,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::native::lazy::Storage$LT$T$C$D$GT$::get_or_init_slow
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t39361104","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377333488,"raw_hex":"e8509a5802","size":5,"soft_fail":false,"target_va":5416697008,"va":5377335899,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t39283435","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377333488,"raw_hex":"e8eb6a5702","size":5,"soft_fail":false,"target_va":5416619360,"va":5377335920,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":422,"record_offset":7916,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":6896,"source_line":419,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::drain
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":422,"record_offset":4616,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":6382,"source_line":2922,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":422,"record_offset":6176,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":5735,"source_line":3021,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":422,"record_offset":3800,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":422,"record_offset":4992,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":422,"record_offset":5108,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":5075,"source_line":213,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":422,"record_offset":3744,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::iterators::impl$0::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":422,"record_offset":3460,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":11533,"source_line":14,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::message_reader::MessageReader::read
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_reader.rs
//     evidence={"module":422,"record_offset":2764,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":9339,"source_line":27,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::message_writer::MessageWriter::write_default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_writer.rs
//     evidence={"module":422,"record_offset":8528,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":11211,"source_line":90,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":422,"record_offset":4908,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":422,"record_offset":9800,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":422,"record_offset":3856,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":422,"record_offset":6204,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":422,"record_offset":1956,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":8276,"source_line":589,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":422,"record_offset":556,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":9267,"source_line":626,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":422,"record_offset":944,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":8242,"source_line":659,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":422,"record_offset":5236,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":7239,"source_line":997,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::insert
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":422,"record_offset":3828,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":5499,"source_line":1295,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":422,"record_offset":6828,"parent_record_offset":6804,"ancestor_record_offset":280,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":422,"record_offset":6852,"parent_record_offset":6828,"ancestor_record_offset":280,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":422,"record_offset":4068,"parent_record_offset":4044,"ancestor_record_offset":280,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":422,"record_offset":4092,"parent_record_offset":4068,"ancestor_record_offset":280,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":422,"record_offset":6876,"parent_record_offset":6852,"ancestor_record_offset":280,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":422,"record_offset":4116,"parent_record_offset":4092,"ancestor_record_offset":280,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":422,"record_offset":6804,"parent_record_offset":6652,"ancestor_record_offset":280,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":422,"record_offset":4044,"parent_record_offset":3952,"ancestor_record_offset":280,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":422,"record_offset":7996,"parent_record_offset":7952,"ancestor_record_offset":280,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":422,"record_offset":7952,"parent_record_offset":7916,"ancestor_record_offset":280,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":422,"record_offset":4676,"parent_record_offset":4652,"ancestor_record_offset":280,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":422,"record_offset":6584,"parent_record_offset":6552,"ancestor_record_offset":280,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":422,"record_offset":3980,"parent_record_offset":3952,"ancestor_record_offset":280,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":422,"record_offset":3952,"parent_record_offset":3928,"ancestor_record_offset":280,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":422,"record_offset":4724,"parent_record_offset":4700,"ancestor_record_offset":280,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":422,"record_offset":4700,"parent_record_offset":4676,"ancestor_record_offset":280,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":422,"record_offset":6652,"parent_record_offset":6616,"ancestor_record_offset":280,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":422,"record_offset":6616,"parent_record_offset":6584,"ancestor_record_offset":280,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":422,"record_offset":3928,"parent_record_offset":3904,"ancestor_record_offset":280,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":422,"record_offset":6692,"parent_record_offset":6652,"ancestor_record_offset":280,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":422,"record_offset":4420,"parent_record_offset":4384,"ancestor_record_offset":280,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":422,"record_offset":4384,"parent_record_offset":4360,"ancestor_record_offset":280,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":422,"record_offset":8768,"parent_record_offset":8728,"ancestor_record_offset":280,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":422,"record_offset":4652,"parent_record_offset":4616,"ancestor_record_offset":280,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":422,"record_offset":3104,"parent_record_offset":3080,"ancestor_record_offset":280,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":422,"record_offset":6960,"parent_record_offset":6936,"ancestor_record_offset":280,"inlinee":5079,"source_line":3988,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":422,"record_offset":4764,"parent_record_offset":4616,"ancestor_record_offset":280,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":422,"record_offset":8696,"parent_record_offset":8660,"ancestor_record_offset":280,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":422,"record_offset":8728,"parent_record_offset":8696,"ancestor_record_offset":280,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":422,"record_offset":4792,"parent_record_offset":4616,"ancestor_record_offset":280,"inlinee":4802,"source_line":2161,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":422,"record_offset":6520,"parent_record_offset":6480,"ancestor_record_offset":280,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":422,"record_offset":6552,"parent_record_offset":6520,"ancestor_record_offset":280,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":422,"record_offset":4932,"parent_record_offset":4908,"ancestor_record_offset":280,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$21::extend_trusted::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":422,"record_offset":7232,"parent_record_offset":7200,"ancestor_record_offset":280,"inlinee":5113,"source_line":4001,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":422,"record_offset":3080,"parent_record_offset":2852,"ancestor_record_offset":280,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":422,"record_offset":8180,"parent_record_offset":8156,"ancestor_record_offset":280,"inlinee":4693,"source_line":497,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":422,"record_offset":8228,"parent_record_offset":8204,"ancestor_record_offset":280,"inlinee":4706,"source_line":501,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":422,"record_offset":7344,"parent_record_offset":7232,"ancestor_record_offset":280,"inlinee":5115,"source_line":18,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":422,"record_offset":6936,"parent_record_offset":6480,"ancestor_record_offset":280,"inlinee":5078,"source_line":26,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":422,"record_offset":6444,"parent_record_offset":6408,"ancestor_record_offset":280,"inlinee":6133,"source_line":32,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":422,"record_offset":6480,"parent_record_offset":6444,"ancestor_record_offset":280,"inlinee":6135,"source_line":50,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":422,"record_offset":3768,"parent_record_offset":3744,"ancestor_record_offset":280,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":422,"record_offset":8552,"parent_record_offset":8528,"ancestor_record_offset":280,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::iterators::MessageIteratorWithId::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":422,"record_offset":2852,"parent_record_offset":2828,"ancestor_record_offset":280,"inlinee":9344,"source_line":54,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::iterators::impl$3::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":422,"record_offset":3484,"parent_record_offset":3460,"ancestor_record_offset":280,"inlinee":11534,"source_line":86,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::message_cursor::MessageCursor::read_with_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_cursor.rs
//     evidence={"module":422,"record_offset":2828,"parent_record_offset":2788,"ancestor_record_offset":280,"inlinee":9342,"source_line":89,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::messages::Messages::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs
//     evidence={"module":422,"record_offset":8636,"parent_record_offset":8612,"ancestor_record_offset":280,"inlinee":8680,"source_line":125,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::messages::Messages::write_default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs
//     evidence={"module":422,"record_offset":8612,"parent_record_offset":8528,"ancestor_record_offset":280,"inlinee":11212,"source_line":168,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::messages::Messages::write_with_caller
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs
//     evidence={"module":422,"record_offset":8660,"parent_record_offset":8636,"ancestor_record_offset":280,"inlinee":8681,"source_line":129,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$14::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/system_param.rs
//     evidence={"module":422,"record_offset":2788,"parent_record_offset":2764,"ancestor_record_offset":280,"inlinee":9614,"source_line":1064,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":422,"record_offset":6716,"parent_record_offset":6692,"ancestor_record_offset":280,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":422,"record_offset":2284,"parent_record_offset":2256,"ancestor_record_offset":280,"inlinee":6507,"source_line":2061,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":422,"record_offset":2312,"parent_record_offset":2284,"ancestor_record_offset":280,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":422,"record_offset":2232,"parent_record_offset":2204,"ancestor_record_offset":280,"inlinee":6546,"source_line":1221,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":422,"record_offset":2256,"parent_record_offset":2232,"ancestor_record_offset":280,"inlinee":6505,"source_line":1255,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":422,"record_offset":2372,"parent_record_offset":2256,"ancestor_record_offset":280,"inlinee":5177,"source_line":2443,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$52::drop
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":422,"record_offset":2484,"parent_record_offset":2460,"ancestor_record_offset":280,"inlinee":4995,"source_line":2052,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":422,"record_offset":5936,"parent_record_offset":5912,"ancestor_record_offset":280,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":422,"record_offset":5504,"parent_record_offset":5480,"ancestor_record_offset":280,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":422,"record_offset":5588,"parent_record_offset":5564,"ancestor_record_offset":280,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":422,"record_offset":6004,"parent_record_offset":5368,"ancestor_record_offset":280,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::and_then_or_clear
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":422,"record_offset":3536,"parent_record_offset":3512,"ancestor_record_offset":280,"inlinee":7044,"source_line":331,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":422,"record_offset":3512,"parent_record_offset":3484,"ancestor_record_offset":280,"inlinee":7042,"source_line":81,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":422,"record_offset":3616,"parent_record_offset":3592,"ancestor_record_offset":280,"inlinee":7430,"source_line":82,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":422,"record_offset":7008,"parent_record_offset":6984,"ancestor_record_offset":280,"inlinee":6631,"source_line":124,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":422,"record_offset":7132,"parent_record_offset":7032,"ancestor_record_offset":280,"inlinee":5396,"source_line":88,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":422,"record_offset":7096,"parent_record_offset":7068,"ancestor_record_offset":280,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":422,"record_offset":7068,"parent_record_offset":7032,"ancestor_record_offset":280,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":422,"record_offset":4956,"parent_record_offset":4932,"ancestor_record_offset":280,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":422,"record_offset":7032,"parent_record_offset":7008,"ancestor_record_offset":280,"inlinee":6141,"source_line":2640,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":422,"record_offset":6984,"parent_record_offset":6960,"ancestor_record_offset":280,"inlinee":5042,"source_line":835,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":422,"record_offset":7200,"parent_record_offset":7132,"ancestor_record_offset":280,"inlinee":5057,"source_line":842,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\manually_drop.rs
//     evidence={"module":422,"record_offset":5020,"parent_record_offset":4992,"ancestor_record_offset":280,"inlinee":5072,"source_line":184,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_dangling::MaybeDangling::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_dangling.rs
//     evidence={"module":422,"record_offset":5044,"parent_record_offset":5020,"ancestor_record_offset":280,"inlinee":5074,"source_line":80,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":422,"record_offset":2336,"parent_record_offset":2312,"ancestor_record_offset":280,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":422,"record_offset":6740,"parent_record_offset":6716,"ancestor_record_offset":280,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":422,"record_offset":3036,"parent_record_offset":3008,"ancestor_record_offset":280,"inlinee":7937,"source_line":917,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":422,"record_offset":6764,"parent_record_offset":6740,"ancestor_record_offset":280,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":422,"record_offset":2896,"parent_record_offset":2852,"ancestor_record_offset":280,"inlinee":9345,"source_line":2344,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":422,"record_offset":4012,"parent_record_offset":3980,"ancestor_record_offset":280,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":422,"record_offset":5736,"parent_record_offset":5708,"ancestor_record_offset":280,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":422,"record_offset":5708,"parent_record_offset":5680,"ancestor_record_offset":280,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":422,"record_offset":2104,"parent_record_offset":2064,"ancestor_record_offset":280,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":422,"record_offset":3560,"parent_record_offset":3536,"ancestor_record_offset":280,"inlinee":4102,"source_line":766,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":422,"record_offset":3592,"parent_record_offset":3512,"ancestor_record_offset":280,"inlinee":7428,"source_line":1647,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":422,"record_offset":892,"parent_record_offset":864,"ancestor_record_offset":280,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":422,"record_offset":2720,"parent_record_offset":2064,"ancestor_record_offset":280,"inlinee":6443,"source_line":266,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":422,"record_offset":5528,"parent_record_offset":5504,"ancestor_record_offset":280,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=25
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":422,"record_offset":2436,"parent_record_offset":2204,"ancestor_record_offset":280,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":422,"record_offset":3352,"parent_record_offset":3328,"ancestor_record_offset":280,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":422,"record_offset":5852,"parent_record_offset":5828,"ancestor_record_offset":280,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":422,"record_offset":3692,"parent_record_offset":3668,"ancestor_record_offset":280,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":422,"record_offset":4452,"parent_record_offset":4420,"ancestor_record_offset":280,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":422,"record_offset":1364,"parent_record_offset":1340,"ancestor_record_offset":280,"inlinee":8268,"source_line":575,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":422,"record_offset":5176,"parent_record_offset":5108,"ancestor_record_offset":280,"inlinee":5076,"source_line":985,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":422,"record_offset":1340,"parent_record_offset":1316,"ancestor_record_offset":280,"inlinee":8267,"source_line":728,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":422,"record_offset":1548,"parent_record_offset":1520,"ancestor_record_offset":280,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":422,"record_offset":5200,"parent_record_offset":5176,"ancestor_record_offset":280,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":422,"record_offset":8880,"parent_record_offset":8728,"ancestor_record_offset":280,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$49::slice_contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":422,"record_offset":1648,"parent_record_offset":1620,"ancestor_record_offset":280,"inlinee":8260,"source_line":405,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":422,"record_offset":1620,"parent_record_offset":1592,"ancestor_record_offset":280,"inlinee":8258,"source_line":2589,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":422,"record_offset":2952,"parent_record_offset":2852,"ancestor_record_offset":280,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":422,"record_offset":3304,"parent_record_offset":2852,"ancestor_record_offset":280,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":422,"record_offset":3008,"parent_record_offset":2980,"ancestor_record_offset":280,"inlinee":8594,"source_line":366,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":422,"record_offset":2980,"parent_record_offset":2952,"ancestor_record_offset":280,"inlinee":8593,"source_line":545,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":422,"record_offset":3328,"parent_record_offset":3304,"ancestor_record_offset":280,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":422,"record_offset":1292,"parent_record_offset":1268,"ancestor_record_offset":280,"inlinee":8265,"source_line":81,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":422,"record_offset":1316,"parent_record_offset":1292,"ancestor_record_offset":280,"inlinee":8266,"source_line":119,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":422,"record_offset":1676,"parent_record_offset":1648,"ancestor_record_offset":280,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":422,"record_offset":3668,"parent_record_offset":3616,"ancestor_record_offset":280,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$172::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":422,"record_offset":1268,"parent_record_offset":1152,"ancestor_record_offset":280,"inlinee":8264,"source_line":438,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":422,"record_offset":684,"parent_record_offset":660,"ancestor_record_offset":280,"inlinee":9253,"source_line":377,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":422,"record_offset":980,"parent_record_offset":944,"ancestor_record_offset":280,"inlinee":8243,"source_line":1527,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":422,"record_offset":1120,"parent_record_offset":1088,"ancestor_record_offset":280,"inlinee":8253,"source_line":125,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":422,"record_offset":1088,"parent_record_offset":1052,"ancestor_record_offset":280,"inlinee":8251,"source_line":215,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":422,"record_offset":1028,"parent_record_offset":1004,"ancestor_record_offset":280,"inlinee":8247,"source_line":315,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":422,"record_offset":1052,"parent_record_offset":1028,"ancestor_record_offset":280,"inlinee":8249,"source_line":709,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":422,"record_offset":1004,"parent_record_offset":980,"ancestor_record_offset":280,"inlinee":8245,"source_line":798,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":422,"record_offset":1592,"parent_record_offset":1052,"ancestor_record_offset":280,"inlinee":8257,"source_line":659,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":422,"record_offset":660,"parent_record_offset":628,"ancestor_record_offset":280,"inlinee":9270,"source_line":438,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":422,"record_offset":628,"parent_record_offset":600,"ancestor_record_offset":280,"inlinee":9269,"source_line":468,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":422,"record_offset":784,"parent_record_offset":752,"ancestor_record_offset":280,"inlinee":9252,"source_line":507,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":422,"record_offset":840,"parent_record_offset":784,"ancestor_record_offset":280,"inlinee":9254,"source_line":527,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":422,"record_offset":752,"parent_record_offset":724,"ancestor_record_offset":280,"inlinee":8274,"source_line":539,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":422,"record_offset":600,"parent_record_offset":556,"ancestor_record_offset":280,"inlinee":8272,"source_line":62,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":422,"record_offset":864,"parent_record_offset":840,"ancestor_record_offset":280,"inlinee":8315,"source_line":189,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":422,"record_offset":1152,"parent_record_offset":1120,"ancestor_record_offset":280,"inlinee":8255,"source_line":78,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":422,"record_offset":1436,"parent_record_offset":1152,"ancestor_record_offset":280,"inlinee":8270,"source_line":16,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":422,"record_offset":1408,"parent_record_offset":1152,"ancestor_record_offset":280,"inlinee":8269,"source_line":10,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":422,"record_offset":1212,"parent_record_offset":1152,"ancestor_record_offset":280,"inlinee":8262,"source_line":23,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":422,"record_offset":4480,"parent_record_offset":4420,"ancestor_record_offset":280,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":422,"record_offset":4512,"parent_record_offset":4384,"ancestor_record_offset":280,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$10::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":422,"record_offset":5960,"parent_record_offset":5936,"ancestor_record_offset":280,"inlinee":12117,"source_line":82,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":422,"record_offset":5648,"parent_record_offset":5620,"ancestor_record_offset":280,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":422,"record_offset":5680,"parent_record_offset":5648,"ancestor_record_offset":280,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":422,"record_offset":5620,"parent_record_offset":5368,"ancestor_record_offset":280,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":422,"record_offset":5480,"parent_record_offset":5368,"ancestor_record_offset":280,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":422,"record_offset":6032,"parent_record_offset":5368,"ancestor_record_offset":280,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":422,"record_offset":5564,"parent_record_offset":5368,"ancestor_record_offset":280,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":422,"record_offset":5424,"parent_record_offset":5368,"ancestor_record_offset":280,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":422,"record_offset":5912,"parent_record_offset":5888,"ancestor_record_offset":280,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":422,"record_offset":5260,"parent_record_offset":5236,"ancestor_record_offset":280,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":422,"record_offset":5888,"parent_record_offset":5780,"ancestor_record_offset":280,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":422,"record_offset":5292,"parent_record_offset":5260,"ancestor_record_offset":280,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":422,"record_offset":5828,"parent_record_offset":5804,"ancestor_record_offset":280,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":422,"record_offset":6124,"parent_record_offset":5368,"ancestor_record_offset":280,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":422,"record_offset":5804,"parent_record_offset":5780,"ancestor_record_offset":280,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":422,"record_offset":5344,"parent_record_offset":5320,"ancestor_record_offset":280,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":422,"record_offset":5320,"parent_record_offset":5260,"ancestor_record_offset":280,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":422,"record_offset":5368,"parent_record_offset":5344,"ancestor_record_offset":280,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":422,"record_offset":5452,"parent_record_offset":5368,"ancestor_record_offset":280,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":422,"record_offset":5780,"parent_record_offset":5368,"ancestor_record_offset":280,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":422,"record_offset":1992,"parent_record_offset":1956,"ancestor_record_offset":280,"inlinee":8278,"source_line":425,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":422,"record_offset":8996,"parent_record_offset":8964,"ancestor_record_offset":280,"inlinee":8283,"source_line":601,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":422,"record_offset":9436,"parent_record_offset":9232,"ancestor_record_offset":280,"inlinee":6607,"source_line":602,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":422,"record_offset":2132,"parent_record_offset":2104,"ancestor_record_offset":280,"inlinee":8280,"source_line":92,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":422,"record_offset":2204,"parent_record_offset":2064,"ancestor_record_offset":280,"inlinee":6545,"source_line":429,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":422,"record_offset":2408,"parent_record_offset":2204,"ancestor_record_offset":280,"inlinee":6548,"source_line":591,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":422,"record_offset":2160,"parent_record_offset":2132,"ancestor_record_offset":280,"inlinee":5707,"source_line":58,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":422,"record_offset":2064,"parent_record_offset":2028,"ancestor_record_offset":280,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":422,"record_offset":2028,"parent_record_offset":1992,"ancestor_record_offset":280,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_elevation_supports/maintain_stone_column_colliders.rs
//     evidence={"module":422,"record_offset":7160,"parent_record_offset":7132,"ancestor_record_offset":280,"inlinee":26573,"source_line":62,"archive":"_pdb/C/modules/00422.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 45
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x135BC
// 首条位置: {"module":1455,"inlinee":79292}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1455,"record_offset":14644,"parent_record_offset":14436,"ancestor_record_offset":14436,"inlinee":9809,"source_line":994,"archive":"_pdb/C/modules/01455.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle3d::center
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":1455,"record_offset":14612,"parent_record_offset":14436,"ancestor_record_offset":14436,"inlinee":11331,"source_line":74,"archive":"_pdb/C/modules/01455.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle3d::height
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":1455,"record_offset":14516,"parent_record_offset":14436,"ancestor_record_offset":14436,"inlinee":11330,"source_line":79,"archive":"_pdb/C/modules/01455.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle3d::length
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":1455,"record_offset":14580,"parent_record_offset":14436,"ancestor_record_offset":14436,"inlinee":11329,"source_line":89,"archive":"_pdb/C/modules/01455.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle3d::width
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":1455,"record_offset":14548,"parent_record_offset":14436,"ancestor_record_offset":14436,"inlinee":11328,"source_line":84,"archive":"_pdb/C/modules/01455.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 58
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x135BB
// 首条位置: {"module":1455,"inlinee":79291}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 62
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x67CD
// 首条位置: {"module":422,"inlinee":26573}
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
// 完整结构及原始类型记录: maintain_stone_column_colliders.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: maintain_stone_column_colliders.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: maintain_stone_column_colliders.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 5; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// 候选参数类型: ["usize", "ref$<glam::f32::vec3::Vec3>"]
// [文件级接口候选，具体函数归属 unknown] country_core::resources::walls::arches::ArchSegment (*)(country_core::resources::walls::wall_path_segment::WallPathSegment)
// 候选参数类型: ["country_core::resources::walls::wall_path_segment::WallPathSegment"]
// [文件级接口候选，具体函数归属 unknown] country_core::resources::walls::arches::LintelSegment (*)(country_core::resources::walls::wall_path_segment::WallPathSegment)
// 候选参数类型: ["country_core::resources::walls::wall_path_segment::WallPathSegment"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec2::Vec2 (*)(glam::f32::vec3::Vec3)
// 候选参数类型: ["glam::f32::vec3::Vec3"]
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_stone_columns::StoneColumnCollisionStorage>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallDeleted>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::walls::wall_stone_columns::StoneColumnsChanged>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_stone_columns::StoneColumnCollisionStorage>", "bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>", "bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallDeleted>", "bevy_ecs::message::message_writer::MessageWriter<country_core::resources::walls::wall_stone_columns::StoneColumnsChanged>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_stone_columns::StoneColumnCollisionStorage>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallDeleted>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::walls::wall_stone_columns::StoneColumnsChanged>

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders
// 信息 ID: b657fb251da5a0f51275a61d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 0979ea3e21396a9b25043dd5: void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_stone_columns::StoneColumnCollisionStorage>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallDeleted>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::walls::wall_stone_columns::StoneColumnsChanged>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_stone_columns::StoneColumnCollisionStorage>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallDeleted>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::walls::wall_stone_columns::StoneColumnsChanged>),tuple$<>,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_stone_columns::StoneColumnCollisionStorage>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallDeleted>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::walls::wall_stone_columns::StoneColumnsChanged>)> > @ 0x2c5f37 -> 0x8398f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_stone_columns::StoneColumnCollisionStorage>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallDeleted>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::walls::wall_stone_columns::StoneColumnsChanged>),tuple$<>,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_stone_columns::StoneColumnCollisionStorage>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallDeleted>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::walls::wall_stone_columns::StoneColumnsChanged>)> @ 0x308989 -> 0x8398f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders::closure$0
// 信息 ID: f5240d1ee5cb1b41718e8815
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders", "captures": "unknown"}
// [闭包环境类型引用] system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders::closure_env$0 <- core::iter::adapters::flatten::impl$3::next<core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<country_core::resources::walls::wall_stone_columns::StoneColumn>,alloc::vec::Vec<utils::geometry::intersect::Triangle,alloc::alloc::Global>,system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders::closure_env$0>,array$<glam::f32::vec3::Vec3,3>,system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders::closure_env$1>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::vec3::Vec3,core::iter::adapters::flatten::FlatMap<core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<country_core::resources::walls::wall_stone_columns::StoneColumn>,alloc::vec::Vec<utils::geometry::intersect::Triangle,alloc::alloc::Global>,system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders::closure_env$0>,array$<glam::f32::vec3::Vec3,3>,system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders::closure_env$0 <- core::ptr::drop_in_place<core::iter::adapters::flatten::FlatMap<core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<country_core::resources::walls::wall_stone_columns::StoneColumn>,alloc::vec::Vec<utils::geometry::intersect::Triangle,alloc::alloc::Global>,system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders::closure_env$0>,array$<glam::f32::vec3::Vec3,3>,system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders::closure_env$1> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] core::iter::adapters::flatten::impl$3::next<core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<country_core::resources::walls::wall_stone_columns::StoneColumn>,alloc::vec::Vec<utils::geometry::intersect::Triangle,alloc::alloc::Global>,system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders::closure_env$0>,array$<glam::f32::vec3::Vec3,3>,system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders::closure_env$1>; module=1455, inline_record=14436; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders::closure$1
// 信息 ID: c3c4ba7fdb64a9809f498e4b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders", "captures": "unknown"}
// [闭包环境类型引用] system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders::closure_env$1 <- core::iter::adapters::flatten::impl$3::next<core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<country_core::resources::walls::wall_stone_columns::StoneColumn>,alloc::vec::Vec<utils::geometry::intersect::Triangle,alloc::alloc::Global>,system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders::closure_env$0>,array$<glam::f32::vec3::Vec3,3>,system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders::closure_env$1>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders::closure_env$1 <- alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::vec3::Vec3,core::iter::adapters::flatten::FlatMap<core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<country_core::resources::walls::wall_stone_columns::StoneColumn>,alloc::vec::Vec<utils::geometry::intersect::Triangle,alloc::alloc::Global>,system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders::closure_env$0>,array$<glam::f32::vec3::Vec3,3>,system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders::closure_env$1 <- core::ptr::drop_in_place<core::iter::adapters::flatten::FlatMap<core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<country_core::resources::walls::wall_stone_columns::StoneColumn>,alloc::vec::Vec<utils::geometry::intersect::Triangle,alloc::alloc::Global>,system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders::closure_env$0>,array$<glam::f32::vec3::Vec3,3>,system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders::closure_env$1> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] core::iter::adapters::flatten::impl$3::next<core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<country_core::resources::walls::wall_stone_columns::StoneColumn>,alloc::vec::Vec<utils::geometry::intersect::Triangle,alloc::alloc::Global>,system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders::closure_env$0>,array$<glam::f32::vec3::Vec3,3>,system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders::closure_env$1>; module=1455, inline_record=15396; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders::closure$2
// 信息 ID: b7af86bf0dfdf9935a4c6ce9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders; module=422, inline_record=7160; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
